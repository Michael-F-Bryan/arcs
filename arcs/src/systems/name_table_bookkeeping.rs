use crate::components::{Name, NameTable};
use specs::prelude::*;

/// A [`System`] which makes sure the global [`NameTable`] is kept up-to-date.
#[derive(Debug)]
pub struct NameTableBookkeeping {
    changes: ReaderId<ComponentEvent>,
    inserted: BitSet,
}

impl NameTableBookkeeping {
    pub const NAME: &'static str = module_path!();

    pub fn new(world: &World) -> Self {
        NameTableBookkeeping {
            changes: world.write_storage::<Name>().register_reader(),
            inserted: BitSet::new(),
        }
    }
}

impl<'world> System<'world> for NameTableBookkeeping {
    type SystemData = (
        Entities<'world>,
        ReadStorage<'world, Name>,
        Write<'world, NameTable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, names, mut name_table) = data;

        // clear any left-over data
        self.inserted.clear();

        // record which changes have happened since we last ran
        for event in names.channel().read(&mut self.changes) {
            match dbg!(event) {
                ComponentEvent::Inserted(id) => {
                    self.inserted.add(*id);
                },
                ComponentEvent::Removed(id) => {
                    name_table.remove_by_id(*id);
                },
                ComponentEvent::Modified(id) => {
                    name_table.remove_by_id(*id);
                    self.inserted.add(*id);
                },
            }
        }

        for (ent, name, _) in (&entities, &names, &self.inserted).join() {
            use std::collections::hash_map::Entry;

            match name_table.names.entry(name.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(ent);
                },
                Entry::Occupied(mut entry) => {
                    log::warn!(
                        "Duplicate name found when associating {:?} with \"{}\" (previous entity: {:?})",
                        ent,
                        name.as_ref(),
                        entry.get()
                    );
                    entry.insert(ent);
                },
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self::SystemData as shred::DynamicSystemData>::setup(
            &self.accessor(),
            world,
        );

        let entities = world.entities();
        let names = world.read_storage::<Name>();
        let mut name_table = world.write_resource::<NameTable>();

        name_table.clear();

        for (ent, name) in (&entities, &names).join() {
            name_table.names.insert(name.clone(), ent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_creates_all_outstanding_names() {
        let mut world = World::new();
        crate::components::register(&mut world);
        let first = world.create_entity().with(Name::new("first")).build();
        let second = world.create_entity().with(Name::new("second")).build();
        let mut system = NameTableBookkeeping::new(&world);

        System::setup(&mut system, &mut world);

        let names = world.read_resource::<NameTable>();
        assert_eq!(names.len(), 2);
        assert_eq!(names.get("first").unwrap(), first);
        assert_eq!(names.get("second").unwrap(), second);
    }

    #[test]
    fn run_will_keep_the_nametable_updated() {
        let mut world = World::new();
        crate::components::register(&mut world);
        let first = world.create_entity().with(Name::new("first")).build();
        let second = world.create_entity().with(Name::new("second")).build();
        let mut system = NameTableBookkeeping::new(&world);
        System::setup(&mut system, &mut world);

        // make some changes after the initial setup
        let third = world.create_entity().with(Name::new("third")).build();
        world.delete_entity(first).unwrap();
        world.maintain();

        // then run the system
        system.run_now(&world);

        let names = world.read_resource::<NameTable>();
        println!("{:?}", *names);
        assert_eq!(names.len(), 2);
        assert!(names.get("first").is_none());
        assert_eq!(names.get("second").unwrap(), second);
        assert_eq!(names.get("third").unwrap(), third);
    }
}

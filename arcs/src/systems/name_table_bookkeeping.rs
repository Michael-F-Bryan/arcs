use crate::components::{Name, NameTable};
use specs::prelude::*;

/// A [`System`] which makes sure the global [`NameTable`] is kept up-to-date.
#[derive(Debug)]
pub struct NameTableBookkeeping {
    changes: ReaderId<ComponentEvent>,
    inserted: BitSet,
    removed: BitSet,
}

impl NameTableBookkeeping {
    pub const NAME: &'static str =
        concat!(module_path!(), "::", stringify!(NameTableBookkeeping));

    pub fn new(world: &World) -> Self {
        NameTableBookkeeping {
            changes: world.write_storage::<Name>().register_reader(),
            inserted: BitSet::new(),
            removed: BitSet::new(),
        }
    }
}

impl<'world> System<'world> for NameTableBookkeeping {
    type SystemData = (
        Entities<'world>,
        ReadStorage<'world, Name>,
        WriteExpect<'world, NameTable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, names, mut name_table) = data;

        // clear any left-over data
        self.inserted.clear();
        self.removed.clear();

        // record which changes have happened since we last ran
        for event in names.channel().read(&mut self.changes) {
            match event {
                ComponentEvent::Inserted(id) => {
                    self.inserted.add(*id);
                },
                ComponentEvent::Removed(id) => {
                    self.removed.add(*id);
                },
                ComponentEvent::Modified(id) => {
                    self.removed.add(*id);
                    self.inserted.add(*id);
                },
            }
        }

        for (name, _) in (&names, &self.removed).join() {
            name_table.names.remove(name);
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
                        name.0,
                        entry.get()
                    );
                    entry.insert(ent);
                },
            }
        }
    }
}

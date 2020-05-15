use crate::{algorithms::Bounded, components::DrawingObject, BoundingBox, DrawingSpace};
use specs::prelude::*;

/// Lets us keep track of a [`DrawingObject`]'s rough location in *Drawing
/// Space*.
#[derive(Debug)]
pub struct SyncBounds {
    changes: ReaderId<ComponentEvent>,
    to_update: BitSet,
    removed: BitSet,
}

impl SyncBounds {
    pub const NAME: &'static str = module_path!();

    pub fn new(world: &World) -> SyncBounds {
        SyncBounds {
            changes: world.write_storage::<DrawingObject>().register_reader(),
            to_update: BitSet::new(),
            removed: BitSet::new(),
        }
    }
}

impl<'world> System<'world> for SyncBounds {
    type SystemData = (
        WriteStorage<'world, BoundingBox<DrawingSpace>>,
        ReadStorage<'world, DrawingObject>,
        Entities<'world>,
    );

    fn run(&mut self, data: Self::SystemData) {
        // clear any left-over flags
        self.to_update.clear();
        self.removed.clear();

        let (mut bounds, drawing_objects, entities) = data;

        // find out which items have changed since we were last polled
        for event in drawing_objects.channel().read(&mut self.changes) {
            match *event {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
                    self.to_update.add(id);
                },
                ComponentEvent::Removed(id) => {
                    self.removed.add(id);
                },
            }
        }

        for (ent, drawing_object, _) in
            (&entities, &drawing_objects, &self.to_update).join()
        {
            bounds
                .insert(ent, drawing_object.geometry.bounding_box())
                .unwrap();
        }

        for (ent, _) in (&entities, &self.removed).join() {
            bounds.remove(ent);
        }
    }
}

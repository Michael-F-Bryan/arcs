use crate::{
    components::{SpatialEntity, BoundingBox, DrawingObject},
    algorithms::{Bounded},
    Vector,
    primitives::Arc,
};
use specs::prelude::*;
use aabb_quadtree::{QuadTree, ItemId, Spatial};
use euclid::{TypedRect};
use std::collections::HashMap;

/// A [`System`] which keeps track of the spatial relation of entities
#[derive(Debug)]
pub struct SpatialRelation {
    changes: ReaderId<ComponentEvent>,
    to_insert: BitSet,
    to_update: BitSet,
    to_remove: BitSet,
    quadtree: QuadTree<SpatialEntity, f64, [(ItemId, TypedRect<f32, f64>); 0]>,
    ids: HashMap<specs::world::Index, ItemId>
}

impl SpatialRelation {
    pub const NAME: &'static str = module_path!();

    // FIXME: Hard-code is bad-bad
    pub const WORLD_RADIUS: f64 = 1_000_000.0;

    pub fn new(world: &World) -> Self {
        // Initialize quadtree
        let size = BoundingBox::new(
            Vector::new(-SpatialRelation::WORLD_RADIUS, -SpatialRelation::WORLD_RADIUS),
            Vector::new(SpatialRelation::WORLD_RADIUS, SpatialRelation::WORLD_RADIUS)
            ).aabb();
        let quadtree: QuadTree<SpatialEntity, f64, [(ItemId, TypedRect<f32, f64>); 0]> = QuadTree::new(
            size,
            true,
            4,
            16,
            8,
            4
        );  
        
        SpatialRelation {
            changes: world.write_storage::<DrawingObject>().register_reader(),
            to_insert: BitSet::new(),
            to_update: BitSet::new(),
            to_remove: BitSet::new(),
            quadtree,
            ids: HashMap::new()
        }
    }

    pub fn query_point(&self, pt: Vector) -> Option<Vec<Entity>> {
        let query_circle = Arc::from_centre_radius(pt, 1.0, 0.0, 2.0 * std::f64::consts::PI);
        let query = self.quadtree.query(query_circle.bounding_box().aabb());
        if query.is_empty() {Option::None}
        else {
            let entities: Vec<_> = query.iter().map(|q| q.0.entity).collect();
            Option::Some(entities)
        }
    }    
}

impl<'world> System<'world> for SpatialRelation {
    type SystemData = (
        WriteStorage<'world, BoundingBox>,
        ReadStorage<'world, DrawingObject>,
        Entities<'world>
    );

    fn run(&mut self, data: Self::SystemData) {
        // clear any left-over flags
        self.to_insert.clear();
        self.to_update.clear();
        self.to_remove.clear();

        let (mut bounds, drawing_objects, entities) = data;

        // find out which items have changed since we were last polled
        for event in drawing_objects.channel().read(&mut self.changes) {
            println!("Event is {:#?}", event);
            match *event {
                ComponentEvent::Inserted(id) => {
                    self.to_insert.add(id);
                },
                ComponentEvent::Modified(id) => {
                    self.to_update.add(id);
                },
                ComponentEvent::Removed(id) => {
                    self.to_remove.add(id);
                    if self.ids.contains_key(&id) {
                        // remove old entries
                        self.quadtree.remove(self.ids[&id]);
                        self.ids.remove(&id);
                    }
                },
            }
        }

        for (ent, drawing_object, _) in
            (&entities, &drawing_objects, &self.to_insert).join()
        {
            let bb = drawing_object.geometry.bounding_box();

            match self.quadtree.insert(SpatialEntity::new(bb, ent)) {
                Some(id) => {
                    // Store entity id for lookup in modify / delete operations
                    self.ids.insert(ent.id(), id);

                    bounds
                    .insert(ent, bb)
                    .unwrap();
                    },
                None => ()
            }

        }

        for (ent, drawing_object, _) in
            (&entities, &drawing_objects, &self.to_update).join()
        {
            // look for entity in quadtree
            let entity_id = ent.id();
            if self.ids.contains_key(&entity_id) {
                // remove old entries
                self.quadtree.remove(self.ids[&entity_id]);
                self.ids.remove(&entity_id);

                // add modified ones
                let bb = drawing_object.geometry.bounding_box();
                match self.quadtree.insert(SpatialEntity::new(bb, ent)) {
                    Some(id) => {
                        // Store entity id for lookup in modify / delete operations
                        self.ids.insert(ent.id(), id);
    
                        bounds
                        .insert(ent, bb)
                        .unwrap();
                        },
                    None => ()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        primitives::{Line},
        components::{register, Layer, Name, DrawingObject, Geometry, LineStyle, Dimension},
        Vector,
        systems::SpatialRelation,
    };
    use specs::prelude::*;
    use piet::Color;

    #[test]
    fn run_will_keep_spatial_updated() {
        let mut world = World::new();

        // make sure we register all components
        register(&mut world);

        // Setup our spatial system
        let mut system = SpatialRelation::new(&world);
        System::setup(&mut system, &mut world);
    
        let layer = Layer::create(
            world.create_entity(),
            Name::new("default"),
            Layer {
                z_level: 0,
                visible: true,
            },
        );
    
        // Add a line to our world
        let line = Line::new(Vector::new(2.0, 1.0), Vector::new(5.0, -1.0));
        let first = world
            .create_entity()
            .with(DrawingObject {
                geometry: Geometry::Line(line),
                layer,
            })
            .with(LineStyle {
                width: Dimension::DrawingUnits(5.0),
                stroke: Color::rgb8(0xff, 0, 0),
            })
            .build()
        ;
        
        // And add another
        let line = Line::new(Vector::new(3.0, 0.0), Vector::new(-1.0, 2.0));
        let _second = world
            .create_entity()
            .with(DrawingObject {
                geometry: Geometry::Line(line),
                layer,
            })
            .with(LineStyle {
                width: Dimension::DrawingUnits(5.0),
                stroke: Color::rgb8(0xff, 0, 0),
            })
            .build()
        ;

        // Test if the system works
        system.run_now(&world);
        let query = system.query_point(Vector::new(3.0, -0.5));
        assert!(query != None);
        assert_eq!(query.unwrap().len(), 1);

        let query = system.query_point(Vector::new(2.5, 0.5));
        assert!(query != None);
        assert_eq!(query.unwrap().len(), 2);

        // Test removing
        world.delete_entity(first).unwrap();
        world.maintain();

        system.run_now(&world);
        let query = system.query_point(Vector::new(3.0, -0.5));
        assert!(query == None);

        // Test modifying
        // TODO

    }
}
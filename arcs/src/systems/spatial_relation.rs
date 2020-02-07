use crate::{
    components::{SpatialEntity, Space, BoundingBox},
};
use specs::prelude::*;

/// A [`System`] which keeps track of the spatial relation of entities
#[derive(Debug)]
pub struct SpatialRelation {
    changes: ReaderId<ComponentEvent>,
    to_insert: BitSet,
    to_update: BitSet,
}

impl SpatialRelation {
    pub const NAME: &'static str = module_path!();

    pub fn new(world: &World) -> Self {        
        SpatialRelation {
            changes: world.write_storage::<BoundingBox>().register_reader(),
            to_insert: BitSet::new(),
            to_update: BitSet::new(),
        }
    }
}

impl<'world> System<'world> for SpatialRelation {
    type SystemData = (
        Write<'world, Space>,
        ReadStorage<'world, BoundingBox>,
        Entities<'world>
    );

    fn run(&mut self, data: Self::SystemData) {
        // clear any left-over flags
        self.to_insert.clear();
        self.to_update.clear();

        let (mut space, bounds, entities) = data;

        // find out which items have changed since we were last polled
        for event in bounds.channel().read(&mut self.changes) {
            match *event {
                ComponentEvent::Inserted(id) => {
                    self.to_insert.add(id);
                },
                ComponentEvent::Modified(id) => {
                    self.to_update.add(id);
                },
                ComponentEvent::Removed(id) => {
                    space.remove_by_id(id);
                },
            }
        }

        for (ent, bounding_box, _) in
            (&entities, &bounds, &self.to_insert).join()
        {
            space.modify(SpatialEntity::new(*bounding_box, ent));
        }

        for (ent, bounding_box, _) in
            (&entities, &bounds, &self.to_update).join()
        {
            space.modify(SpatialEntity::new(*bounding_box, ent));
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self::SystemData as shred::DynamicSystemData>::setup(
            &self.accessor(),
            world,
        );

        let bounding_storage = world.read_storage::<BoundingBox>();
        let mut space = world.write_resource::<Space>();

        space.clear();

        for (entity, bounding_box) in (&world.entities(), &bounding_storage).join() {
            space.modify(SpatialEntity::new(*bounding_box, entity));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::{register, Layer, Name, DrawingObject, Geometry, LineStyle, Dimension, Space},
        {Point, Line},
        systems::{SpatialRelation, SyncBounds},
        algorithms::{Bounded, Translate},
        Vector
    };
    use specs::prelude::*;
    use piet::Color;
    use euclid::Length;

    #[test]
    fn setup_creates_all_outstanding_spatial_entities() {
        let mut world = World::new();

        // make sure we register all components
        register(&mut world);
    
        let layer = Layer::create(
            world.create_entity(),
            Name::new("default"),
            Layer {
                z_level: 0,
                visible: true,
            },
        );
    
        // Add a line to our world
        let line = Line::new(Point::new(2.0, 1.0), Point::new(5.0, -1.0));
        let first = world
            .create_entity()
            .with(DrawingObject {
                geometry: Geometry::Line(line),
                layer,
            })
            .with(LineStyle {
                width: Dimension::DrawingUnits(Length::new(5.0)),
                stroke: Color::rgb8(0xff, 0, 0),
            })
            .with(line.bounding_box())
            .build()
        ;

        // Setup our spatial system
        let mut system = SpatialRelation::new(&world);
        System::setup(&mut system, &mut world);

        let space = world.read_resource::<Space>();
        assert_eq!(space.len(), 1);
        assert_eq!(space.iter().next().unwrap().entity, first);

    }

    #[test]
    fn run_will_keep_spatial_updated() {
        let mut world = World::new();

        // make sure we register all components
        register(&mut world);
    
        let layer = Layer::create(
            world.create_entity(),
            Name::new("default"),
            Layer {
                z_level: 0,
                visible: true,
            },
        );
    
        // Add a line to our world
        let line = Line::new(Point::new(2.0, 1.0), Point::new(5.0, -1.0));
        let first = world
            .create_entity()
            .with(DrawingObject {
                geometry: Geometry::Line(line),
                layer,
            })
            .with(LineStyle {
                width: Dimension::DrawingUnits(Length::new(5.0)),
                stroke: Color::rgb8(0xff, 0, 0),
            })
            .with(line.bounding_box())
            .build()
        ;

        // Setup our spatial system
        let mut system = SpatialRelation::new(&world);
        System::setup(&mut system, &mut world);
        
        // make some changes after the initial setup
        let line = Line::new(Point::new(3.0, 0.0), Point::new(-1.0, 2.0));
        let second = world
            .create_entity()
            .with(DrawingObject {
                geometry: Geometry::Line(line),
                layer,
            })
            .with(LineStyle {
                width: Dimension::DrawingUnits(Length::new(5.0)),
                stroke: Color::rgb8(0xff, 0, 0),
            })
            .with(line.bounding_box())
            .build()
        ;

        // Test if the system works
        system.run_now(&world);

        // query which is inside the bounding_box of first
        let query: Vec<_> = world.read_resource::<Space>()
            .query_point(Point::new(3.0, -0.5), 1.0)
            .collect();
        assert!(!query.is_empty());
        assert_eq!(query.len(), 1);
        assert_eq!(query[0].entity, first);

        // query which is inside bounding_box of both first and second
        let query: Vec<_> = world.read_resource::<Space>()
            .query_point(Point::new(2.5, 0.5), 1.0)
            .collect();
        assert!(!query.is_empty());
        assert_eq!(query.len(), 2);
        assert!((query[0].entity == first && query[1].entity == second) |
                (query[0].entity == second && query[1].entity == first)
        );
    }

    #[test]
    fn spatial_will_update_on_modified() {
        let mut world = World::new();

        // make sure we register all components
        register(&mut world);
    
        let layer = Layer::create(
            world.create_entity(),
            Name::new("default"),
            Layer {
                z_level: 0,
                visible: true,
            },
        );
    
        // Add a line to our world
        let line = Line::new(Point::new(2.0, 1.0), Point::new(5.0, -1.0));
        let first = world
            .create_entity()
            .with(DrawingObject {
                geometry: Geometry::Line(line),
                layer,
            })
            .with(LineStyle {
                width: Dimension::DrawingUnits(Length::new(5.0)),
                stroke: Color::rgb8(0xff, 0, 0),
            })
            .with(line.bounding_box())
            .build()
        ;

        // Setup our spatial system and our syncbounds system
        let mut spatial_system = SpatialRelation::new(&world);
        System::setup(&mut spatial_system, &mut world);
        let mut syncbounds_system = SyncBounds::new(&world);
        System::setup(&mut syncbounds_system, &mut world);

        // Do first query before modification to test everything works as expected
        let query: Vec<_> = world.read_resource::<Space>()
            .query_point(Point::new(3.0, -0.5), 1.0)
            .collect();
        assert!(!query.is_empty());
        assert_eq!(query.len(), 1);
        assert_eq!(query[0].entity, first);

        // Modify geometry of our drawing_object
        let mut temp_line = Line::new(Point::new(3.0, 0.0), Point::new(-1.0, 2.0));
        temp_line.translate(Vector::new(100.0, 0.0));
        world.write_storage::<DrawingObject>()
            .get_mut(first)
            .unwrap()
            .geometry = Geometry::Line(temp_line);

        // run both systems
        syncbounds_system.run_now(&world);
        spatial_system.run_now(&world);

        // do the same query again, this time we expect no results
        let query: Vec<_> = world.read_resource::<Space>()
            .query_point(Point::new(3.0, -0.5), 1.0)
            .collect();
        assert!(query.is_empty());
    }

    #[test]
    fn spatial_will_update_on_removed() {
        let mut world = World::new();

        // make sure we register all components
        register(&mut world);
    
        let layer = Layer::create(
            world.create_entity(),
            Name::new("default"),
            Layer {
                z_level: 0,
                visible: true,
            },
        );
    
        // Add a line to our world
        let line = Line::new(Point::new(2.0, 1.0), Point::new(5.0, -1.0));
        let first = world
            .create_entity()
            .with(DrawingObject {
                geometry: Geometry::Line(line),
                layer,
            })
            .with(LineStyle {
                width: Dimension::DrawingUnits(Length::new(5.0)),
                stroke: Color::rgb8(0xff, 0, 0),
            })
            .with(line.bounding_box())
            .build()
        ;

        // Setup our spatial system and our syncbounds system
        let mut spatial_system = SpatialRelation::new(&world);
        System::setup(&mut spatial_system, &mut world);
        let mut syncbounds_system = SyncBounds::new(&world);
        System::setup(&mut syncbounds_system, &mut world);

        // Do first query before modification to test everything works as expected
        let query: Vec<_> = world.read_resource::<Space>()
            .query_point(Point::new(3.0, -0.5), 1.0)
            .collect();
        assert!(!query.is_empty());
        assert_eq!(query.len(), 1);
        assert_eq!(query[0].entity, first);

        // remove `first`
        world.delete_entity(first).unwrap();
        world.maintain();

        // run both systems
        syncbounds_system.run_now(&world);
        spatial_system.run_now(&world);

        // do the same query again, this time we expect no results
        let query: Vec<_> = world.read_resource::<Space>()
            .query_point(Point::new(3.0, -0.5), 1.0)
            .collect();
        assert!(query.is_empty());
    }
}

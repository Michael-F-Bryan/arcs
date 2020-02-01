use crate::{
    components::{SpatialEntity, DrawingObject, Space},
    algorithms::{Bounded},
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
            changes: world.write_storage::<DrawingObject>().register_reader(),
            to_insert: BitSet::new(),
            to_update: BitSet::new(),
        }
    }
}

impl<'world> System<'world> for SpatialRelation {
    type SystemData = (
        Write<'world, Space>,
        ReadStorage<'world, DrawingObject>,
        Entities<'world>
    );

    fn run(&mut self, data: Self::SystemData) {
        // clear any left-over flags
        self.to_insert.clear();
        self.to_update.clear();

        let (mut space, drawing_objects, entities) = data;

        // find out which items have changed since we were last polled
        for event in drawing_objects.channel().read(&mut self.changes) {
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

        for (ent, drawing_object, _) in
            (&entities, &drawing_objects, &self.to_insert).join()
        {
            let bb = drawing_object.geometry.bounding_box();
            space.insert(SpatialEntity::new(bb, ent));
        }

        for (ent, drawing_object, _) in
            (&entities, &drawing_objects, &self.to_update).join()
        {
            let bb = drawing_object.geometry.bounding_box();
            space.modify(SpatialEntity::new(bb, ent));
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self::SystemData as shred::DynamicSystemData>::setup(
            &self.accessor(),
            world,
        );

        let drawing_storage = world.read_storage::<DrawingObject>();
        let mut space = world.write_resource::<Space>();

        space.clear();

        for entity in world.entities().join() {
            if let Some(drawing) = drawing_storage.get(entity) {
                space.insert(SpatialEntity::new(drawing.geometry.bounding_box(), entity));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::{register, Layer, Name, DrawingObject, Geometry, LineStyle, Dimension, Space},
        {Point, Line},
        systems::SpatialRelation,
        algorithms::{Bounded},
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
        let _first = world
            .create_entity()
            .with(DrawingObject {
                geometry: Geometry::Line(line),
                layer,
            })
            .with(LineStyle {
                width: Dimension::DrawingUnits(Length::new(5.0)),
                stroke: Color::rgb8(0xff, 0, 0),
            })
            .build()
        ;

        // Setup our spatial system
        let mut system = SpatialRelation::new(&world);
        System::setup(&mut system, &mut world);
        
        // make some changes after the initial setup
        let line = Line::new(Point::new(3.0, 0.0), Point::new(-1.0, 2.0));
        let _second = world
            .create_entity()
            .with(DrawingObject {
                geometry: Geometry::Line(line),
                layer,
            })
            .with(LineStyle {
                width: Dimension::DrawingUnits(Length::new(5.0)),
                stroke: Color::rgb8(0xff, 0, 0),
            })
            .build()
        ;

        // Test if the system works
        system.run_now(&world);
        let space = world.read_resource::<Space>();

        let mut query = space.query_point(Point::new(3.0, -0.5), 1.0);
        match query.next() {
            None => assert!(false),
            Some(spatial_entity) => {
                let draw = world.read_storage::<DrawingObject>();
                assert_eq!(spatial_entity.bounds, draw.get(spatial_entity.entity).unwrap().geometry.bounding_box())
            }
        }

        // let query = space.query_point(Point::new(2.5, 0.5), 1.0);
        // assert!(query != None);
        // assert_eq!(query.unwrap().len(), 2);
    }
}
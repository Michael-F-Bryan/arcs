use crate::{
    components::BoundingBox,
    Vector,
};
use specs::{Entity, world::Index};
use aabb_quadtree::{QuadTree, Spatial, ItemId};
use euclid::{TypedRect, TypedPoint2D, TypedSize2D};
use std::collections::HashMap;

/// A intermediate struct that maps an [`Entity`] to its [`BoundingBox`]
/// 
/// This is used to populate an efficient spatial lookup structure like a `QuadTree`
#[derive(Debug)]
pub struct SpatialEntity {
    pub bounds: BoundingBox,
    pub entity: Entity
}

impl Spatial<f64> for SpatialEntity {
    fn aabb(&self) -> TypedRect<f32, f64> {
        let bb = self.bounds;
        TypedRect::<f32, f64>::new(
            // TypedRects have their origin at the bottom left corner (this is undocumented!)
            TypedPoint2D::new(bb.bottom_left().x as f32, bb.bottom_left().y as f32),
            TypedSize2D::new(bb.width() as f32, bb.height() as f32))
    }
}

impl SpatialEntity {
    pub fn new(bounds: BoundingBox, entity: Entity) -> SpatialEntity {
        SpatialEntity {
            bounds,
            entity
        }
    }
}

/// A global [`Resource`] for looking up which [`Entity`]s inhabit
/// a given spatial point or region
#[derive(Debug)]
pub struct Space {
    quadtree: QuadTree<SpatialEntity, f64, [(ItemId, TypedRect<f32, f64>); 0]>,
    ids: HashMap<Index, ItemId>
}

impl Default for Space {
    fn default() -> Self {
        // Initialize quadtree
        let size = BoundingBox::new(
            Vector::new(-Self::WORLD_RADIUS, -Self::WORLD_RADIUS),
            Vector::new(Self::WORLD_RADIUS, Self::WORLD_RADIUS)
            ).aabb();
        let quadtree: QuadTree<SpatialEntity, f64, [(ItemId, TypedRect<f32, f64>); 0]> = QuadTree::new(
            size,
            true,
            4,
            16,
            8,
            4
        );  
        Space {
            quadtree,
            ids: HashMap::new()
        }
    }
}

impl Space {
    // FIXME: Hard-code is bad-bad
    const WORLD_RADIUS: f64 = 1_000_000.0;

    pub fn insert(&mut self, spatial: SpatialEntity) {
        unimplemented!();
    }

    pub fn modify(&mut self, spatial: SpatialEntity) {
        unimplemented!();
    }

    pub fn remove_by_id(&mut self, id: Index) {
        unimplemented!();
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn query_point(&self, point: Vector) -> Option<Vec<Entity>> {
        unimplemented!();
    }

    pub fn query_region(&self, region: BoundingBox) -> Option<Vec<Entity>> {
        unimplemented!();
    }

    pub fn clear(&mut self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        primitives::{Line, Arc},
        components::{BoundingBox, SpatialEntity, register, Layer, Name, DrawingObject, Geometry, LineStyle, Dimension},
        Vector,
        algorithms::{Bounded},
    };
    use aabb_quadtree::{QuadTree, Spatial, ItemId};
    use euclid::{TypedRect};
    use std::f64::consts::PI;
    use specs::prelude::*;
    use piet::Color;

    #[test]
    fn line_insertion() {
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
        let line = Line::new(Vector::new(2.0, 1.0), Vector::new(5.0, -1.0));
        world
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

        // Create the Quadtree to hold all spacial entities
        // Size needs to be bigger then everything we plan on drawing :/
        let max_size = 1_000_000.0;
        let size = BoundingBox::new(Vector::new(-max_size, -max_size), Vector::new(max_size, max_size)).aabb();
        // I don't really understand why we need to specify A -> [(ItemId, TypedRect<f32, u64>); 0]
        let mut qt: QuadTree<SpatialEntity, f64, [(ItemId, TypedRect<f32, f64>); 0]> = QuadTree::new(
            size,
            true,
            4,
            16,
            8,
            4
        );        

        // get the lines entity from the world
        let mut id: Option<ItemId> = None;
        let drawing_storage = world.read_storage::<DrawingObject>();
        for entity in world.entities().join() {
            if let Some(drawing) = drawing_storage.get(entity) {
                match drawing.geometry {
                    Geometry::Line(l) => {
                        // Add to quad tree
                        id = qt.insert(SpatialEntity::new(l.bounding_box(), entity));
                        assert_eq!(qt.get(id.unwrap()).unwrap().entity, entity);
                    },
                    _ => (),
                }
            }
        }
        
        // simulate cursor query at line start
        let cursor_pos = Vector::new(2.0, 1.0);
        let cursor_circle = Arc::from_centre_radius(cursor_pos, 1.0, 0.0, 2.0 * PI);

        let query = qt.query(cursor_circle.bounding_box().aabb());
        assert_eq!(query.len(), 1);
        assert_eq!(query[0].2, id.unwrap());

        // simulate cursor query at line end
        // FIXME: Fails if we input exactly line end point
        let cursor_pos = Vector::new(4.9, -0.9);
        let cursor_circle = Arc::from_centre_radius(cursor_pos, 1.0, 0.0, 2.0 * PI);

        let query = qt.query(cursor_circle.bounding_box().aabb());
        assert_eq!(query.len(), 1);
        assert_eq!(query[0].2, id.unwrap());

    }
}
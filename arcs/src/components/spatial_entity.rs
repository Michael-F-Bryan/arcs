use crate::{
    components::BoundingBox,
    {Point, Arc},
    algorithms::{Bounded},
};
use specs::{Entity, world::Index};
use aabb_quadtree::{QuadTree, Spatial, ItemId};
use quadtree_euclid::{TypedRect, TypedPoint2D, TypedSize2D};
use std::collections::HashMap;
use std::iter;
use euclid::Angle;

pub(crate) type SpatialTree = QuadTree<SpatialEntity, f64, [(ItemId, TypedRect<f32, f64>); 0]>;

/// A intermediate struct that maps an [`Entity`] to its [`BoundingBox`]
/// 
/// This is used to populate an efficient spatial lookup structure like a `QuadTree`
#[derive(Debug, Copy, Clone)]
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
            TypedSize2D::new(bb.width().0 as f32, bb.height().0 as f32))
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
    quadtree: SpatialTree,
    ids: HashMap<Index, ItemId>
}

impl Default for Space {
    fn default() -> Self {
        Space {
            quadtree: Self::default_tree(),
            ids: HashMap::new()
        }
    }
}

impl Space {
    // FIXME: Hard-code is bad-bad
    const WORLD_RADIUS: f64 = 1_000_000.0;

    fn default_tree() -> SpatialTree{
        // Initialize quadtree
        let size = BoundingBox::new(
            Point::new(-Self::WORLD_RADIUS, -Self::WORLD_RADIUS),
            Point::new(Self::WORLD_RADIUS, Self::WORLD_RADIUS)
            ).aabb();
        let quadtree: SpatialTree = QuadTree::new(
            size,
            true,
            4,
            16,
            8,
            4
        );

        quadtree
    }

    pub fn insert(&mut self, spatial: SpatialEntity) {
        let entity_id = spatial.entity.id();
        if let Some(id) = self.quadtree.insert(spatial) {
            self.ids.insert(entity_id, id);
        }
    }

    pub fn modify(&mut self, spatial: SpatialEntity) {
        let entity_id = spatial.entity.id();
        if self.ids.contains_key(&entity_id) {
            let item_id = self.ids[&entity_id];

            // remove old item
            self.quadtree.remove(item_id);
            self.ids.remove(&entity_id);

            // Add modified
            self.insert(spatial);
        }
    }

    pub fn remove_by_id(&mut self, id: Index) {
        if self.ids.contains_key(&id) {
            let item_id = self.ids[&id];

            // remove old item
            self.quadtree.remove(item_id);
            self.ids.remove(&id);
        }
    }

    pub fn iter<'this>(
        &'this self,
    ) -> impl Iterator<Item = SpatialEntity> + 'this {
        self.quadtree.iter().map(|(_, (ent, _))| *ent)
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    // FIXME: radius in CanvasSpace in method signature
    pub fn query_point<'this>(
        &'this self, point: Point, radius: f64
    ) -> impl Iterator<Item = SpatialEntity> + 'this {
        let cursor_circle = Arc::from_centre_radius(
            point,
            radius,
            Angle::radians(0.0),
            Angle::radians(2.0 * std::f64::consts::PI)
        );
        self.query_region(cursor_circle.bounding_box())
    }

    pub fn query_region<'this>(
        &'this self, region: BoundingBox
    ) -> impl Iterator<Item = SpatialEntity> + 'this {
        self.quadtree.query(region.aabb()).into_iter().map(|q| *q.0)
    }

    pub fn clear(&mut self) {
        self.quadtree = Self::default_tree();
        self.ids.clear();
    }
}

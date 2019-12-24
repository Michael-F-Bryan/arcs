use crate::{
    algorithms::BoundingBox,
    components::{DrawingObject, Geometry, Layer, LineStyle, PointStyle},
    primitives::Point,
    render::Viewport,
    Vector,
};
use kurbo::{Circle, Rect};
use piet::{Error, RenderContext};
use shred_derive::SystemData;
use specs::prelude::*;
use std::{cmp::Reverse, collections::BTreeMap};

/// Long-lived state used when rendering.
#[derive(Debug, Clone)]
pub struct Renderer {
    viewport: Viewport,
}

impl Renderer {
    pub fn new(viewport: Viewport) -> Self {
        Renderer { viewport }
    }

    pub fn viewport(&self) -> &Viewport { &self.viewport }

    pub fn viewport_mut(&mut self) -> &mut Viewport { &mut self.viewport }

    /// Get a [`System`] which will render using a particular [`RenderContext`].
    pub fn system<'a, R>(
        &'a mut self,
        backend: R,
        window_size: Rect,
    ) -> impl System<'a> + 'a
    where
        R: RenderContext + 'a,
    {
        RenderSystem {
            backend,
            window_size,
            renderer: self,
        }
    }
}

/// The [`System`] which actually renders things. This needs to be a temporary
/// object "closing over" the [`Renderer`] and some [`RenderContext`] due to
/// lifetimes.
///
/// The `RenderContext` for the `piet_web` crate takes the HTML5 canvas by
/// `&mut` reference instead of owning it, and we don't want to tie our
/// [`Renderer`] to a particular stack frame because it's so long lived (we'd
/// end up fighting the borrow checker and have self-referential types).
#[derive(Debug)]
struct RenderSystem<'renderer, B> {
    backend: B,
    window_size: Rect,
    renderer: &'renderer mut Renderer,
}

impl<'world, 'renderer, B> RenderSystem<'renderer, B> {
    /// Calculate the area of the drawing displayed by the viewport.
    fn viewport_dimensions(&self) -> BoundingBox {
        let scale = self.renderer.viewport.pixels_per_drawing_unit;
        let width = scale * self.window_size.width();
        let height = scale * self.window_size.height();

        BoundingBox::from_centre_and_dimensions(
            self.renderer.viewport.centre,
            width,
            height,
        )
    }
}

impl<'world, 'renderer, B: RenderContext> RenderSystem<'renderer, B> {
    fn render(
        &mut self,
        ent: Entity,
        drawing_object: &DrawingObject,
        styles: &Styling,
    ) {
        match drawing_object.geometry {
            Geometry::Point(ref point) => {
                self.render_point(ent, point, drawing_object.layer, styles)
            },
            _ => unimplemented!(),
        }
    }

    /// Draw a [`Point`] as a circle on the canvas.
    fn render_point(
        &mut self,
        entity: Entity,
        point: &Point,
        layer: Entity,
        styles: &Styling,
    ) {
        let fallback = PointStyle::default();
        let style = styles
            .point_styles
            .get(entity)
            .or_else(|| styles.point_styles.get(layer))
            .unwrap_or(&fallback);

        let radius = style
            .radius
            .in_pixels(self.renderer.viewport.pixels_per_drawing_unit);
        let point = Circle {
            center: self.to_viewport_coordinates(point.location),
            radius,
        };

        self.backend.fill(point, &style.colour);
    }

    /// Translates a [`Vector`] from drawing space to a [`kurbo::Point`] on the
    /// canvas.
    fn to_viewport_coordinates(&self, point: Vector) -> kurbo::Point {
        unimplemented!()
    }
}

impl<'world, 'renderer, B: RenderContext> System<'world>
    for RenderSystem<'renderer, B>
{
    type SystemData = (DrawOrder<'world>, Styling<'world>);

    fn run(&mut self, data: Self::SystemData) {
        let (draw_order, styling) = data;

        let viewport_dimensions = self.viewport_dimensions();

        for (ent, obj) in draw_order.calculate(viewport_dimensions) {
            self.render(ent, obj, &styling);
        }
    }
}

/// Styling information.
#[derive(SystemData)]
struct Styling<'world> {
    line_styles: ReadStorage<'world, LineStyle>,
    point_styles: ReadStorage<'world, PointStyle>,
}

/// The state needed when calculating which order to draw things in so z-levels
/// are implemented correctly.
#[derive(SystemData)]
struct DrawOrder<'world> {
    entities: Entities<'world>,
    drawing_objects: ReadStorage<'world, DrawingObject>,
    layers: ReadStorage<'world, Layer>,
}

impl<'world> DrawOrder<'world> {
    fn calculate(
        &self,
        viewport_dimensions: BoundingBox,
    ) -> impl Iterator<Item = (Entity, &'_ DrawingObject)> + '_ {
        // Iterate through all drawing objects, grouping them by the parent
        // layer's z-level in reverse order (we want to yield higher z-levels
        // first)
        let mut drawing_objects: BTreeMap<
            Reverse<usize>,
            Vec<(Entity, &DrawingObject)>,
        > = BTreeMap::new();

        // PERF: This could be improved with a cache that maps entities layers,
        // letting us ignore hidden layers entirely

        for (ent, obj) in (&self.entities, &self.drawing_objects).join() {
            let Layer { z_level, visible } =
                self.layers.get(obj.layer).unwrap();

            if *visible {
                drawing_objects
                    .entry(Reverse(*z_level))
                    .or_default()
                    .push((ent, obj));
            }
        }

        drawing_objects.into_iter().flat_map(|(_, items)| items)
    }
}

use crate::{
    algorithms::Bounded,
    components::{
        BoundingBox, Dimension, DrawingObject, Geometry, Layer, LineStyle,
        PointStyle,
    },
    primitives::{Line, Point},
    render::Viewport,
    Vector,
};
use kurbo::{Circle, Size};
use piet::{Color, RenderContext};
use shred_derive::SystemData;
use specs::{join::MaybeJoin, prelude::*};
use std::{cmp::Reverse, collections::BTreeMap};

/// Long-lived state used when rendering.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Renderer {
    pub viewport: Viewport,
    pub background: Color,
}

impl Renderer {
    pub fn new(viewport: Viewport, background: Color) -> Self {
        Renderer {
            viewport,
            background,
        }
    }

    /// Get a [`System`] which will render using a particular [`RenderContext`].
    pub fn system<'a, R>(
        &'a self,
        backend: R,
        window_size: Size,
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

/// The [`System`] which actually renders things.
///
/// This needs to be a temporary object "closing over" the [`Renderer`] and some
/// [`RenderContext`] due to lifetimes.
///
/// In particular, the `RenderContext` for the `piet_web` crate takes the HTML5
/// canvas by `&mut` reference instead of owning it, and we don't want to tie
/// our [`Renderer`] to a particular stack frame because it's so long lived
/// (we'd end up fighting the borrow checker and have self-referential types).
#[derive(Debug)]
struct RenderSystem<'renderer, B> {
    backend: B,
    window_size: Size,
    renderer: &'renderer Renderer,
}

impl<'world, 'renderer, B> RenderSystem<'renderer, B> {
    /// Calculate the area of the drawing displayed by the viewport.
    fn viewport_dimensions(&self) -> BoundingBox {
        let scale = self.renderer.viewport.pixels_per_drawing_unit;
        let width = scale * self.window_size.width;
        let height = scale * self.window_size.height;

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
                self.render_point(ent, point, drawing_object.layer, styles);
            },
            Geometry::Line(ref line) => {
                self.render_line(ent, line, drawing_object.layer, styles);
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
        let style = styles.resolve_point_style(entity, layer);

        let shape = Circle {
            center: self.to_viewport_coordinates(point.location),
            radius: style
                .radius
                .in_pixels(self.renderer.viewport.pixels_per_drawing_unit),
        };
        log::trace!("Drawing {:?} as {:?} using {:?}", point, shape, style);

        self.backend.fill(shape, &style.colour);
    }

    fn render_line(
        &mut self,
        entity: Entity,
        line: &Line,
        layer: Entity,
        styles: &Styling,
    ) {
        let style = styles.resolve_line_style(entity, layer);

        let start = self.to_viewport_coordinates(line.start);
        let end = self.to_viewport_coordinates(line.end);
        let shape = kurbo::Line::new(start, end);
        let stroke_width = style
            .width
            .in_pixels(self.renderer.viewport.pixels_per_drawing_unit);
        log::trace!("Drawing {:?} as {:?} using {:?}", line, shape, style);

        self.backend
            .stroke(shape, &style.stroke, dbg!(stroke_width));
    }

    /// Translates a [`Vector`] from drawing space to a [`kurbo::Point`] on the
    /// canvas.
    fn to_viewport_coordinates(&self, point: Vector) -> kurbo::Point {
        super::to_canvas_coordinates(
            point,
            &self.renderer.viewport,
            self.window_size,
        )
    }
}

impl<'world, 'renderer, B: RenderContext> System<'world>
    for RenderSystem<'renderer, B>
{
    type SystemData = (DrawOrder<'world>, Styling<'world>);

    fn run(&mut self, data: Self::SystemData) {
        // make sure we're working with a blank screen
        self.backend.clear(self.renderer.background.clone());

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
    point_styles: ReadStorage<'world, PointStyle>,
    line_styles: ReadStorage<'world, LineStyle>,
}

impl<'world> Styling<'world> {
    const DEFAULT_LINE_STYLE: LineStyle = LineStyle {
        width: Dimension::Pixels(1.0),
        stroke: Color::BLACK,
    };
    const DEFAULT_POINT_STYLE: PointStyle = PointStyle {
        radius: Dimension::Pixels(1.0),
        colour: Color::BLACK,
    };

    fn resolve_point_style(&self, point: Entity, layer: Entity) -> &PointStyle {
        self
            .point_styles
            // the style for this point may have been overridden explicitly
            .get(point)
            // otherwise fall back to the layer's PointStyle
            .or_else(|| self.point_styles.get(layer))
            // fall back to the global default if the layer didn't specify one
            .unwrap_or(&Self::DEFAULT_POINT_STYLE)
    }

    fn resolve_line_style(&self, line: Entity, layer: Entity) -> &LineStyle {
        self.line_styles
            .get(line)
            .or_else(|| self.line_styles.get(layer))
            .unwrap_or(&Self::DEFAULT_LINE_STYLE)
    }
}

/// The state needed when calculating which order to draw things in so z-levels
/// are implemented correctly.
#[derive(SystemData)]
struct DrawOrder<'world> {
    entities: Entities<'world>,
    drawing_objects: ReadStorage<'world, DrawingObject>,
    layers: ReadStorage<'world, Layer>,
    bounding_boxes: ReadStorage<'world, BoundingBox>,
}

impl<'world> DrawOrder<'world> {
    fn calculate(
        &self,
        viewport_dimensions: BoundingBox,
    ) -> impl Iterator<Item = (Entity, &'_ DrawingObject)> + '_ {
        type EntitiesByZLevel<'a> =
            BTreeMap<Reverse<usize>, Vec<(Entity, &'a DrawingObject)>>;

        // Iterate through all drawing objects, grouping them by the parent
        // layer's z-level in reverse order (we want to yield higher z-levels
        // first)
        let mut drawing_objects = EntitiesByZLevel::new();

        // PERF: This function has a massive impact on render times
        // Some ideas:
        //   - Use a pre-calculated quad-tree so we just need to check items
        //     within the viewport bounds
        //   - use a entities-to-layers cache so we can skip checking whether to
        //     draw an object on a hidden layer

        for (ent, obj, bounds) in (
            &self.entities,
            &self.drawing_objects,
            MaybeJoin(&self.bounding_boxes),
        )
            .join()
        {
            let Layer { z_level, visible } = self
                .layers
                .get(obj.layer)
                .expect("The object's layer was deleted");

            // try to use the cached bounds, otherwise re-calculate them
            let bounds = bounds
                .copied()
                .unwrap_or_else(|| obj.geometry.bounding_box());

            if *visible && viewport_dimensions.intersects_with(bounds) {
                drawing_objects
                    .entry(Reverse(*z_level))
                    .or_default()
                    .push((ent, obj));
            }
        }

        drawing_objects.into_iter().flat_map(|(_, items)| items)
    }
}

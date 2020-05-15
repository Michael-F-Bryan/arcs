use arcs::{
    components::{
        Dimension, DrawingObject, Geometry, Layer, LineStyle, Name, PointStyle,
        Viewport,
    },
    window::Window,
    Length, Line, Point,
};
use euclid::{Scale, Size2D};
use image::RgbaImage;
use piet::{Color, ImageFormat};
use specs::prelude::*;
use std::f64::consts::PI;

fn main() {
    env_logger::init();

    // Create a world and add some items to it
    let mut world = World::new();

    // make sure we register all components
    arcs::components::register(&mut world);

    let layer = Layer::create(
        world.create_entity(),
        Name::new("default"),
        Layer {
            z_level: 0,
            visible: true,
        },
    );

    // add a green dot to the world
    world
        .create_entity()
        .with(DrawingObject {
            geometry: Geometry::Point(Point::new(20.0, 0.0)),
            layer,
        })
        .with(PointStyle {
            radius: Dimension::Pixels(50.0),
            colour: Color::rgb8(0, 0xff, 0),
        })
        .build();
    // and a red hexagon
    let angles = (0..7).map(|i| i as f64 * 2.0 * PI / 6.0);
    let radius = 50.0;
    for (start_angle, end_angle) in angles.clone().zip(angles.clone().skip(1)) {
        let start =
            Point::new(radius * start_angle.cos(), radius * start_angle.sin());
        let end =
            Point::new(radius * end_angle.cos(), radius * end_angle.sin());

        world
            .create_entity()
            .with(DrawingObject {
                geometry: Geometry::Line(Line::new(start, end)),
                layer,
            })
            .with(LineStyle {
                width: Dimension::DrawingUnits(Length::new(5.0)),
                stroke: Color::rgb8(0xff, 0, 0),
            })
            .build();
    }

    // now we've added some objects to the world we can start rendering
    let window = Window::create(&mut world);

    // set the viewport and background colour
    *window.viewport_mut(&mut world.write_storage()) = Viewport {
        centre: Point::zero(),
        pixels_per_drawing_unit: Scale::new(5.0),
    };
    window
        .style_mut(&mut world.write_storage())
        .background_colour = Color::WHITE;

    // We'll need a canvas to draw things on
    let width = 640;
    let height = 480;
    let mut device = piet_common::Device::new().unwrap();
    let mut bitmap_canvas = device.bitmap_target(width, height, 1.0).unwrap();

    {
        // now we've got a piet::RenderContext we can create the rendering
        // system
        let mut system = window.render_system(
            bitmap_canvas.render_context(),
            Size2D::new(width as f64, height as f64),
        );
        // and run the system
        RunNow::run_now(&mut system, &world);
    }

    let raw_image = bitmap_canvas
        .into_raw_pixels(ImageFormat::RgbaPremul)
        .unwrap();
    RgbaImage::from_raw(width as u32, height as u32, raw_image)
        .unwrap()
        .save("rendered.png")
        .unwrap();
}

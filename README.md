# A Rust CAD System

[![Build Status](https://travis-ci.com/Michael-F-Bryan/arcs.svg?branch=master)](https://travis-ci.com/Michael-F-Bryan/arcs)

([API Docs])

An extensible framework for creating 2D CAD applications, written in Rust based
on an *Entity-Component-System* architecture.

## Project Goals and Milestones

I've broken the direction of this project up into a handful of milestones, each
containing a list of related features or concepts.

- [X] Milestone: MVP
  - [X] Geometry primitives (`Arc`, `Point`, `Line`, etc.)
  - [X] Basic styling component (e.g. `LineStyle` with a colour and stroke width)
  - [X] All drawing objects are attached to a `Layer`
  - [X] Drawing objects have a `BoundingBox` which gets recalculated whenever
        something changes
  - [X] Render drawing objects on a canvas
  - [X] Example showing all of the above and rendering a simple drawing
        ([`render_to_image.rs`](arcs/examples/render_to_image.rs))

- [ ] Milestone: Online demo
  - [ ] Render correctly to a HTML5 `<canvas>`
  - Interactive tools for creating drawing objects:
    - [ ] Point
    - [ ] Line
    - [ ] Arc
  - [ ] Move items between layers
  - [ ] Selectively show/hide layers
  - [ ] `Viewport` panning
  - [ ] Zoom `Viewport` in/out

- [ ] Milestone: Required by Real-World Applications
      (see [`Michael-F-Bryan/rustmatic#38][rustmatic-38])
  - [ ] Robust undo/redo mechanism

- <span id="wishlist">Wish List</span>
  - [X] Z-levels so objects can be drawn on top of each other
  - [X] Entities can be tagged with a name to give them semantic meaning
  - [X] Approximation algorithm
  - [X] Translation algorithm
  - [ ] Uniform scale algorithm
  - [ ] Scaling algorithm without maintaining aspect ratio
  - [ ] Calculate the length of a geometric primitive
  - [ ] B-Splines
  - [ ] Interpolated splines
  - [ ] Bézier curves
  - [ ] Elliptical sections
  - [ ] Closest point algorithm for all geometric primitives

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE_APACHE.md) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE_MIT.md) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

The easiest way to start contributing is to check the issue tracker and look for
an easy issue to tackle. Alternatively [the wishlist](#wishlist) contains a
list of features we'd like to implement, although these may require more effort
or experience.

We're always keen to help mentor contributors!

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[API Docs]: https://michael-f-bryan.github.io/arcs
[rustmatic-38]: https://github.com/Michael-F-Bryan/rustmatic/issues/38
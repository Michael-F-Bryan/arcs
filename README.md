# A Rust CAD System

[![Build Status](https://travis-ci.com/Michael-F-Bryan/arcs.svg?branch=master)](https://travis-ci.com/Michael-F-Bryan/arcs)
[![Docs.rs Badge](https://docs.rs/arcs/badge.svg)](https://docs.rs/arcs)
[![Crates.io](https://img.shields.io/crates/v/arcs)](https://crates.io/crates/arcs)
![Crates.io](https://img.shields.io/crates/l/arcs)

(**[API Docs for `master`][docs]**)

An extensible framework for creating 2D CAD applications, written in Rust and
based on an *Entity-Component-System* architecture.

If you want a high-level understanding of how this project is implemented and
the way things are designed, you may want to check out [A Thought Experiment:
Using the ECS Pattern Outside of Game
Engines][article].

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

- [ ] Milestone: Required by Real-World Applications
      (see [Michael-F-Bryan/rustmatic#38][rustmatic-38])
  - [ ] Robust undo/redo mechanism

- <span id="wishlist">Wish List</span>
  - [X] Z-levels so objects can be drawn on top of each other
  - [X] Entities can be tagged with a name to give them semantic meaning
  - [X] Approximation algorithm
  - [X] Translation algorithm
  - [X] Uniform scale algorithm
  - [x] Scaling algorithm without maintaining aspect ratio
  - [x] Calculate the length of a geometric primitive
  - [ ] B-Splines
  - [ ] Interpolated splines
  - [ ] Bézier curves
  - [ ] Elliptical sections
  - [x] Closest point algorithm for all geometric primitives

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE_APACHE.md) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE_MIT.md) or
   http://opensource.org/licenses/MIT)

at your option.

It is recommended to always use [cargo-crev][crev] to verify the
trustworthiness of each of your dependencies, including this one.

### Contribution

The easiest way to start contributing is to check the issue tracker and look for
an easy issue to tackle. Alternatively [the wish-list](#wishlist) contains a
list of features we'd like to implement, although these may require more effort
or experience.

We're always keen to help mentor contributors!

The intent of this crate is to be free of soundness bugs. The developers will
do their best to avoid them, and welcome help in analysing and fixing them.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[docs]: https://michael-f-bryan.github.io/arcs/crate_docs
[rustmatic-38]: https://github.com/Michael-F-Bryan/rustmatic/issues/38
[crev]: https://github.com/crev-dev/cargo-crev
[article]: http://adventures.michaelfbryan.com/posts/ecs-outside-of-games/

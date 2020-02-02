# A Rust CAD System

[![Build Status](https://travis-ci.com/Michael-F-Bryan/arcs.svg?branch=master)](https://travis-ci.com/Michael-F-Bryan/arcs)
[![Docs.rs Badge](https://docs.rs/arcs/badge.svg)](https://docs.rs/arcs)
[![Crates.io](https://img.shields.io/crates/v/arcs)](https://crates.io/crates/arcs)
![Crates.io](https://img.shields.io/crates/l/arcs)

(**[API Docs for `master`][docs]/[WebAssembly Demo][demo]**)

An extensible framework for creating 2D CAD applications, written in Rust and
based on an *Entity-Component-System* architecture.

If you want a high-level understanding of how this project is implemented and
the way things are designed, you may want to check out [A Thought Experiment:
Using the ECS Pattern Outside of Game
Engines](http://adventures.michaelfbryan.com/posts/ecs-outside-of-games/).

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
  - [x] Render correctly to a HTML5 `<canvas>`
  - Interactive tools for creating drawing objects:
    - [ ] Point
    - [ ] Line
    - [ ] Arc
  - [ ] Move items between layers
  - [ ] Selectively show/hide layers
  - [ ] `Viewport` panning
  - [ ] Zoom `Viewport` in/out

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

## Building the WebAssembly Demo

If you want to run the WebAssembly demo locally you'll first need a copy of the
code.

```console
$ git clone https://github.com/Michael-F-Bryan/arcs
```

You'll also need the [`wasm-pack`][wp] program. We'll use this to build the
demo.

```console
$ cargo install --force wasm-pack
    Updating crates.io index
  Installing wasm-pack v0.8.1
  Downloaded cc v1.0.50
  ...
  Compiling wasm-pack v0.8.1
    Finished release [optimized] target(s) in 2m 20s
   Installed /home/michael/.cargo/bin/wasm-pack
```

Now we can build the demo.

```console
$ cd arcs/demo
$ wasm-pack build --target web
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
   Compiling arcs-demo v0.1.0 (/home/michael/Documents/arcs/demo)
    Finished release [optimized] target(s) in 8.19s
:-) [WARN]: origin crate has no README
[INFO]: Installing wasm-bindgen...
[INFO]: :-) Done in 8.37s
[INFO]: :-) Your wasm pkg is ready to publish at ./pkg.
$ ls pkg
arcs_demo.d.ts arcs_demo.js arcs_demo_bg.d.ts arcs_demo_bg.wasm
package.json README.md
```

Now the demo is compiled, it can be served from disk.

```console
$ python3 -m http.server
Serving HTTP on 0.0.0.0 port 8000 (http://0.0.0.0:8000/) ...
127.0.0.1 - - [10/Jan/2020 18:31:35] "GET / HTTP/1.1" 200 -
127.0.0.1 - - [10/Jan/2020 18:31:35] "GET /pkg/arcs_demo.js HTTP/1.1" 200 -
127.0.0.1 - - [10/Jan/2020 18:31:35] "GET /pkg/arcs_demo_bg.wasm HTTP/1.1" 200 -
```

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
an easy issue to tackle. Alternatively [the wishlist](#wishlist) contains a
list of features we'd like to implement, although these may require more effort
or experience.

We're always keen to help mentor contributors!

The intent of this crate is to be free of soundness bugs. The developers will
do their best to avoid them, and welcome help in analyzing and fixing them.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[docs]: https://michael-f-bryan.github.io/arcs/crate_docs
[demo]: https://michael-f-bryan.github.io/arcs
[rustmatic-38]: https://github.com/Michael-F-Bryan/rustmatic/issues/38
[wp]: https://crates.io/crates/wasm-pack
[crev]: https://github.com/crev-dev/cargo-crev


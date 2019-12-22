# Arcs WebAssembly Demo

An online demo of the `arcs` CAD library.

## Getting Started

First you'll need to pull up a terminal and `cd` into this `wasm-demo/`
directory.

Next you can install all JavaScript dependencies from NPM.

```console
$ yarn install
yarn install v1.15.2
[1/4] Resolving packages...
success Already up-to-date.
Done in 1.61s.
```

You can use `yarn build` to generate a release build.

```console
$ yarn build
```

Use `yarn start` to spin up a dev server and automatically recompile and
reload the application.

```console
$ yarn start
yarn run v1.15.2
$ rimraf dist pkg && webpack-dev-server --open -d
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
    Blocking waiting for file lock on build directory
ℹ ｢wdm｣: wait until bundle finished: /
   Compiling proc-macro2 v1.0.6
   Compiling unicode-xid v0.2.0
   ...
   Compiling stdweb-internal-macros v0.2.9
   Compiling arcs-wasm-demo v0.1.0 (/home/michael/Documents/arcs/wasm-demo)
[INFO]: :-) Done in 4m 34s
[INFO]: :-) Your wasm pkg is ready to publish at ./pkg.
✅  Your crate has been correctly compiled

ℹ ｢wdm｣: Hash: 1ed6d6c0052d8ecf544c
Version: webpack 4.41.4
Time: 302886ms
Built at: 12/22/2019 10:33:08 PM
                           Asset       Size  Chunks                         Chunk Names
                            0.js    214 KiB       0  [emitted]
5f71c020937b91836ed8.module.wasm   1010 KiB       0  [emitted] [immutable]
                      index.html  167 bytes          [emitted]
                        index.js    912 KiB   index  [emitted]              index
Entrypoint index = index.js
[./pkg/index.js] 26.1 KiB {0} [built]
    + 81 hidden modules
ℹ ｢wdm｣: Compiled successfully.
```

This should automatically open the demo in your browser, otherwise you can go
to http://localhost:8080/.
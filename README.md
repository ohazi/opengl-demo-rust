# OpenGL Core Profile 3.3 Demo (Rust)

This is a reimplementation of Chris Wellons' [Minimal OpenGL 3.3 Demo][opengl-demo] project in Rust, using the [gl][gl] and [glfw][glfw] bindings. 

## Build
    $ cargo build

Cargo appears to be building glfw from source (via an optional [glfw-sys][glfw-sys] dependency) and linking it statically. OpenGL is linked dynamically, so you'll have to set that up separately. 

## Run
### Windowed
    $ cargo run
### Fullscreen
    $ cargo run -- -f

[opengl-demo]: https://github.com/skeeto/opengl-demo
[glfw]: https://crates.io/crates/glfw
[gl]: https://crates.io/crates/gl
[glfw-sys]: https://crates.io/crates/glfw-sys

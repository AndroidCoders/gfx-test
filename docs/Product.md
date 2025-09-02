File version: 1.02

**TLDR:**
This document describes the `gfx-test` project:
* Rust application for graphical effects using SDL3
* Displays animated white box on black background

`gfx-test` is a Rust application that uses the SDL3 library to create graphical effects. It displays an animated white box on a black background in a fullscreen window. The application exits when the 'Escape' key is pressed.

The codebase is modular, with `app.rs` for the main application, `game_state.rs` for managing the box's position and frame counter, and `renderer.rs` as a drawing module.

A key debug feature allows capturing and saving specific frames (1, 100, 200...900) as PNG images in the `output` directory upon exit. The debug feature is enabled by a feature flag:
cargo run --features debug-video-to-png

The project follows a clear structure, coding style, and requirements, and is built with Cargo.

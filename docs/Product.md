File version: 1.01

> **TLDR:**
> *   This document describes the `gfx-test` project.
> *   It's a Rust application for graphical effects using SDL3.
> *   Key features include an animated box and a debug frame-saving option.

This project, `gfx-test`, is a Rust application for creating graphical effects using the SDL3 library. It opens a fullscreen window and displays a white box animated on a black background. The application can be closed by pressing the 'Escape' key.

The codebase is organized into modules for the main application (`app.rs`), application state (`game_state.rs`), and a placeholder for a renderer (`renderer.rs`). The `game_state.rs` module manages the position of the box and a frame counter.

A key feature is the ability to capture and save specific frames as PNG images for debugging purposes, which can be enabled with a feature flag. The application saves frames 1, 100, 200, and so on, up to 900, into an `output` directory. The project's documentation outlines a clear structure, coding style, and a set of requirements. The project is built using Cargo.


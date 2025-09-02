File version: 1.01

**TLDR:**
This document outlines the architecture and components of the `gfx-test` project:
* Simple game loop architecture
* Key components: `app`, `renderer`, `game_state`, `frame_capture`

## Architecture

A simple game loop architecture will be used.

## Components

- `main.rs`: Entry point
- `app.rs`: Initializes SDL, creates the fullscreen canvas, and runs the main loop.
- `renderer.rs`: A module responsible for drawing on the canvas.
- `game_state.rs`: A module to hold the state of the application (e.g., pixel position).
- `renderer.rs`: A module responsible for drawing on the canvas.
- `game_state.rs`: A module to hold the state of the application (e.g., pixel position).
- `frame_capture.rs`: A conditionally compiled module for capturing and saving frames for debugging.

## Version Control

This project uses Git, with development done on `feature/` branches that are merged into `main` upon completion.

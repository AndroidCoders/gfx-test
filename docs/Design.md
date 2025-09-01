File version: 1.01

## Architecture

A simple game loop architecture will be used.

## Components

- `main.rs`: Entry point
- `app.rs`: Initializes SDL, creates the fullscreen canvas, and runs the main loop.
- `renderer.rs`: A module responsible for drawing on the canvas.
- `game_state.rs`: A module to hold the state of the application (e.g., pixel position).

## Version Control

This project uses Git, with development done on `feature/` branches that are merged into `main` upon completion.

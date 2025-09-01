File version: 1.00

> **TLDR:**
> *   This document provides an overview of the `gfx-test` project's file structure.
> *   It lists the main directories and key source code and documentation files.

- `README.md`: Project description.
- `docs/`: Folder for the Guiding Documents.
- `docs/Structure.md`: File structure overview.
- `docs/Product.md`: Product description.
- `docs/Tech.md`: Technology stack.
- `docs/Requirements.md`: Project requirements.
- `docs/Tasks.md`: Project tasks.
- `docs/Design.md`: Design and architecture.
- `docs/CodingStyle.md`: Coding conventions and development philosophy.
- `docs/Workflow.md`: Development workflow.
- `src/main.rs`: Main application source code.
- `src/app.rs`: Initializes SDL, creates the window, and runs the main application loop.
- `src/renderer.rs`: Handles all drawing operations on the canvas.
- `src/game_state.rs`: Game state module.
- `src/frame_capture.rs`: A conditionally compiled module for capturing and saving frames for debugging.
# Development Workflow

> **TLDR:**
> *   This document outlines the development workflow for the project.
> *   All work is done on separate branches.
> *   Pull requests are used to merge changes into `main`.
> *   Before committing, run `cargo check`, `cargo clippy`, and `cargo test`.

This document outlines the development workflow for the `gfx-test` project. Following this workflow will help maintain a clean and consistent codebase.

## Document Structure

Each guiding document in this project should start with a "TLDR" (Too Long; Didn't Read) section. This section provides a concise summary of the document's content, allowing readers to quickly grasp the main points without reading the entire document.

The TLDR section should be formatted as a blockquote, with each key point on a new line, similar to a bulleted list.

Example:

> **TLDR:**
> *   This document outlines the development workflow for the project.
> *   All work is done on separate branches.
> *   Pull requests are used to merge changes into `main`.

## Branching Strategy

All new work should be done on a separate branch. Branch names should be descriptive and follow this convention:

-   **Features:** `feature/<short-description>` (e.g., `feature/add-player-character`)
-   **Bugfixes:** `bugfix/<short-description>` (e.g., `bugfix/fix-rendering-glitch`)
-   **Refactoring:** `refactor/<short-description>` (e.g., `refactor/move-drawing-logic`)
-   **Documentation:** `docs/<short-description>` (e.g., `docs/update-readme`)

## Development Process

1.  **Create a Branch:**
    Start by creating a new branch from the `main` branch with a descriptive name.

    ```bash
    git checkout -b <branch-name>
    ```

2.  **Implement Changes:**
    Make your code changes, following the project's coding style and design principles.

3.  **Run Checks:**
    Before committing your changes, ensure that the code is correct and follows the project's standards by running the following commands:

    ```bash
    cargo check  # Check for compilation errors
    cargo clippy # Check for common Rust mistakes
    cargo test   # Run all tests
    ```

4.  **Update Documentation:**
    If your changes affect the project's design, requirements, or structure, update the relevant guiding documents in the `docs/` directory.

5.  **Commit Changes:**
    Commit your changes with a clear and descriptive commit message. Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

    ```bash
    git commit -m "feat: Add new feature" -m "Detailed description of the new feature."
    ```

## Pull Requests

-   **Push Your Branch:**
    Push your branch to the remote repository on GitHub.

    ```bash
    git push --set-upstream origin <branch-name>
    ```

-   **Create a Pull Request:**
    Go to the project's GitHub page and create a new pull request from your branch to the `main` branch.

-   **Code Review:**
    At least one other person should review your pull request before it is merged. The reviewer should check for correctness, style, and adherence to the project's standards.

## Merging

Once the pull request has been approved, it can be merged into the `main` branch. Use the "Squash and merge" option on GitHub to keep the commit history clean. After merging, the feature branch should be deleted.

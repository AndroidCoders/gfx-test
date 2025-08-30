The source code file is assigned a version number (like 1.00), and the version number is increased by 0.01 (from 1.00 to 1.01, etc) when there is a change to the code.

The source code file is also assigned a maturity status of "testing" after code changes, and then promoted to a maturity status of "stable" after running the code and passing all functional tests.

The version number and the maturity status is written in the source files, for example at the top of the source code file.

### Workflow

- Version number of the source code file is increased when the source code file is edited.
- Status of the source file is set to 'testing' when the source code file is edited.
- Source code files with a status of 'testing' is copied to the `src/testing` directory.
- After successful testing, the source code file status is promoted to 'stable'.
- Source code files with a status of 'stable' is copied to the `src/stable` directory.
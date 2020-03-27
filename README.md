# C++ Auto-Format
This is a GitHub Action that provides both automatic clang-formatting and CI
checks.

## Building
This GitHub Action is a docker container action, so it requires a build step.
A simple `docker build` command should be sufficient to build the Rust tool and
the final GitHub Action image.
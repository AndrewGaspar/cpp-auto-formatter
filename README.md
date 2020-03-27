# GitHub clang-format Action
This is a GitHub Action that can both automatically clang-format code and
provide a clang-format check.

## Building
This GitHub Action is a docker container action, so it requires a build step.
A simple `docker build` command should be sufficient to build the Rust tool and
the final GitHub Action image.
# Build the GitHub Action
FROM rust:1.42 as builder
WORKDIR /usr/src/myapp
COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src
RUN cargo install --path .

# GitHub Action Image
FROM ubuntu:18.04
# Install our apt packages
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y git

# Install clang-formats
ADD ./clang-format /clang-format
COPY --from=builder /usr/local/cargo/bin/clang-format-action /clang-format-action
FROM ubuntu:latest

# Install our apt packages
RUN apt-get update
RUN apt-get upgrade -y

# Install clang-formats
ADD ./clang-format /clang-format
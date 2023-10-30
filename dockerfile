# First stage - Rust build
FROM rust AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Second stage - Ubuntu setup
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN apt-get update && \
    apt-get install -y curl && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    echo "source $HOME/.cargo/env" >> ~/.bashrc && \
    /bin/bash -c "source ~/.bashrc" && \
    /root/.cargo/bin/rustup install stable && \
    /root/.cargo/bin/rustup default stable && \
    /root/.cargo/bin/rustup update && \
    apt-get install -y cargo

# Copy the app into the Docker image
COPY . /app

# Set the working directory to the app
WORKDIR /app

# Build the app
RUN cargo build --release

# Run the app
CMD ["./target/release/resizer"]
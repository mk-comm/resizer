# First stage - Rust build
FROM rust AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Second stage - Ubuntu setup
FROM ubuntu:22.04

# Copy the app into the Docker image
COPY . /app

# Set the working directory to the app
WORKDIR /app

# Build the app
RUN cargo build --release

# Run the app
CMD ["./target/release/resizer"]
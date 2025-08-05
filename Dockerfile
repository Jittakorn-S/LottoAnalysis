FROM rust:1.80-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy manifests first
COPY Cargo.toml Cargo.lock ./

# Pre-fetch dependencies without compiling
RUN cargo fetch

# Copy full source
COPY src ./src

# Build release binary
RUN cargo build --release

# ---- Builder Stage ----
FROM rust:1.81-slim-bookworm AS builder

# Install build dependencies with minimal extra packages
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy only dependency manifests first to cache builds
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -r src

# Copy actual source code
COPY src ./src

# Build application
RUN cargo build --release

# ---- Final Stage ----
FROM debian:bookworm-slim

# Install runtime-only dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN groupadd --system app && useradd --system --gid app app

WORKDIR /home/app

# Copy assets and set proper ownership
COPY --chown=app:app templates ./templates
COPY --chown=app:app static ./static

# Copy compiled binary and set permissions
COPY --from=builder /usr/src/app/target/release/lotto_analysis_rust .
RUN chown app:app lotto_analysis_rust

# Run as non-root user
USER app

# Expose application port
EXPOSE 8080

# Run the application
ENTRYPOINT ["./lotto_analysis_rust"]

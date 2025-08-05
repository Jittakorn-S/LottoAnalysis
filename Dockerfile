# -------- Builder Stage --------
FROM rust:1.80-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies to leverage layer caching
RUN cargo fetch

# Copy actual source code
COPY src ./src

# Build release binary
RUN cargo build --release

# Strip debug symbols to reduce binary size (optional)
RUN strip target/release/lotto_analysis_rust || true

# -------- Runtime Stage --------
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd --system app && useradd --system --gid app --home /home/app app

# Set working directory
WORKDIR /home/app

# Copy assets and binary from builder
COPY --chown=app:app templates ./templates
COPY --chown=app:app static ./static
COPY --from=builder /usr/src/app/target/release/lotto_analysis_rust ./lotto_analysis_rust

# Use non-root user
USER app

# Expose port for Render
EXPOSE 8080

# Run app
CMD ["./lotto_analysis_rust"]

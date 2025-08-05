# ---- Builder Stage ----
FROM rust:1.80-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/app

# Copy manifests first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to cache build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source
COPY src ./src

# Build final binary
RUN cargo build --release

# ---- Final Stage ----
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN groupadd --system app && useradd --system --gid app --home-dir /home/app app

# Set working directory
WORKDIR /home/app

# Copy assets and binary from builder
COPY --chown=app:app templates ./templates
COPY --chown=app:app static ./static
COPY --from=builder /usr/src/app/target/release/lotto_analysis_rust ./lotto_analysis_rust

# Set ownership (redundant since COPY already sets it, but safe)
RUN chown app:app lotto_analysis_rust

# Switch to non-root user
USER app

# Expose application port
EXPOSE 8080

# Run the application
CMD ["./lotto_analysis_rust"]

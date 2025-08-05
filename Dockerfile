# ---- Builder Stage ----
# Use the latest stable Rust image to ensure all dependencies are supported.
FROM rust:1.80-slim-bookworm AS builder

# Install build dependencies required for native libraries like OpenSSL.
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy dependency definitions first to leverage Docker's layer caching.
# This step is only re-run if Cargo.toml or Cargo.lock change.
COPY Cargo.toml Cargo.lock ./

# Build dependencies separately to cache them.
# This dummy build compiles dependencies without the full source code.
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/deps/lotto_analysis_rust*

# Now, copy the actual source code.
COPY src ./src

# Build the final application. This will be much faster as dependencies are cached.
RUN cargo build --release

# ---- Final Stage ----
# Use a minimal Debian image for a small and secure final container.
FROM debian:bookworm-slim

# Install only necessary runtime dependencies. `ca-certificates` is crucial for making HTTPS requests.
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user 'app' for enhanced security.
RUN groupadd --system app && useradd --system --gid app app

WORKDIR /home/app

# Copy frontend assets and set ownership to the 'app' user.
COPY --chown=app:app templates ./templates
COPY --chown=app:app static ./static

# Copy the compiled binary from the builder stage and set ownership.
COPY --from=builder /usr/src/app/target/release/lotto_analysis_rust .
RUN chown app:app lotto_analysis_rust

# Switch to the non-root user.
USER app

# Expose the port the application will run on.
EXPOSE 8080

# Set the command to run the application.
CMD ["./lotto_analysis_rust"]
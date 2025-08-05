# ---- Builder Stage ----
# Use the latest stable Rust version (1.80) to ensure dependency compatibility.
FROM rust:1.80-slim-bookworm AS builder

# Install build dependencies with minimal extras to reduce image size.
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory for the application.
WORKDIR /usr/src/app

# Copy dependency manifests to leverage Docker's layer caching.
COPY Cargo.toml Cargo.lock ./

# --- Build dependencies to cache them in a single layer ---
# This dummy build compiles dependencies without the full source code.
# This is the most time-consuming step and will be cached by Docker.
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/deps/lotto_analysis_rust*

# Copy the actual source code into a new layer.
COPY src ./src

# Build the final application. This will be much faster because dependencies are cached.
RUN cargo build --release

# ---- Final Stage ----
# Use a minimal Debian image for a small and secure final container.
FROM debian:bookworm-slim

# Install only the necessary runtime dependencies.
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user for enhanced security.
RUN groupadd --system app && useradd --system --gid app app

# Set the working directory for the final stage.
WORKDIR /home/app

# Copy assets and the compiled binary from the builder stage.
COPY --chown=app:app templates ./templates
COPY --chown=app:app static ./static
COPY --from=builder /usr/src/app/target/release/lotto_analysis_rust ./
RUN chown app:app lotto_analysis_rust

# Switch to the non-root user.
USER app

# Expose the port the application will run on.
EXPOSE 8080

# Set the default command to run the application.
CMD ["./lotto_analysis_rust"]
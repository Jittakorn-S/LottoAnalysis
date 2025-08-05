# -------- Builder Stage --------
FROM rust:1.80-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy manifests and source
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Fetch dependencies for layer caching
RUN cargo fetch

# Build release binary
RUN cargo build --release

# Optional: reduce binary size
RUN strip target/release/lotto_analysis_rust || true

# -------- Runtime Stage --------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --system app && useradd --system --gid app --home /home/app app

WORKDIR /home/app

COPY --chown=app:app templates ./templates
COPY --chown=app:app static ./static
COPY --from=builder /usr/src/app/target/release/lotto_analysis_rust ./lotto_analysis_rust

USER app

EXPOSE 8080

CMD ["./lotto_analysis_rust"]

FROM rust:1.88-slim as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY build.rs ./
COPY src ./src
COPY proto ./proto

RUN cargo build --release --bin server

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

RUN useradd -r -s /bin/false -m -d /app appuser

WORKDIR /app

COPY --from=builder /app/target/release/server ./embedding-server

RUN chown -R appuser:appuser /app

USER appuser

RUN mkdir -p /app/.cache

EXPOSE 6010

ENV RUST_LOG=info

HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD timeout 5s bash -c "</dev/tcp/localhost/6010" || exit 1

CMD ["./embedding-server"]
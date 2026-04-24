# syntax=docker/dockerfile:1
# `rust:latest` — последняя стабильная Rust в официальном образе (см. https://hub.docker.com/_/rust).
FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY assets ./assets
RUN cargo build --release

# Текущий stable Debian, чтобы glibc в рантайме соответствовал тому, под которым был линк бинарника.
FROM debian:stable-slim
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/rust-axum-template /app/server
COPY migrations ./migrations
COPY assets ./assets
COPY docker/config.docker.ini /app/config.docker.ini
ENV CONFIG_INI=/app/config.docker.ini
EXPOSE 8000
CMD ["/app/server"]

FROM rust:latest AS builder

WORKDIR /usr/src/axum-api
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/axum-api /usr/local/bin/axum-api

CMD ["axum-api"]
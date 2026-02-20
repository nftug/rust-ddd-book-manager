# syntax=docker/dockerfile:1

# Build stage
FROM rust:1.93-bookworm AS builder

WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/Cargo.toml
COPY application/Cargo.toml application/Cargo.toml
COPY domain/Cargo.toml domain/Cargo.toml
COPY infrastructure/Cargo.toml infrastructure/Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml

COPY api api
COPY application application
COPY domain domain
COPY infrastructure infrastructure
COPY migration migration

RUN cargo fetch --locked
RUN cargo build -p api --release --locked

# Deployment stage
FROM gcr.io/distroless/cc-debian12:nonroot

WORKDIR /app

COPY --from=builder --chown=nonroot:nonroot /workspace/target/release/api /app/api

ENV PORT=8080
EXPOSE 8080

USER nonroot:nonroot

CMD ["/app/api"]

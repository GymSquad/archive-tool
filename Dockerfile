FROM rust:1.68 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/archive-tool .

ENTRYPOINT  ["./archive-tool"]

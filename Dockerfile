FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

FROM debian:buster-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev wget
COPY --from=builder /app/target/release/archive-tool /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/archive-tool" ]
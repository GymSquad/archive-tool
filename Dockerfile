ARG BASE_IMAGE=rust:1.68

FROM ${BASE_IMAGE} as planner
WORKDIR /app
RUN cargo install cargo-chef --version 0.1.56 --locked
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM ${BASE_IMAGE} as cacher
WORKDIR /app
RUN cargo install cargo-chef --version 0.1.56 --locked
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM ${BASE_IMAGE} as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher ${CARGO_HOME} ${CARGO_HOME}
RUN cargo build --release

FROM alan910127/wget2:latest
WORKDIR /app
COPY --from=builder /app/target/release/archive-tool /
ENTRYPOINT [ "/archive-tool" ]
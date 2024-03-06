# Using the `rust-musl-builder` as base image, instead of
# the official Rust toolchain
FROM --platform=linux/amd64 clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM clux/muslrust:stable AS bunyan
RUN cargo install bunyan

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin cli

FROM scratch AS prod
WORKDIR /user
COPY src/public/config/00-default.toml 00-default.toml
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cli /usr/local/bin/rust-server
ENTRYPOINT ["/usr/local/bin/rust-server", "--config-path=*.toml"]

FROM alpine AS dev
WORKDIR /user
COPY src/public/config/00-default.toml 00-default.toml
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cli /usr/local/bin/rust-server
COPY --from=bunyan /root/.cargo/bin/bunyan /usr/local/bin/
ENTRYPOINT ["/bin/sh"]
CMD ["-c", "/usr/local/bin/rust-server --config-path=*.toml | bunyan"]

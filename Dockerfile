FROM rust:latest AS base
RUN apt update && \
    apt install -y sqlite3 && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown && \
    cargo install --locked trunk wasm-bindgen-cli  && \
    cargo install diesel_cli \
      --no-default-features \
      --features "sqlite"

FROM base AS ui-builder
COPY kyward-ui /kyward-ui
WORKDIR /kyward-ui
RUN trunk build --release

FROM base AS api-builder
COPY kyward-api /kyward-api
COPY --from=ui-builder \
    /kyward-ui/dist \
    /kyward-ui/dist
WORKDIR /kyward-api
RUN cargo build --release

FROM base AS db-init
COPY kyward-api /kyward-api
WORKDIR /kyward-api
RUN  diesel migration run

FROM debian:bookworm-slim
COPY --from=api-builder \
    /kyward-api/target/release/kyward \
    kyward
RUN apt update && \
    apt install -y sqlite3
COPY --from=db-init \
    /dev.sqlite \
    /dev.sqlite
COPY kyward-api/Rocket.toml .
EXPOSE 8000
CMD ["./kyward"]
FROM rust:latest AS ui-builder
COPY kyward-ui /kyward-ui
WORKDIR /kyward-ui
RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown && \
    cargo install trunk wasm-bindgen-cli
RUN trunk build --release

FROM rust:latest AS api-builder
COPY kyward-api /kyward-api
COPY --from=ui-builder \
    /kyward-ui/dist \
    /kyward-ui/dist
WORKDIR /kyward-api
RUN rustup default nightly
RUN cargo build --release

FROM ubuntu:latest
COPY --from=api-builder \
    /kyward-api/target/release/kyward \
    kyward
RUN apt update && \
    apt install -y sqlite3
COPY kyward-api/Rocket.toml .
EXPOSE 8000
CMD ["./kyward"]
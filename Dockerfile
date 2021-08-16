FROM rust:latest AS ui-builder
COPY kyward-ui /kyward-ui
WORKDIR /kyward-ui
RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown && \
    cargo install trunk wasm-bindgen-cli
RUN trunk build

FROM rust:latest AS api-builder
COPY kyward-api /kyward-api
COPY --from=ui-builder \
    /kyward-ui/dist \
    /kyward-ui/dist
WORKDIR /kyward-api
RUN rustup default nightly
RUN cargo build

FROM scratch
COPY --from=api-builder \
    /kyward-api/target/debug/kyward \
    /kyward
CMD ["/kyward"]
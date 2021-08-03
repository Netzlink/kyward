dev.api.watch: kyward-api/Cargo.toml dev.ui.build
	cd kyward-api && \
	systemfd --no-pid -s http::8080 -- cargo watch -x run
dev.api.run: kyward-api/Cargo.toml dev.ui.build
	cd kyward-api && \
	cargo run
dev.db.migrate:
	diesel migration run
dev.db.remigrate:
	diesel migration redo
dev.ui.run:
	cd kyward-ui && \
	trunk serve
dev.ui.build:
	cd kyward-ui && \
	cargo build
dev.setup:
	rustup default nightly
	rustup target add wasm32-unknown-unknown
	cargo install trunk wasm-bindgen-cli
	cargo install systemfd cargo-watch
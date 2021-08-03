dev.api.watch: kyward-api/Cargo.toml
	cd kyward-api && \
	systemfd --no-pid -s http::8080 -- cargo watch -x run
dev.api.run: kyward-api/Cargo.toml
	cd kyward-api && \
	cargo run
dev.db.migrate:
	diesel migration run
dev.db.remigrate:
	diesel migration redo
dev.setup: Cargo.toml
	rustup default nightly
	cargo install systemfd cargo-watch
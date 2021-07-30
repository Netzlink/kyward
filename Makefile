dev.watch: Cargo.toml
	systemfd --no-pid -s http::8080 -- cargo watch -x run
dev.run: Cargo.toml
	cargo run
dev.setup: Cargo.toml
	rustup default nightly
	cargo install systemfd cargo-watch

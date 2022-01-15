run:
	cargo +nightly run

format:
	rustfmt src/*.rs

lint: format
	cargo clippy

clean:
	cargo clean

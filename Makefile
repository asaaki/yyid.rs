ci:
	cargo build
	cargo fmt -- --check --verbose
	cargo clippy
	cargo test --verbose
	cargo run --example basic
	@echo "Done."

local:
	cargo build --verbose
	cargo fmt -- --verbose
	cargo clippy
	cargo test --verbose
	cargo run --example basic

print-version:
	cargo run --example version

release-tag:
	$(shell cargo run --example release-tag)
	git push --tags

publish:
	cargo publish

release: release-tag publish

# cargo install cargo-criterion
benchmark:
	RUSTFLAGS="-C target-cpu=native" cargo criterion --benches

bench: benchmark

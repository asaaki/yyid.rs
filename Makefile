ci:
	cargo build --all-features
	cargo fmt -- --check --verbose
	cargo clippy --all-features
	cargo test --all-features --verbose
	cargo run --all-features --example basic
	@echo "Done."

local:
	cargo build --all-features --verbose
	cargo fmt -- --verbose
	cargo clippy --all-features
	cargo test --all-features --verbose
	cargo run --all-features --example basic

print-version:
	cargo run --example version

release-tag:
	$(shell cargo run --example release-tag)
	git push --tags

docs:
	cargo doc --all-features --open

publish:
	cargo publish

release: release-tag publish

# cargo install cargo-criterion
benchmark:
	RUSTFLAGS="-C target-cpu=native" cargo criterion --benches

bench: benchmark

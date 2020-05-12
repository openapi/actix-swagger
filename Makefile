.PHONY: demo demo-compile demo-format demo-check publish-swagg publish-actix

demo: demo-compile demo-format demo-check

demo-compile:
	@cargo run --package cargo-swagg -- ./demo/openapi.json --out-file ./demo/src/lib.rs
	@cargo run --package cargo-swagg -- ./demo/openapi.yaml --out-file ./demo/src/lib.rs

demo-format:
	@rustfmt -v ./demo/src/lib.rs

demo-check:
	@cargo check --package demo

publish-swagg:
	@cargo publish --manifest-path=./swagg/Cargo.toml; cargo publish --manifest-path=./cargo-swagg/Cargo.toml

publish-actix:
	@cargo publish --manifest-path=./actix-swagger/Cargo.toml

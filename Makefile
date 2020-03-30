.PHONY: demo demo-compile demo-format publish-swagg publish-actix

demo: demo-compile demo-format

demo-compile:
	@cargo run --package cargo-swagg -- /Users/sergeysova/Projects/authmenow/backend/public-api.openapi.yaml --out-file ./demo/src/lib.rs

demo-format:
	@rustfmt -v ./demo/src/lib.rs

publish-swagg:
	@cargo publish --manifest-path=./swagg/Cargo.toml
	@cargo publish --manifest-path=./cargo-swagg

publish-actix:
	@cargo publish --manifest-path=./actix-swagger/Cargo.toml

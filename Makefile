.PHONY: all demo format

all: demo format

demo:
	@cargo run --package cargo-swagg -- /Users/sergeysova/Projects/authmenow/backend/public-api.openapi.yaml --out-file ./demo/src/lib.rs

format:
	@rustfmt -v ./demo/src/lib.rs

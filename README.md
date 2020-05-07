# actix swagger

## Usage

> Not for production use yet

```bash
# Add cargo command to simplify usage
cargo install cargo-swagg

# Add support library to your project (via cargo-edit or manual)
cargo add actix-swagger

# Generate your code with cargo subcommand
cargo swagg ./openapi.yaml --out-file ./src/api.rs

# Format file after
rustfmt ./src/api.rs
```

## Development

It uses [insta](https://github.com/mitsuhiko/insta) for snapshot testing.

Install `cargo-insta` to better review experience.

### Members

- `cargo-actix` — support library, contents typed response named `Answer` and custom `Method` and `ContentType` that supports in swagg
- `swagg` — library that transforms openapi3 (yaml|json) spec to rust code
- `cargo-swagg` — same as `swagg` but for cli
- `demo` — checks that generated code is compiles

### Demo

```bash
# to convert ./demo/openapi.yaml to ./demo/src/lib.rs
# format ./demo/src/lib.rs
# and check just run
make
```

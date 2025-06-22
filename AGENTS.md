# MicroMcp Agent Instructions

MicroMcp is a Ruby gem with a Rust extension. It provides a simple way to build MCP servers that run over stdio. All Ruby sources are under `lib/`, while the Rust extension lives in `ext/micro_mcp`.

## Running checks

Always run `bundle exec rake` before committing. This command compiles the extension, runs Ruby and Rust tests, and lints the code with StandardRB and clippy. Additionally, run `cargo fmt --all` to keep the Rust code formatted consistently.

## Development tips

- Use Ruby 3.1 or newer.
- Install dependencies with `bin/setup` or `bundle install`.
- Avoid committing build artifacts such as `target/` from Cargo.


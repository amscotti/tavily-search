# Tavily Search Project Guidelines

## Build Commands
- Build: `cargo build`
- Run: `cargo run`
- Release build: `cargo build --release`
- Check: `cargo check`

## Test Commands
- Run all tests: `cargo test`
- Run specific test: `cargo test test_name`
- Run tests with output: `cargo test -- --nocapture`

## Lint Commands
- Clippy (linter): `cargo clippy`
- Format code: `cargo fmt`
- Check formatting: `cargo fmt -- --check`

## Code Style Guidelines
- Use 4-space indentation
- Follow Rust standard naming conventions (snake_case for variables/functions, CamelCase for types)
- Group imports by standard library, external crates, then local modules
- Use strong typing - minimize use of `Option` and `Result` unwrapping
- Prefer pattern matching over conditional statements where appropriate
- Handle all errors explicitly - no `.unwrap()` in production code
- Add documentation comments (`///`) for public functions and types
- Use `#[derive]` attributes for common traits when possible
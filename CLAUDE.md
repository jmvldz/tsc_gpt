# TSC_GPT Project Guide

## Build and Run Commands
- Build: `cargo build`
- Run: `cargo run`
- Check: `cargo check`
- Release build: `cargo build --release`
- Lint: `cargo clippy`
- Format: `cargo fmt`
- Test: `cargo test`
- Test single: `cargo test test_name`
- Test specific module: `cargo test module::test_name`
- Doc tests: `cargo test --doc`

## Code Style Guidelines
- **Imports**: Group standard library, external crates, then internal modules
- **Formatting**: Follow rustfmt conventions (`cargo fmt`)
- **Types**: Use strong typing with descriptive names; prefer Result<T, E> for error handling
- **Naming**: snake_case for variables/functions, CamelCase for types/structs
- **Error Handling**: Use ? operator for propagating errors; avoid unwrap() except in tests
- **Doc Comments**: Add /// doc comments for public API functions and types
- **Constants**: Use SCREAMING_SNAKE_CASE for constants
- **Function Length**: Keep functions focused and concise (< 50 lines preferred)
- **Testing**: Write unit tests for all public functionality
- **String Formatting**: Prefer format!() over string concatenation with +

## Project Structure
This is a Rust CLI tool that runs TypeScript's type checker (tsc) and summarizes errors using GPT API.
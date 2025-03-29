# Qlty Development Guide for AI Assistants

## Build & Test Commands

- Check that the code compiles (faster than build): `cargo check`
- Build: `cargo build`
- Run auto-formatter: `qlty fmt`
- Run linter: `qlty check --fix --no-formatters`
- Run all tests: `cargo test`
- Run specific test: `cargo test test_name_here`
- With coverage: `cargo llvm-cov --lcov --output-path target/lcov.info -- --include-ignored`
- Run project Makefile task: `cargo make task_name`

## Code Style Guidelines

- Follow standard Rust idioms and `.qlty/configs/.rustfmt.toml` settings
- Use `anyhow::Error` for errors and `thiserror` for defining error types
- Use `anyhow::Result` for return values instead of the built-in `Result`
- Naming: snake_case for functions/variables, UpperCamelCase for types/enums
- Always use strong typing with enums for bounded sets of values
- Imports: group std first, then external crates, then internal modules
- Comprehensive error handling with proper context using `context()` or `with_context()`
- Tests live alongside implementation in `tests/` module or `#[cfg(test)]` blocks
- Use descriptive variable names that clearly express intent
- Write docstrings for public APIs and complex functions

Run lint check, tests, and build to verify code quality.

Execute these commands in sequence:
1. `cargo fmt --check` - Check code format
2. `cargo clippy -- -D warnings` - Lint check (treat warnings as errors)
3. `cargo test` - Run all tests
4. `cargo build --release` - Build the project

Report any failures with the specific error messages.

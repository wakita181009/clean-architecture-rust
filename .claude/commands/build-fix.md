# Build and Fix

Incrementally fix Rust compilation and build errors:

1. Run build: `cargo build`

2. Parse error output:
   - Group by file
   - Sort by severity (compilation errors first, then warnings)

3. For each error:
   - Show error context (5 lines before/after)
   - Explain the issue
   - Propose fix
   - Apply fix
   - Re-run build
   - Verify error resolved

4. Stop if:
   - Fix introduces new errors
   - Same error persists after 3 attempts
   - User requests pause

5. Show summary:
   - Errors fixed
   - Errors remaining
   - New errors introduced

Fix one error at a time for safety!

## Common Rust Build Errors

### Compilation Errors
- Type mismatch: Check expected vs actual types, consider `.into()` or explicit conversion
- Borrow checker: Review ownership, consider `clone()`, `Arc`, or refactoring
- Lifetime errors: Add explicit lifetimes or restructure to avoid them
- Missing trait: Check if trait is in scope (`use` statement) or needs to be implemented

### Async Errors
- `async_trait` missing: Add `#[async_trait]` macro
- Future not Send: Check for non-Send types across await points
- Missing `.await`: Ensure all async calls are awaited

### Dependency Errors
- Version conflicts: Check `Cargo.toml` and run `cargo update`
- Missing features: Add required features to dependency declaration
- Workspace issues: Check workspace `Cargo.toml`

## Commands Reference

```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo build -p domain          # Build specific package
cargo check                    # Fast check without building
cargo fmt                      # Fix code format
cargo clippy --fix             # Auto-fix lint issues
```

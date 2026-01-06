# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with Rust migration project in this repository.

## Project Overview

libxaac is a production-grade AAC audio codec library supporting multiple MPEG audio standards. The codebase consists of C implementations for both encoder and decoder, with an ongoing Rust migration.

## Architecture

An original C implementation is used as a reference for the new Rust implementation. The Git history of original implementation is placed in the origin/main branch. We should not make any changes in the original C code.

### Directory Structure

libxaac/
├── docs/
│   └── LIBXAAC-Enc-API.pdf    # the main AAC codec documentation
├── common/         # Shared types, constants, ROM tables
├── decoder/        # AAC decoder
│   ├── drc_src/
│   ├── src/        # new Rust implementation
│   └── x86_64/     # Platform-specific SIMD
├── encoder/        # AAC encoder
│   └── drc_src/
└── test/           # Testing data and applications
    ├── decoder/    # CLI decoder integration test
    └── encoder/    # CLI encoder integration test

### File creating rules

- Put generated analysis document into `docs` directory
- Prefix all document files with `claude_`  

### Rust Migration

The decoder is being incrementally migrated to Rust following a 5-phase cycle per group of functions (see `RUST.migration.md`):

1. **Isolate** - Identify group of C functions
2. **Stub** - Create Rust wrappers calling C via FFI
3. **Tests** - Write unit tests with `--features fallback`
4. **Implement** - Pure Rust, validated by tests
5. **Integrate** - Export as C-compatible API

The new Rust implementation is committing into dev/rusty_water branch.

**Key files:**
- `decoder/src/lib.rs` - Library root with FFI exports
- `decoder/src/ixheaacd.rs` - Module publishing migrated functions
- `decoder/src/gen_ixheaacd_ref.rs` - Auto-generated FFI bindings (bindgen)
- `decoder/build.rs` - Bindgen configuration

**Cargo features:**
- `fallback` - Use C implementation via FFI (testing/validation)
- `integration` - Export Rust functions as C-compatible API for integration test


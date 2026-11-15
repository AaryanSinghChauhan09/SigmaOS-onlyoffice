# SigmaOS-onlyoffice

> **OnlyOffice Suite Coordinator** — OnlyOffice integration and suite coordinator for SigmaOS.

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) ecosystem.  
Parity / inspiration: **OnlyOffice**

## Quickstart

```bash
cargo test    # run deterministic AAA unit tests
cargo check   # verify compilation
```

## Component Role

This crate provides the core suite coordination for SigmaOS OnlyOffice:
- Fully real Rust implementation (no stubs, no placeholders)
- Fail-fast validation with typed domain errors
- Deterministic unit tests (Arrange-Act-Assert pattern)
- Single Responsibility, Modular, Loose Coupling

## API

```rust
use sigmaos_onlyoffice::*;
// See src/lib.rs for the full public API surface
```

## Documentation

Full documentation: [SigmaOS GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

## License

AGPL-3.0

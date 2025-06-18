# merkltrees

A Rust library project for building and working with Merkle trees. Merkle trees summarize collections of values by hashing leaves and combining child hashes up the tree; they are useful when comparing large datasets or proving that a value belongs to a collection without sending every item.

The crate is named `merkle` and provides a generic tree implementation with selectable digest primitives. Its API supports constructing a tree, comparing tree contents, updating values, and recomputing hashes. The digest implementation needed by the crate is vendored under `vendor/digest-primitives`, so the project does not depend on a sibling checkout.

## Build

Install a recent Rust toolchain with Cargo and build the crate from the repository root:

```bash
cargo build
```

Run the crate's test suite with:

```bash
cargo test
```

## Project layout

- `Cargo.toml` defines the Rust package and local digest dependency.
- `src/` contains the Merkle tree implementation and public library entry point.
- `vendor/digest-primitives/` contains the digest code required for standalone builds.
- `Cargo.lock` records the resolved dependency graph for reproducible application builds.

The vendored implementation retains its source attribution comments and notices. Consult the Rust modules for concrete type names and supported digest algorithms; the API is still evolving.


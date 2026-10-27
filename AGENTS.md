# fractic-core

Small utilities shared by every Fractic Rust crate: `collection!` and other
macros, vector algorithms, `FxFingerprintMap`, `impl_display_with_serialize!`,
`Lockable` / `LockableArc` (closure-scoped `tokio` lock access),
`req_not_none`, serialization helpers. Flat library: `src/lib.rs` declares
one private module per concern and `pub use`s each. Errors come from
`fractic-server-error`.

- Keep additions small, generic and dependency-light: a utility that needs
  an AWS SDK belongs in the matching `fractic-aws-*` crate, one that needs
  a TTY in `fractic-cli-scaffolding`. A new concern is a new file plus
  `mod` / `pub use` pair, not an addition to `utils.rs`.
- Commit / PR titles: imperative, trailing period.

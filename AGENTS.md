# fractic-core

Small utilities shared by every Fractic Rust crate: `collection!` and other
macros, vector algorithms, `FxFingerprintMap`, `impl_display_with_serialize!`,
`Lockable` / `LockableArc` (closure-scoped `tokio` lock access),
`req_not_none`, serialization helpers. Public, "provided as-is"
(`fractic-io/rust-core`, identity `fractic`), consumed as a git dependency
and `[patch]`ed to this checkout by the product repos.

## Read first

- `~/.agents/AGENTS.md` (workspace orientation; loaded automatically by Claude
  Code, read it if you are Codex or Grok) and `~/.agents/guides/rust.md`
  (style; §2 describes the flat library layout this crate uses). The shared
  style guide that used to live in this crate (`res/style.md`, compiled into
  consumers' `AGENTS.md` by `DocCompiler`) has moved there; the compiler is
  gone.

## Layout

Flat library: `src/lib.rs` declares one private module per concern and
`pub use`s each (`algorithms_vec`, `fx_fingerprint_map`,
`impl_display_with_serialize`, `lockable`, `req_not_none`, `serialization`,
`utils`). No `_.rs`, no resources. Errors come from `fractic-server-error`.

## Working here

- There is no flake here; build with a product devshell, for example
  `nix develop /efs/fractic/storyengine/util --command cargo test`.
- Everything is public API for the other crates: keep additions small,
  generic and dependency-light (a utility that needs an AWS SDK belongs in the
  matching `fractic-aws-*` crate; one that needs a TTY belongs in
  `fractic-cli-scaffolding`). Add a new concern as a new file + `mod` /
  `pub use` pair, not into `utils.rs`.
- Consumers pick a change up immediately through `[patch]`; they only see the
  pushed git revision once they `cargo update -p fractic-core`. Say in the PR
  if a change is breaking.
- Commit / PR titles: imperative, trailing period, PR number appended by the
  squash merge.

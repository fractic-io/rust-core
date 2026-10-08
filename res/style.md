# General Principles

- Prefer small, cohesive modules over long mixed-purpose files. A file should usually have one dominant reason to change.
- Keep public type definitions easy to scan. If a module defines several related structs/enums, group the related caller-facing definitions together before longer impl blocks.
- In medium/large files, use section headers as a reader-facing map of the module's API and mechanics. Use `Definitions` for the compact vocabulary of public/domain types that callers need to understand before reading behavior; use `Public interface` for the main public type, trait, functions, and caller-facing impls; use `Internal`, `Helpers`, and `Tests` for implementation details, free utilities, and tests:

```rust
// Definitions.
// ----------------------------------------------------------------------------

...

// Public interface.
// ----------------------------------------------------------------------------

...

// Internal.
// ----------------------------------------------------------------------------

...
```

- Put the main public type or trait and its caller-facing impls under `Public interface`; keep private state types, parsing structs, and implementation-only impls under `Internal`; keep free utility functions under `Helpers`. Do not move a private helper type into the public definitions area just because it is a type definition.
- Use subsection headers such as `Internal: Parsing.` or `Helpers: Search.` when a long file has clearly distinct implementation or helper groups. Prefer a short domain label that explains the concern, not the Rust construct.
- Put file-local `#[cfg(test)]` test modules at the end of the file under a `Tests` section header:

```rust
// Tests.
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    ...
}
```

- Small, tightly scoped helper types or functions may live inside the function that uses them when they are not reused elsewhere.
- Use concise, factual doc comments for public input/configuration structs and enums. Explain what the caller provides, not how the internals work.
- Put imports at the top of the file. Avoid `use` statements midway through code.
- Avoid absolute paths unless they make the code much clearer, such as resolving duplicate type names. Prefer `use` statements at the top of the file.
- Keep orchestration separate from mechanics. Entrypoint files should assemble dependencies and delegate to focused modules rather than owning terminal logic, parsing, rendering, and state transitions directly.
- Look for similar code before adding new patterns. Mirror established naming, module boundaries, comment headers, error handling, and test structure where possible.
- Prefer idiomatic borrowing and ownership over defensive cloning. Clone only when ownership, lifetime, or API boundaries require it.
- Avoid temporary locals that only name a single borrowed expression for one immediate call. Inline them when it keeps ownership and lifetimes clear.
- Minimize visibility. Keep helpers, types, functions, fields, and modules private by default.
- If code is shared by multiple focused modules but is not part of the caller-facing API, move it into a small internal utility module instead of making an unrelated UI or config module public.

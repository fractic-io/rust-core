# General Principles

- Prefer small, cohesive modules over long mixed-purpose files. A file should usually have one dominant reason to change.
- Keep public type definitions easy to scan. If a module defines several related structs/enums, group the definitions together before longer impl blocks.
- Separate definitions from behavior in medium/large files with section headers:

```rust
// Definitions.
// ----------------------------------------------------------------------------

...

// Impls.
// ----------------------------------------------------------------------------

...
```

- Use concise, factual doc comments for public input/configuration structs and enums. Explain what the caller provides, not how the internals work.
- Keep constructors, `Default`, conversion impls, and helper methods in an `Impls` section when colocating them with definitions.
- Put imports at the top of the file. Avoid `use` statements midway through code.
- Avoid absolute paths unless they make the code much clearer, such as resolving duplicate type names. Prefer `use` statements at the top of the file.
- Keep orchestration separate from mechanics. Entrypoint files should assemble dependencies and delegate to focused modules rather than owning terminal logic, parsing, rendering, and state transitions directly.
- Look for similar code before adding new patterns. Mirror established naming, module boundaries, comment headers, error handling, and test structure where possible.
- Prefer idiomatic borrowing and ownership over defensive cloning. Clone only when ownership, lifetime, or API boundaries require it.
- Minimize visibility. Keep helpers, types, functions, fields, and modules private by default.
- If code is shared by multiple focused modules but is not part of the caller-facing API, move it into a small internal utility module instead of making an unrelated UI or config module public.

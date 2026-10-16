# {{project-name}}

A Rust procedural macro workspace template built with
[`manyhow`](https://github.com/ModProg/manyhow),
[`darling`](https://github.com/TedDriggs/darling),
[`syn`](https://github.com/dtolnay/syn),
[`quote`](https://github.com/dtolnay/quote) and
[`proc-macro2`](https://github.com/dtolnay/proc-macro2).

- Rust edition: `2024`
- Macro entry points & diagnostics: `manyhow` 0.14
- Attribute parsing: `darling` 0.24
- Token parsing: `syn` 3
- Code generation: `quote` + `proc-macro2`

## Workspace layout

| Crate | Type | Responsibility |
| --- | --- | --- |
| `{{project-name}}-macros` | proc-macro crate | Macro implementations. A proc-macro crate can only export macros, so never place regular public items here. |
| `{{project-name}}` | regular library | Runtime facade that re-exports the macros. Put traits and helper functions referenced by generated code here. |

## Included example macros

### `#[derive(Getter)]` — derive macro, parsed with darling

Generates getter methods for every field of a named struct:

```rust
use {{project-name}}::Getter;

#[derive(Getter)]
struct User {
    #[getter(copy)]                       // return by value instead of by reference
    id: u64,
    name: String,                         // -> fn name(&self) -> &String
    #[getter(skip)]                       // do not generate a getter
    password: String,
    #[getter(copy, name = "user_age")]    // override the method name
    age: u32,
}

#[derive(Getter)]
#[getter(prefix = "get_")]                // container-level attribute
struct Point {
    x: i32,
    y: i32,
}
```

### `#[duration]` — attribute macro, arguments parsed with darling

Wraps a synchronous function and prints its elapsed time to stderr.
Supports `#[duration(us)]`, `#[duration(ms)]` (default) and `#[duration(sec)]`.
Using it on an `async fn` or specifying multiple units produces a compile error.

```rust
use {{project-name}}::duration;

#[duration(ms)]
fn slow_work() {
    // ...
}
```

### `hashmap!` — function-like macro, parsed with syn's `Parse` trait

darling only handles derives and helper attributes, so function-like macros
parse their token input directly. This example shows a custom `Parse`
implementation over `Punctuated`, plus a duplicate literal-key check that
fails compilation with a precisely spanned error:

```rust
use std::collections::HashMap;
use {{project-name}}::hashmap;

let map: HashMap<String, i32> = hashmap! {
    "a" => 1,
    "b" => 2,
};
```

## Error handling with manyhow

Entry points are annotated with `#[manyhow(proc_macro*)]`, so handlers take
`proc_macro2::TokenStream` (or any `syn::Parse` type) and return
`manyhow::Result` — no manual `into_compile_error()` boilerplate.

- **Spanned single errors** — `?` works directly for `syn::Error` and
  `darling::Error` (the `darling` feature is enabled), and
  `manyhow::bail!` / `error_message!` attach diagnostics to any token span.
- **Aggregated diagnostics with `Emitter`** — derive and attribute macros
  emit several errors before failing, so one compilation reports every
  problem: `#[derive(Getter)]` flags all fields whose getters collide, and
  `#[duration(us, sec)] async fn` reports both the conflicting units and the
  unsupported `async fn` at once.
- **Panics become compile errors** — a panic inside a macro is caught and
  reported as `proc macro panicked` at the macro call site with the panic
  message attached, instead of an opaque compiler crash.

Note the position difference: derive/attribute macros expand to items, where
multiple `compile_error!` tokens coexist legally. A function-like macro
expands in expression position, where several `compile_error!` tokens in a
row cause cascading parse errors — so `hashmap!` bails on the first problem
rather than collecting them.

## Build and test

```bash
cargo test
cargo clippy --all-targets -- -D warnings
```

Integration tests that actually expand the macros live in
`{{project-name}}/tests/macros` and double as usage documentation.

## Adding a new macro

1. Create a new module under `{{project-name}}-macros/src/` and write the
   implementation against `proc_macro2::TokenStream`, returning
   `manyhow::Result<TokenStream>` so errors surface as spanned compile
   errors. Accept an `emitter: &mut Emitter` parameter to aggregate multiple
   diagnostics.
2. Register the entry point in `{{project-name}}-macros/src/lib.rs` with a
   single attribute: `#[manyhow(proc_macro_derive(Trait, attributes(...)))]`,
   `#[manyhow(proc_macro_attribute)]` or `#[manyhow(proc_macro)]`.
3. Re-export it from `{{project-name}}/src/lib.rs`.
4. Add coverage in `{{project-name}}/tests/`.

Generated code should use fully-qualified paths (for example
`::std::collections::HashMap`) or items re-exported from the facade crate,
so it works regardless of the caller's imports.

# {{ project-name }}

A GPUI WebAssembly application template using `gpui-kit`.

## Features

- **GUI Framework**: [gpui-kit](https://github.com/longbridge/gpui-kit) (GPUI) with component library and asset support
- **Workspace**: Multi-crate workspace (`{{ project-name }}-ui`, `{{ project-name }}-web`)
- **WebAssembly**: `gpui-kit` web backend running in the browser via Trunk
- **i18n**: [rust-i18n](https://github.com/longbridge/rust-i18n) with `zh-CN` / `en` locales
- **Trunk**: No Node.js, npm or shell scripts required

## Project Structure

```
.
├── index.html                 # Trunk 入口 HTML (根目录直接 trunk serve)
├── Trunk.toml                 # Trunk 配置
├── rust-toolchain.toml        # 锁定 nightly + wasm32-unknown-unknown
├── {{ project-name }}-ui/     # Shared views, i18n and assets
│   ├── src/
│   │   └── lib.rs
│   ├── assets/                # Embedded assets (rust-embed)
│   │   └── icons/
│   └── locales/               # i18n translation files (rust-i18n)
├── {{ project-name }}-web/    # WebAssembly entry point
│   ├── src/
│   │   └── lib.rs
│   └── fonts/                 # Fonts embedded in the WASM module
├── .github/
│   └── workflows/             # CI/CD pipelines
├── cargo-generate.toml
└── Cargo.toml                 # Workspace root
```

## Quick Start

- Rust nightly (required by GPUI's web dependencies)

```bash
cargo install trunk --locked

trunk serve                # dev server with auto-reload, http://127.0.0.1:3000
trunk build --release      # static bundle in dist/
```

`dist/` can be served by any static host.

## Notes

- `rust-toolchain.toml` at workspace root pins `nightly` channel and `wasm32-unknown-unknown`
  target (GPUI's web dependencies use the unstable `stdarch_wasm_atomic_wait`); rustup
  installs both on the first build.
- `[tools] wasm_bindgen` in `Trunk.toml` must match the `wasm-bindgen` version in
  `Cargo.lock`, otherwise Trunk aborts with a version-mismatch error.
- `index.html` declares `data-trunk rel="rust"` pointing to `{{ project-name }}-web/Cargo.toml`,
  and `data-wasm-opt="0"` keeps `wasm-opt` out of the toolchain (change it to `"z"` to shrink
  the release bundle; Trunk then downloads `wasm-opt` automatically).
- `data-target-name="app_web"` selects the `cdylib` library target of the web crate.
- Web support is currently **showcase-level** in GPUI Kit; verify input, accessibility,
  startup cost and deployment for your own product.
- The web platform has no system font database. `IBMPlexSans-Regular.ttf` is embedded
  (SIL Open Font License, see `{{ project-name }}-web/fonts/LICENSE.txt`) and serves as
  both the UI font and the `.SystemUIFont` alias. Other glyphs fall back to the browser's Canvas.
- GPUI Kit's default icon set is not downloadable on the web. The template embeds the
  icons its components use with `gpui_kit::assets::icon_assets!` (see
  `{{ project-name }}-ui/src/lib.rs`); add icon names to that list when you introduce
  components that need more, otherwise those icons simply render empty.
- The web runtime supports a single top-level window; render dialogs inside `Root`.

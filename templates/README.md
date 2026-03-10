# Template Catalog

This repository organizes templates by **language** and **function**.

## Directory layout

- `templates/rust/ffi/cargo-cxx`
  - Rust-first CXX interop template (Rust + C++ FFI bridge).
- `templates/rust/docker/basic`
  - Minimal Rust application template with Docker multi-stage build.
- `templates/cpp/cmake/vcpkg-rust-ffi`
  - C++-first CMake + vcpkg project with Rust static library integration.

## Classification rules

- First level: primary language (`rust`, `cpp`, ...).
- Second/third level: functional category (for example `ffi`, `docker`, `cmake`).
- Final level: concrete template name.

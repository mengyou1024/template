//! Runtime facade crate.
//!
//! A proc-macro crate (`proc-macro = true`) can only export procedural
//! macros. This regular library crate re-exports them so that users depend
//! on a single crate, and it is also the place to add any runtime support
//! code (traits, helper functions) referenced by the generated code.

pub use {{ project-name | replace: "-", "_" }}_macros::{Getter, duration, hashmap};

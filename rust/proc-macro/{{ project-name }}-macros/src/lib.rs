//! Procedural macro implementation crate.
//!
//! Entry points use `#[manyhow(...)]`, so the handlers below are plain
//! functions that take `proc_macro2` token streams (or a type implementing
//! `syn::Parse`) and return `manyhow::Result`. manyhow converts errors
//! (including panics) into correctly spanned `compile_error!` tokens.

mod attr;
mod derive;
mod func;

use manyhow::{Emitter, manyhow};
use proc_macro2::TokenStream;
use syn::DeriveInput;

/// Derive getter methods for a named struct.
#[manyhow(proc_macro_derive(Getter, attributes(getter)))]
pub fn derive_getter(input: DeriveInput, emitter: &mut Emitter) -> manyhow::Result<TokenStream> {
    derive::derive_getter(input, emitter)
}

/// Measure the execution time of a synchronous function.
#[manyhow(proc_macro_attribute)]
pub fn duration(
    attr: TokenStream,
    item: TokenStream,
    emitter: &mut Emitter,
) -> manyhow::Result<TokenStream> {
    attr::duration(attr, item, emitter)
}

/// Construct a `HashMap` from `key => value` pairs.
#[manyhow(proc_macro)]
pub fn hashmap(input: TokenStream) -> manyhow::Result<TokenStream> {
    func::hashmap(input)
}

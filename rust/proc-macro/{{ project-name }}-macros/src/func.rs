//! `hashmap!` function-like procedural macro.
//!
//! Usage:
//! ```ignore
//! let map: HashMap<String, i32> = hashmap! {
//!     "a" => 1,
//!     "b" => 2,
//! };
//! ```
//!
//! Parsing is implemented with syn's `Parse` trait (darling only handles
//! attributes and derives, so function-like macros parse their input
//! directly). `manyhow::bail!` produces a precisely spanned compile error.
//!
//! Note: a function-like macro expands in expression (or pattern) position,
//! where several `compile_error!` tokens in a row fail to parse and cause
//! cascading diagnostics. Multi-error collection with `Emitter` therefore
//! fits derive/attribute macros (whose output is items), while this macro
//! bails on the first semantic problem.

use std::collections::HashSet;

use manyhow::bail;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, ExprLit, Lit, Token};

struct KeyValue {
    key: Expr,
    _arrow: Token![=>],
    value: Expr,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _arrow: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct HashMapInput {
    pairs: Punctuated<KeyValue, Token![,]>,
}

impl Parse for HashMapInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pairs: input.parse_terminated(KeyValue::parse, Token![,])?,
        })
    }
}

/// Normalize literal keys that have an obvious value equality so duplicate
/// keys can be detected. Non-literal keys are skipped (const evaluation is
/// out of scope).
fn literal_key(expr: &Expr) -> Option<String> {
    let Expr::Lit(ExprLit { lit, .. }) = expr else {
        return None;
    };
    match lit {
        Lit::Str(s) => Some(format!("str:{}", s.value())),
        Lit::Int(i) => Some(format!("int:{}", i.base10_digits())),
        Lit::Bool(b) => Some(format!("bool:{}", b.value)),
        _ => None,
    }
}

pub fn hashmap(input: TokenStream) -> manyhow::Result<TokenStream> {
    let HashMapInput { pairs } = syn::parse2::<HashMapInput>(input)?;

    let mut known_keys = HashSet::new();
    let mut inserts = Vec::new();
    for pair in &pairs {
        if let Some(normalized) = literal_key(&pair.key)
            && !known_keys.insert(normalized)
        {
            bail!(&pair.key, "duplicate key in `hashmap!`");
        }

        let key = &pair.key;
        let value = &pair.value;
        inserts.push(quote! {
            __map.insert(#key.into(), #value);
        });
    }

    // Build inside an immediately-invoked closure so the macro expands to
    // an expression in every position, while keeping inserted locals scoped.
    Ok(quote! {
        (|| {
            let mut __map = ::std::collections::HashMap::new();
            #(#inserts)*
            __map
        })()
    })
}

//! `#[derive(Getter)]` — generate getter methods for named structs.
//!
//! Attributes are parsed with darling:
//! - container level: `#[getter(prefix = "get_")]`
//! - field level: `#[getter(skip)]`, `#[getter(copy)]`, `#[getter(name = "x")]`
//!
//! darling handles attribute-shape errors; the cross-field semantic check
//! (duplicate generated method names) feeds a manyhow `Emitter` so that every
//! colliding field is reported in one compilation instead of one at a time.

use std::collections::HashMap;

use darling::{FromDeriveInput, FromField, ast::Data};
use manyhow::{Emitter, error_message};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Ident, Type};

#[derive(FromDeriveInput)]
#[darling(attributes(getter), supports(struct_named))]
struct StructArgs {
    ident: Ident,
    data: Data<(), FieldArgs>,
    /// Prefix prepended to every generated method name.
    #[darling(default)]
    prefix: Option<String>,
}

#[derive(FromField)]
#[darling(attributes(getter))]
struct FieldArgs {
    ident: Option<Ident>,
    ty: Type,
    /// Do not generate a getter for this field.
    #[darling(default)]
    skip: bool,
    /// Return the value by copy instead of by reference.
    #[darling(default, rename = "copy")]
    r#copy: bool,
    /// Override the generated method name.
    #[darling(default)]
    name: Option<Ident>,
}

struct GeneratedMethod {
    field: Ident,
    method: Ident,
}

pub fn derive_getter(input: DeriveInput, emitter: &mut Emitter) -> manyhow::Result<TokenStream> {
    let args = StructArgs::from_derive_input(&input)?;

    let fields = args
        .data
        .take_struct()
        .expect("darling `supports(struct_named)` guarantees a named struct");

    let ident = &args.ident;
    let mut generated = Vec::new();
    let mut methods = Vec::new();

    for field in fields.fields {
        if field.skip {
            continue;
        }

        let field_ident = field
            .ident
            .clone()
            .expect("darling `supports(struct_named)` guarantees named fields");

        let method_ident = match (&field.name, &args.prefix) {
            (Some(name), _) => name.clone(),
            (None, Some(prefix)) => format_ident!("{prefix}{field_ident}"),
            (None, None) => field_ident.clone(),
        };

        let ty = &field.ty;
        let (return_ty, body) = if field.r#copy {
            (quote!(#ty), quote!(self.#field_ident))
        } else {
            (quote!(&#ty), quote!(&self.#field_ident))
        };

        methods.push(quote! {
            #[doc = concat!("Returns the value of field `", stringify!(#field_ident), "`.")]
            pub fn #method_ident(&self) -> #return_ty {
                #body
            }
        });

        generated.push(GeneratedMethod {
            field: field_ident,
            method: method_ident,
        });
    }

    // Report every field participating in a name collision at once.
    let mut first_owner: HashMap<String, &Ident> = HashMap::new();
    for gm in &generated {
        let key = gm.method.to_string();
        if let Some(first) = first_owner.get(&key) {
            emitter.emit(error_message!(
                gm.field,
                "getter method `{}` is already generated for field `{}`",
                gm.method,
                first
            ));
        } else {
            first_owner.insert(key, &gm.field);
        }
    }
    emitter.into_result()?;

    Ok(quote! {
        impl #ident {
            #(#methods)*
        }
    })
}

//! `#[duration]` attribute macro — times a synchronous function.
//!
//! Usage:
//! - `#[duration]` — print elapsed milliseconds
//! - `#[duration(us)]` — microseconds
//! - `#[duration(ms)]` — milliseconds (default)
//! - `#[duration(sec)]` — seconds
//!
//! Attribute arguments are parsed with darling. Validation errors are
//! collected in a manyhow `Emitter`, so an `async fn` with conflicting units
//! reports both problems in a single compilation instead of one at a time.

use darling::{FromMeta, ast::NestedMeta};
use manyhow::{Emitter, error_message};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

#[derive(FromMeta, Default)]
struct DurationArgs {
    #[darling(default)]
    us: bool,
    #[darling(default)]
    ms: bool,
    #[darling(default)]
    sec: bool,
}

pub fn duration(
    attr: TokenStream,
    item: TokenStream,
    emitter: &mut Emitter,
) -> manyhow::Result<TokenStream> {
    let nested = NestedMeta::parse_meta_list(attr)?;
    let args = DurationArgs::from_list(&nested)?;
    let func = syn::parse2::<ItemFn>(item)?;

    let unit_count = usize::from(args.us) + usize::from(args.ms) + usize::from(args.sec);
    if unit_count > 1 {
        emitter.emit(error_message!(
            "only one time unit may be specified: `us`, `ms`, or `sec`"
        ));
    }

    if func.sig.asyncness.is_some() {
        emitter.emit(error_message!(
            &func.sig.ident,
            "`#[duration]` does not support `async fn`"
        ));
    }

    // Bail out with all collected diagnostics before generating code.
    emitter.into_result()?;

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
        ..
    } = func;
    let fn_name = &sig.ident;

    let report = if args.us {
        quote! {
            ::std::eprintln!("`{}` took {} us", stringify!(#fn_name), __start.elapsed().as_micros());
        }
    } else if args.sec {
        quote! {
            ::std::eprintln!("`{}` took {:.3} s", stringify!(#fn_name), __start.elapsed().as_secs_f64());
        }
    } else {
        quote! {
            ::std::eprintln!("`{}` took {} ms", stringify!(#fn_name), __start.elapsed().as_millis());
        }
    };

    Ok(quote! {
        #(#attrs)*
        #vis #sig {
            let __start = ::std::time::Instant::now();
            let __result = #block;
            #report
            __result
        }
    })
}

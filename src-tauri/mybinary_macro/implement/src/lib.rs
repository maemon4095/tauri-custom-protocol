use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;

fn make_compile_error(str: &str) -> TokenStream {
    quote!(compile_error!(#str))
}

pub fn command(attr: TokenStream, body: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return make_compile_error("unexpected attribute input.");
    }

    let item_fn: syn::ItemFn = match syn::parse2(body) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    let ident = quote::format_ident!("__mybinary_ipc_abi_{}", &item_fn.sig.ident);
    let vis = &item_fn.vis;
    let ret = &item_fn.sig.output;
    let arg_tys = item_fn.sig.inputs.iter().map(|e| match e {
        syn::FnArg::Receiver(_) => unreachable!(),
        syn::FnArg::Typed(arg) => &arg.ty,
    });

    let arg_ids = item_fn
        .sig
        .inputs
        .iter()
        .enumerate()
        .map(|(i, _)| quote::format_ident!("arg{}", i));

    quote! {
        #vis fn #ident (bin: &[u8]) -> ::bincode::Result<#ret> {
            let args = ::bincode::deserialize(bin)?;


        }
    };

    todo!()
}

struct A {
    a: usize,
    b: usize,
}

fn a(A { a, b }: A) {}

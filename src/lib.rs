//! This crate allows you to destructure structs that implement `Drop`.
//! 
//! If you've ever struggled with error E0509
//! "cannot move out of type `T`, which implements the `Drop` trait"
//! then this crate may be for you.
//! 
//! Simply mark your struct with `#[derive(destructure)]`.
//! That gives it a `fn destructure`, which takes `self` by value and
//! turns that `self` into a tuple of the fields of the struct,
//! **without running the struct's `drop()` method**, like this:
//! 
//! ```ignore
//! let (field_1, field_2, ...) = my_struct.destructure();
//! ```
//! 
//! For structs that don't implement `Drop` you don't really need this crate,
//! because Rust lets you destructure those kinds of structs very easily already.
//! You can simply move their fields out of them.
//! But for structs that do implement `Drop`, you can't simply move values out,
//! so there's no easy way to destructure them. That's why I made this crate.
//! 
//! Note: a tuple of 1 element in Rust is denoted as `(x,)`, not `(x)`.
//! 
//! # Example:
//! ```
//! #[macro_use]
//! extern crate derive_destructure;
//! 
//! #[derive(destructure)]
//! struct ImplementsDrop {
//!     some_str: String,
//!     some_int: i32
//! }
//! 
//! impl Drop for ImplementsDrop {
//!     fn drop(&mut self) {
//!         panic!("We don't want to drop this");
//!     }
//! }
//! 
//! fn main() {
//!     let x = ImplementsDrop {
//!         some_str: "foo".to_owned(),
//!         some_int: 4
//!     };
//!     let (some_str, some_int) = x.destructure();
//!     // x won't get dropped now
//! }
//! ```

// The `quote!` macro requires deep recursion.
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Index};

#[proc_macro_derive(destructure)]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let field_types = quote_field_types(&input.data);
    let field_reads = quote_field_reads(&input.data);

    let output = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            fn destructure(self) -> #field_types {
                let maybe_uninit = ::std::mem::MaybeUninit::new(self);
                unsafe {
                    let self_ref = &*maybe_uninit.as_ptr();
                    #field_reads
                }
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

fn quote_field_types(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            #ty
                        }
                    });
                    quote! {
                        (#(#recurse,)*)
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().map(|f| {
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            #ty
                        }
                    });
                    quote! {
                        (#(#recurse,)*)
                    }
                }
                Fields::Unit => {
                    quote!(())
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn quote_field_reads(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let ident = &f.ident;
                        quote_spanned! {f.span()=>
                            std::ptr::read(&self_ref.#ident)
                        }
                    });
                    quote! {
                        (#(#recurse,)*)
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i,f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            std::ptr::read(&self_ref.#index)
                        }
                    });
                    quote! {
                        (#(#recurse,)*)
                    }
                }
                Fields::Unit => {
                    quote!(())
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

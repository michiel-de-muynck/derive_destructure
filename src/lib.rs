//! This crate allows you to destructure structs that implement `Drop`.
//! 
//! If you've ever struggled with error E0509
//! "cannot move out of type `T`, which implements the `Drop` trait"
//! then this crate may be for you.
//! 
//! To use this crate, put this in your `lib.rs` or `main.rs`:
//! ```ignore
//! #[macro_use]
//! extern crate derive_destructure;
//! ```
//! 
//! Then you have 2 ways to use this crate:
//! 
//! # Option 1: `#[derive(destructure)]`
//! 
//! If you mark a struct with `#[derive(destructure)]`, then you can destructure it using
//! ```ignore
//! let (field_1, field_2, ...) = my_struct.destructure();
//! ```
//! 
//! This turns the struct into a tuple of its fields **without running the struct's `drop()`
//! method**. You can then happily move elements out of this tuple.
//! 
//! Note: in Rust, a tuple of 1 element is denoted as `(x,)`, not `(x)`.
//! 
//! # Option 2: `#[derive(remove_trait_impls)]`
//! 
//! If you mark your struct with `#[derive(remove_trait_impls)]`, then you can do
//! ```ignore
//! let my_struct = my_struct.remove_trait_impls();
//! ```
//! 
//! The result is a struct with the same fields, but it implements no traits
//! (except automatically-implemented traits like `Sync` and `Send`).
//! In particular, it doesn't implement `Drop`, so you can move fields out of it.
//! 
//! The name of the resulting struct is the original name plus the suffix `WithoutTraitImpls`.
//! For example, `Foo` becomes `FooWithoutTraitImpls`. But you usually don't need to write
//! out this name.
//! 
//! `#[derive(remove_trait_impls)]` works on enums too.
//! 
//! # Example:
//! ```
//! #[macro_use]
//! extern crate derive_destructure;
//! 
//! #[derive(destructure, remove_trait_impls)]
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
//!     // Using destructure():
//!     let x = ImplementsDrop {
//!         some_str: "foo".to_owned(),
//!         some_int: 4
//!     };
//!     let (some_str, some_int) = x.destructure();
//!     // x's drop() method never gets called
//! 
//!     // Using remove_trait_impls():
//!     let x = ImplementsDrop {
//!         some_str: "foo".to_owned(),
//!         some_int: 4
//!     };
//!     let x = x.remove_trait_impls();
//!     // this x doesn't implement drop,
//!     // so we can move fields out of it
//!     drop(x.some_str);
//!     println!("{}", x.some_int);
//! }
//! ```

// The `quote!` macro requires deep recursion.
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Index};

#[proc_macro_derive(destructure)]
pub fn derive_destructure(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let field_types = fields.named.iter().map(|f| {
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            #ty
                        }
                    });
                    let field_reads = fields.named.iter().map(|f| {
                        let ident = &f.ident;
                        quote_spanned! {f.span()=>
                            ::std::ptr::read(&self_ref.#ident)
                        }
                    });
                    quote! {
                        impl #impl_generics #name #ty_generics #where_clause {
                            #[inline(always)]
                            fn destructure(self) -> (#(#field_types,)*) {
                                let maybe_uninit = ::std::mem::MaybeUninit::new(self);
                                unsafe {
                                    let self_ref = &*maybe_uninit.as_ptr();
                                    (#(#field_reads,)*)
                                }
                            }
                        }
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let field_types = fields.unnamed.iter().map(|f| {
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            #ty
                        }
                    });
                    let field_reads = fields.unnamed.iter().enumerate().map(|(i,f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            ::std::ptr::read(&self_ref.#index)
                        }
                    });
                    quote! {
                        impl #impl_generics #name #ty_generics #where_clause {
                            #[inline(always)]
                            fn destructure(self) -> (#(#field_types,)*) {
                                let maybe_uninit = ::std::mem::MaybeUninit::new(self);
                                unsafe {
                                    let self_ref = &*maybe_uninit.as_ptr();
                                    (#(#field_reads,)*)
                                }
                            }
                        }
                    }
                }
                Fields::Unit => {
                    quote! {
                        impl #impl_generics #name #ty_generics #where_clause {
                            #[inline(always)]
                            fn destructure(self) {
                                let _ = ::std::mem::MaybeUninit::new(self);
                            }
                        }
                    }
                }
            }
        }
        Data::Enum(_) => panic!("#[derive(destructure)] doesn't work on enums, use #[derive(remove_trait_impls)] instead."),
        Data::Union(_) => panic!("#[derive(destructure)] doesn't work on unions."),
    };

    proc_macro::TokenStream::from(output)
}

#[proc_macro_derive(remove_trait_impls)]
pub fn derive_remove_trait_impls(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let new_type_name = Ident::new(&(name.to_string()+"WithoutTraitImpls"), Span::call_site());

    let output = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let fields_iter = fields.named.iter().map(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            #ident: #ty
                        }
                    });
                    let field_reads_iter = fields.named.iter().map(|f| {
                        let ident = &f.ident;
                        quote_spanned! {f.span()=>
                            #ident: ::std::ptr::read(&self_ref.#ident)
                        }
                    });
                    quote! {
                        struct #new_type_name #ty_generics #where_clause {
                            #(#fields_iter,)*
                        }

                        impl #impl_generics #name #ty_generics #where_clause {
                            #[inline(always)]
                            fn remove_trait_impls(self) -> #new_type_name #ty_generics {
                                let maybe_uninit = ::std::mem::MaybeUninit::new(self);
                                unsafe {
                                    let self_ref = &*maybe_uninit.as_ptr();
                                    #new_type_name {
                                        #(#field_reads_iter,)*
                                    }
                                }
                            }
                        }
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let fields_iter = fields.unnamed.iter().map(|f| {
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            #ty
                        }
                    });
                    let field_reads_iter = fields.unnamed.iter().enumerate().map(|(i,f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            ::std::ptr::read(&self_ref.#index)
                        }
                    });
                    quote! {
                        struct #new_type_name #ty_generics #where_clause (#(#fields_iter,)*);

                        impl #impl_generics #name #ty_generics #where_clause {
                            #[inline(always)]
                            fn remove_trait_impls(self) -> #new_type_name #ty_generics {
                                let maybe_uninit = ::std::mem::MaybeUninit::new(self);
                                unsafe {
                                    let self_ref = &*maybe_uninit.as_ptr();
                                    #new_type_name(#(#field_reads_iter,)*)
                                }
                            }
                        }
                    }
                }
                Fields::Unit => {
                    quote! {
                        struct #new_type_name #ty_generics #where_clause;

                        impl #impl_generics #name #ty_generics #where_clause {
                            #[inline(always)]
                            fn remove_trait_impls(self) -> #new_type_name #ty_generics {
                                let _ = ::std::mem::MaybeUninit::new(self);
                                #new_type_name
                            }
                        }
                    }
                }
            }
        }
        Data::Enum(ref data) => {
            let variants_iter = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                match variant.fields {
                    Fields::Named(ref fields) => {
                        let fields_iter = fields.named.iter().map(|f| {
                            let ident = &f.ident;
                            let ty = &f.ty;
                            quote_spanned! {f.span()=>
                                #ident: #ty
                            }
                        });
                        quote! {
                            #variant_ident {
                                #(#fields_iter,)*
                            }
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        let fields_iter = fields.unnamed.iter().map(|f| {
                            let ty = &f.ty;
                            quote_spanned! {f.span()=>
                                #ty
                            }
                        });
                        quote! {
                            #variant_ident(#(#fields_iter,)*)
                        }
                    }
                    Fields::Unit => {
                        quote!(#variant_ident)
                    }
                }
            });
            let match_arms_iter = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                match variant.fields {
                    Fields::Named(ref fields) => {
                        let fields_iter = fields.named.iter().map(|f| {
                            let ident = &f.ident;
                            quote_spanned! {f.span()=>
                                ref #ident
                            }
                        });
                        let field_reads_iter = fields.named.iter().map(|f| {
                            let ident = &f.ident;
                            quote_spanned! {f.span()=>
                                #ident: ::std::ptr::read(#ident)
                            }
                        });
                        quote! {
                            #name::#variant_ident { #(#fields_iter,)* } => #new_type_name::#variant_ident { #(#field_reads_iter,)* }
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        let fields_iter = fields.unnamed.iter().enumerate().map(|(i,f)| {
                            let index = Ident::new(&format!("__{}", i), f.span());
                            quote_spanned! {f.span()=>
                                ref #index
                            }
                        });
                        let field_reads_iter = fields.unnamed.iter().enumerate().map(|(i,f)| {
                            let index = Ident::new(&format!("__{}", i), f.span());
                            quote_spanned! {f.span()=>
                                ::std::ptr::read(#index)
                            }
                        });
                        quote! {
                            #name::#variant_ident(#(#fields_iter,)*) => #new_type_name::#variant_ident(#(#field_reads_iter,)*)
                        }
                    }
                    Fields::Unit => {
                        quote!{
                            #name::#variant_ident => #new_type_name::#variant_ident
                        }
                    }
                }
            });
            quote! {
                enum #new_type_name #ty_generics #where_clause {
                    #(#variants_iter,)*
                }

                impl #impl_generics #name #ty_generics #where_clause {
                    #[inline(always)]
                    fn remove_trait_impls(self) -> #new_type_name #ty_generics {
                        let maybe_uninit = ::std::mem::MaybeUninit::new(self);
                        unsafe {
                            match &*maybe_uninit.as_ptr() {
                                #(#match_arms_iter,)*
                            }
                        }
                    }
                }
            }
        }
        Data::Union(_) => panic!("#[derive(remove_trait_impls)] doesn't work on unions."),
    };

    proc_macro::TokenStream::from(output)
}

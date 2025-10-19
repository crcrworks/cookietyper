extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Lit, Meta, parse_macro_input};

#[proc_macro_derive(Facility, attributes(facility))]
pub fn facility_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_facility_macro(&input)
}

fn impl_facility_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut base_cost = quote! { 15u32 };
    let mut base_cps = quote! { 0.1 };
    let mut key = quote! { crate::facilities::FacilityKey::Cursor };

    for attr in &ast.attrs {
        if attr.path().is_ident("facility")
            && let Meta::List(meta_list) = &attr.meta
        {
            let nested = meta_list.tokens.clone();
            let parser = syn::meta::parser(|meta| {
                if meta.path.is_ident("base_cost") {
                    if let Some(value) = parse_lit_int(&meta)? {
                        base_cost = quote! { #value };
                    }
                } else if meta.path.is_ident("base_cps") {
                    if let Some(value) = parse_lit_float(&meta)? {
                        base_cps = quote! { #value };
                    }
                } else if meta.path.is_ident("key")
                    && let Some(value) = parse_lit_str(&meta)?
                {
                    let key_ident = format_ident!("{}", value);
                    key = quote! { crate::facilities::FacilityKey::#key_ident };
                }

                Ok(())
            });
            syn::parse::Parser::parse2(parser, nested).ok();
        }
    }

    let g = quote! {
        impl crate::facilities::FacilityProperties for #name {
            fn key() -> crate::facilities::FacilityKey {
                #key
            }

            fn amount(&self) -> u32 {
                self.amount
            }

            fn multiplier(&self) -> f64 {
                self.multiplier
            }

            fn base_cost(&self) -> U512 {
                #base_cost.into()
            }

            fn base_cps(&self) -> f64 {
                #base_cps
            }
        }
    };
    g.into()
}

fn parse_lit_int(meta: &syn::meta::ParseNestedMeta) -> syn::Result<Option<u32>> {
    let value = meta.value()?;
    let lit: Lit = value.parse()?;
    if let Lit::Int(lit_int) = lit {
        Ok(Some(lit_int.base10_parse()?))
    } else {
        Ok(None)
    }
}

fn parse_lit_float(meta: &syn::meta::ParseNestedMeta) -> syn::Result<Option<f64>> {
    let value = meta.value()?;
    let lit: Lit = value.parse()?;
    match lit {
        Lit::Float(lit_float) => Ok(Some(lit_float.base10_parse()?)),
        Lit::Int(lit_int) => Ok(Some(lit_int.base10_parse::<i64>()? as f64)),
        _ => Ok(None),
    }
}

fn parse_lit_str(meta: &syn::meta::ParseNestedMeta) -> syn::Result<Option<String>> {
    let value = meta.value()?;
    let lit: Lit = value.parse()?;
    if let Lit::Str(lit_str) = lit {
        Ok(Some(lit_str.value()))
    } else {
        Ok(None)
    }
}

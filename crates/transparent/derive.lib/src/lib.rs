extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Transparent)]
pub fn derive_transparent(item: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(item as DeriveInput);
    match generate_getter(input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_getter(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let field = {
        let err = Err(syn::Error::new_spanned(
            &derive_input.ident,
            "Must be single field tuple struct type",
        ));
        match &derive_input.data {
            syn::Data::Struct(v) => match &v.fields {
                syn::Fields::Unnamed(fields) => {
                    if fields.unnamed.len() == 1 {
                        &fields.unnamed[0]
                    } else {
                        // TODO: support single field named struct
                        return err;
                    }
                }
                _ => return err,
            },
            _ => return err,
        }
    };

    let struct_name = &derive_input.ident;
    let inner_ty = &field.ty;
    let (impl_generics, type_generics, where_clause) = &derive_input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Transparent for #struct_name #type_generics #where_clause {
            type Inner = #inner_ty;
            fn into_inner(self) -> Self::Inner {
                self.0
            }
            fn from_inner(inner: Self::Inner) -> Self {
                Self(inner)
            }
            fn inner(&self) -> &Self::Inner {
                &self.0
            }
            fn inner_mut(&mut self) -> &mut Self::Inner {
                &mut self.0
            }
        }
        impl #impl_generics ::std::convert::From<#inner_ty> for #struct_name #type_generics #where_clause {
            fn from(inner: #inner_ty) -> Self {
                Self(inner)
            }
        }
    };

    Ok(expanded.into())
}

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use proc_quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::{Data, DataStruct, Fields, Ident, Type};

/// the macro derives `anim::Animatable` for you automatically.
#[proc_macro_derive(Animatable, attributes(tag))]
pub fn animatable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand_derive(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let anim = get_crate()?;
    let fields = get_fields(input.data)
        .unwrap()
        .iter()
        .map(|(field_name, _)| {
            Ok(quote! {
                res.#field_name = #anim::Animatable::animate(&self.#field_name,&to.#field_name, time);
            })
        })
        .collect::<syn::Result<proc_macro2::TokenStream>>()?;
    let st_name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl  #impl_generics #anim::Animatable for #st_name #ty_generics #where_clause
         {
            #[inline]
            fn animate(&self, to: &Self, time: f64) -> Self{
                let mut res = self.clone();
                #fields
                res
            }
        }
    })
}

fn get_crate() -> syn::Result<Ident> {
    let anim = match crate_name("anim") {
        Ok(found) => match found {
            FoundCrate::Itself => Ident::new("crate", Span::call_site()),
            FoundCrate::Name(name) => Ident::new(&name, Span::call_site()),
        },
        Err(_) => Ident::new("crate", Span::call_site()),
    };
    Ok(anim)
}

fn get_fields(data: Data) -> syn::Result<Vec<(Ident, Type)>> {
    let fields = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };
    let items = fields
        .into_iter()
        .map(|f| {
            let field_name = f.ident.unwrap();
            let ty = f.ty;
            (field_name, ty)
        })
        .collect();

    Ok(items)
}

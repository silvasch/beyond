use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

#[derive(Debug)]
pub struct Route {
    name: Ident,
    request: Ident,
    response: Ident,
}

impl Route {
    pub fn to_clientside_method_tokens(&self) -> TokenStream {
        let name = &self.name;
        let request = &self.request;
        let response = &self.response;

        quote! {
            pub fn #name(&self, request: #request) -> #response {
                todo!("implement clientside logic")
            }
        }
    }
}

impl syn::parse::Parse for Route {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let request = input.parse()?;
        let response = input.parse()?;

        Ok(Self {
            name,
            request,
            response,
        })
    }
}

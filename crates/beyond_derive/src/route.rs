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
            pub fn #name(&self, request: #request) -> ::core::result::Result<#response, ::beyond::Error> {
                let encoded_request = ::beyond::serde::encode_request(request)?;

                let output = ::std::process::Command::new("ssh")
                    .args([
                        &self.destination,
                        &self.server_binary,
                        "beyond-server-process",
                        stringify!(#name),
                        &encoded_request,
                    ])
                    .output()
                    .map_err(::beyond::Error::SSHProcessLaunch)?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    return ::core::result::Result::Err(::beyond::Error::SSHProcessExecute { stderr });
                }

                let encoded_response = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let response = ::beyond::serde::decode_response(&encoded_response)?;

                Ok(response)
            }
        }
    }

    pub fn to_serverside_impl_tokens(&self, server_impl: &Ident) -> TokenStream {
        let name = &self.name;
        let request = &self.request;
        let response = &self.response;

        let ident = quote::format_ident!("{}_serverside_impl", name);

        quote! {
            #[doc(hidden)]
            fn #ident(server_impl: &mut #server_impl, encoded_request: String) -> ::core::result::Result<String, ::beyond::Error> {
                let request: #request = ::beyond::serde::decode_request(&encoded_request)?;
                let response: #response = server_impl.#name(request);
                let encoded_response = ::beyond::serde::encode_response(response)?;
                ::core::result::Result::Ok(encoded_response)
            }
        }
    }

    pub fn to_serverside_routing_tokens(&self) -> TokenStream {
        let name = &self.name;

        let ident = quote::format_ident!("{}_serverside_impl", name);

        quote! {
            stringify!(#name) => #ident(&mut server_impl, encoded_request),
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

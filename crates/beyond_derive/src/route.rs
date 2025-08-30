use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// The syntax items needed to generate functions for the server and client.
#[derive(Debug)]
pub struct Route {
    /// The name of the route.
    name: Ident,
    /// The type of the request.
    request: Ident,
    /// The type of the response.
    response: Ident,
}

impl Route {
    /// Generate the function that will be called on the client.
    pub fn to_clientside_method_tokens(&self) -> TokenStream {
        let name = &self.name;
        let request = &self.request;
        let response = &self.response;

        quote! {
            pub fn #name(&self, request: #request) -> ::core::result::Result<#response, ::beyond::Error> {
                // Prepare the request to be used as a command-line argument.
                let encoded_request = ::beyond::serde::encode_request(request)?;

                // Execute the server binary over SSH.
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

                // Check if the execution succeeded and handle the failure case.
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    return ::core::result::Result::Err(::beyond::Error::SSHProcessExecute { stderr });
                }

                // Extract the response and decode it.
                let encoded_response = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let response = ::beyond::serde::decode_response(&encoded_response)?;

                Ok(response)
            }
        }
    }

    // Generate the server-side wrapper function around the user logic.
    pub fn to_serverside_wrapper_tokens(&self, server_ident: &Ident) -> TokenStream {
        let name = &self.name;
        let request = &self.request;
        let response = &self.response;

        let ident = quote::format_ident!("{}_wrapper", name);

        quote! {
            #[doc(hidden)]
            fn #ident(server: &mut #server_ident, encoded_request: String) -> ::core::result::Result<String, ::beyond::Error> {
                let request: #request = ::beyond::serde::decode_request(&encoded_request)?;
                let response: #response = server.#name(request);
                let encoded_response = ::beyond::serde::encode_response(response)?;
                ::core::result::Result::Ok(encoded_response)
            }
        }
    }

    // Generate the match arm used for routing.
    pub fn to_serverside_routing_tokens(&self) -> TokenStream {
        let name = &self.name;

        let ident = quote::format_ident!("{}_wrapper", name);

        quote! {
            stringify!(#name) => Self::#ident(&mut server, encoded_request),
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

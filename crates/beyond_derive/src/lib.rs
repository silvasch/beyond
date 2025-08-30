use proc_macro2::TokenTree;
use syn::{DeriveInput, Ident, Meta};

struct Route {
    name: Ident,
    request: Ident,
    response: Ident,
}

#[proc_macro_derive(Beyond, attributes(beyond_route))]
pub fn beyond_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<DeriveInput>(input).unwrap();

    let internal_item = &input.ident;

    let mut output = quote::quote! {};

    let mut run_server_match_arms = quote::quote! {};

    for attribute in &input.attrs {
        let ident = match attribute.path().get_ident() {
            Some(ident) => ident,
            None => continue, // This means the attribute looks like `#[hello::world]`, which we are not interested in
        };

        match ident.to_string().as_str() {
            "beyond_route" => {
                let route = match parse_route(&attribute.meta) {
                    Some(route) => route,
                    None => panic!("routes are expected to be in the form of `#[beyond_route(route_name, Request, Response)]`"),
                };

                let route_name = route.name;
                let request = route.request;
                let response = route.response;

                output.extend(quote::quote! {
                    impl Beyond {
                        pub fn #route_name(&self, request: #request) -> #response {
                            let encoded_request = ::base64::Engine::encode(
                                &::base64::prelude::BASE64_STANDARD,
                                ::serde_json::to_string(&request).unwrap(),
                            );

                            let output = ::std::process::Command::new("ssh").args([
                                &self.destination,
                                &self.server_binary,
                                stringify!(#route_name),
                                &encoded_request,
                            ]).output().unwrap();

                            if !output.status.success() {
                                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                                eprintln!("{}", stderr);
                                todo!("handle failure")
                            }

                            let encoded_response = String::from_utf8_lossy(&output.stdout).to_string();
                            println!("{}", encoded_response);

                            todo!("decode response")
                        }
                    }
                });

                run_server_match_arms.extend(quote::quote!{
                    stringify!(#route_name) => {
                        let request = serde_json::from_str(&encoded_request).unwrap();
                        let response = internal.#route_name(request);
                        let encoded_response = ::base64::Engine::encode(
                            &::base64::prelude::BASE64_STANDARD,
                            ::serde_json::to_string(&response).unwrap(),
                        );
                        encoded_response
                    }
                });
            }
            _ => continue,
        }
    }

    output.extend(quote::quote! {
        pub struct Beyond {
            destination: String,
            server_binary: String,
            internal: Option<#internal_item>,
        }

        impl Beyond {
            pub fn new_client(destination: String, server_binary: String) -> Self {
                Self {
                    destination,
                    server_binary,
                    internal: None,
                }
            }

            pub fn run_server(mut internal: #internal_item) {
                let route_name = ::std::env::args().nth(1).unwrap_or_default();
                let encoded_request = ::std::env::args().nth(2).unwrap_or_default();
                let encoded_request = ::base64::Engine::decode(
                    &::base64::prelude::BASE64_STANDARD,
                    encoded_request,
                ).unwrap();
                let encoded_request = String::from_utf8_lossy(&encoded_request).to_string();

                let encoded_response = match route_name.as_str() {
                    #run_server_match_arms
                    _ => todo!(),
                };

                println!("{}", encoded_response);
            }
        }
    });


    output.into()
}

fn parse_route(meta: &Meta) -> Option<Route> {
    let mut tokens = match meta {
        Meta::List(meta_list) => meta_list.tokens.clone(),
        _ => return None,
    }
    .into_iter();

    let route_name = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident,
        _ => return None,
    };

    let request = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident,
        _ => return None,
    };

    let response = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident,
        _ => return None,
    };

    Some(Route {
        name: route_name,
        request,
        response,
    })
}

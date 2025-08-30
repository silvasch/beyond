mod route;
use quote::quote;
pub(crate) use route::Route;

#[proc_macro_derive(Beyond, attributes(beyond_route))]
pub fn beyond_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    beyond_derive_impl(input).unwrap()
}

fn beyond_derive_impl(input: proc_macro::TokenStream) -> syn::Result<proc_macro::TokenStream> {
    let input = syn::parse::<syn::DeriveInput>(input)?;

    let server_impl = input.ident;

    let mut output = proc_macro2::TokenStream::new();

    let mut serverside_impls = proc_macro2::TokenStream::new();
    let mut serverside_routing = proc_macro2::TokenStream::new();

    for attribute in input.attrs {
        let ident = match attribute.path().get_ident() {
            Some(ident) => ident,
            _ => continue,
        };

        match ident.to_string().as_str() {
            "beyond_route" => {
                let meta_list = attribute.meta.require_list()?;
                let route: Route = meta_list.parse_args()?;

                let clientside_method_tokens = route.to_clientside_method_tokens();
                output.extend(quote! {
                    impl Beyond {
                        #clientside_method_tokens
                    }
                });

                let serverside_impl_tokens = route.to_serverside_impl_tokens(&server_impl);
                serverside_impls.extend(serverside_impl_tokens);

                let serverside_routing_tokens = route.to_serverside_routing_tokens();
                serverside_routing.extend(serverside_routing_tokens);
            }
            _ => continue,
        }
    }

    output.extend(quote::quote! {
        pub struct Beyond {
            destination: String,
            server_binary: String,
        }

        impl Beyond {
            pub fn new_client(destination: String, server_binary: String) -> Self {
                Self { destination, server_binary }
            }

            pub fn run_server(mut server_impl: #server_impl) -> ::core::option::Option<::std::process::ExitCode> {
                if ::std::env::args().nth(1).unwrap_or_default() != "beyond-server-process" {
                    return ::core::option::Option::None;
                }

                let route_name = ::std::env::args().nth(2).unwrap_or_default();
                let encoded_request = ::std::env::args().nth(3).unwrap_or_default();

                #serverside_impls

                let encoded_response_result = match route_name.as_str() {
                    #serverside_routing
                    _ => ::core::result::Result::Err(::beyond::Error::InvalidRoute { route_name }),
                };

                match encoded_response_result {
                    Ok(encoded_response) => {
                        println!("{}", encoded_response);
                        return Some(::std::process::ExitCode::SUCCESS);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        return Some(std::process::ExitCode::FAILURE);
                    }
                }
            }
        }
    });

    Ok(output.into())
}

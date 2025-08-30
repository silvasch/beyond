mod route;
use quote::quote;
pub(crate) use route::Route;

#[proc_macro_derive(Beyond, attributes(beyond_route))]
pub fn beyond_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    beyond_derive_impl(input).unwrap()
}

fn beyond_derive_impl(input: proc_macro::TokenStream) -> syn::Result<proc_macro::TokenStream> {
    let input = syn::parse::<syn::DeriveInput>(input)?;

    let internal = input.ident;

    let mut output = proc_macro2::TokenStream::new();

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
            },
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

            pub fn run_server(internal: #internal) -> ::std::process::ExitCode {
                todo!("implement serverside logic")
            }
        }
    });

    Ok(output.into())
}

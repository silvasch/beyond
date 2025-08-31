mod route;
use quote::quote;
pub(crate) use route::Route;

/// Generate client and server code to execute functions remotely.
#[proc_macro_derive(Beyond, attributes(beyond_route))]
pub fn beyond_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    beyond_derive_impl(input).unwrap()
}

// The macro's logic is wrapped in a new function that returns a `Result` to
// make error handling easier using the `?` operator.
fn beyond_derive_impl(input: proc_macro::TokenStream) -> syn::Result<proc_macro::TokenStream> {
    let input = syn::parse::<syn::DeriveInput>(input)?;

    // The name of the item the proc macro was used on.
    let server_ident = input.ident;

    let mut output = proc_macro2::TokenStream::new();

    // This will contain wrappers around the server-side functions.
    // They will return `Result<String, String>`'s, which is required
    // because they cannot use the un-encoded structs, as this would
    // clash with the type system.
    let mut serverside_wrappers = proc_macro2::TokenStream::new();

    // This will contain match arms that call the respective server-side wrappers
    // depending on the route that the user chose.
    let mut serverside_routing = proc_macro2::TokenStream::new();

    // Loop over each attribute of the annotated item.
    for attribute in input.attrs {
        // Extract the identifier of the attribute.
        let attribute_ident = match attribute.path().get_ident() {
            Some(ident) => ident,
            _ => continue,
        };

        // Check if the attribute belongs to `beyond`.
        match attribute_ident.to_string().as_str() {
            // The attribute is `beyond_route`, which is used
            // to define a new route and the request- and response-types
            // it will use.
            "beyond_route" => {
                // Parse the route definition from the tokens in `#[beyond_route(...)]`.
                let meta_list = attribute.meta.require_list()?;
                let route: Route = meta_list.parse_args()?;

                // Insert the code for the client to call the server binary
                // with the correct route over SSH.
                let clientside_method_tokens = route.to_clientside_method_tokens();
                output.extend(quote! {
                    impl Client {
                        #clientside_method_tokens
                    }
                });

                // Add the server-side wrapper to the other wrappers, which will later be injected into the
                // `struct Server` definition.
                let serverside_wrapper_tokens = route.to_serverside_wrapper_tokens(&server_ident);
                serverside_wrappers.extend(serverside_wrapper_tokens);

                // Add a match arm to the routing logic to call the correct wrapper
                // for the route.
                let serverside_routing_tokens = route.to_serverside_routing_tokens();
                serverside_routing.extend(serverside_routing_tokens);
            }
            _ => continue, // Ignore all other attributes.
        }
    }

    // Add the core logic to the final code.
    output.extend(quote::quote! {
        pub struct Client {
            ssh: ::beyond::ssh::SSH,
            server_binary: String,
        }

        impl Client {
            pub fn new(destination: &str, server_binary: String) -> ::core::result::Result<Self, ::beyond::Error> {
                Ok(Self {
                    ssh: ::beyond::ssh::SSH::new(destination)?,
                    server_binary,
                })
            }

            pub fn check_server(&mut self) -> ::core::result::Result<(), ::beyond::Error> {
                let output = self.ssh.execute(&format!("which {}", self.server_binary))?;

                if output.status.success() {
                    ::core::result::Result::Ok(())
                } else {
                    ::core::result::Result::Err(::beyond::Error::ServerComponentNotInstalled)
                }
            }
        }

        impl #server_ident {
            // Insert the server-side wrappers around the user logic here.
            #serverside_wrappers

            pub fn run(server: #server_ident) -> ::core::option::Option<::std::process::ExitCode> {
                // Check if the should actually run.
                if ::std::env::args().nth(1).unwrap_or_default() != "beyond-server-process" {
                    return ::core::option::Option::None;
                }

                // Get the route and it's request to use.
                let route_name = ::std::env::args().nth(2).unwrap_or_default();
                let encoded_request = ::std::env::args().nth(3).unwrap_or_default();

                // Call the function associated with the route.
                let encoded_response_result = match route_name.as_str() {
                    #serverside_routing
                    _ => ::core::result::Result::Err(::beyond::Error::InvalidRoute { route_name }),
                };

                // Check if the function succeeded and print the result
                // to stdout or stderr accordingly.
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

#[proc_macro_derive(Beyond, attributes(beyond_route))]
pub fn beyond_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<syn::DeriveInput>(input).unwrap();
    let internal = input.ident;

    let mut output = proc_macro2::TokenStream::new();

    output.extend(quote::quote! {
        pub struct Beyond {
            internal: #internal
        }
    });

    output.into()
}


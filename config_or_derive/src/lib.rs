use proc_macro::TokenStream;
mod config_or_default;
use config_or_default::impl_config_or_derive;

#[proc_macro_derive(ConfigOr, attributes(config_or))]
pub fn config_or(tokens: TokenStream) -> TokenStream {
    // Parse the string representation

    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    // Build the impl
    let gen = impl_config_or_derive(&ast);

    // Return the generated impl
    gen.unwrap_or_else(syn::Error::into_compile_error).into()
}

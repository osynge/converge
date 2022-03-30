use proc_macro::TokenStream;
mod clone_default;
use clone_default::impl_hello_world;

#[proc_macro_derive(CloneWithDefault, attributes(cwd))]
pub fn clone_with_default(tokens: TokenStream) -> TokenStream {
    // Parse the string representation

    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.unwrap_or_else(syn::Error::into_compile_error).into()
}

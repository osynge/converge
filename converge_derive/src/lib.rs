use proc_macro::TokenStream;
mod converge_default;
use converge_default::impl_converge_derive;

#[proc_macro_derive(Converge, attributes(converge))]

/** Simple macro to derive the Converge trait.

The combine attribute can take either:

* #[converge(nest)]
* #[converge(strategy = custom_converge_function)]

## nest

This recursively applies the converge trait.

## strategy

Takes a function to implement the converge operation for the field.

 */
pub fn converge(tokens: TokenStream) -> TokenStream {
    // Parse the string representation

    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

    // Build the impl
    let gen = impl_converge_derive(&ast);

    // Return the generated impl
    gen.unwrap_or_else(syn::Error::into_compile_error).into()
}

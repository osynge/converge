use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Path, Type, TypePath};

pub fn impl_config_or_derive(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let field_token_stream = fields
        .into_iter()
        .map(|f| {
            // Interpolation only works for variables, not arbitrary expressions.
            // That's why we need to move these fields into local variables first
            // (borrowing would also work though).
            let field_name = &f.ident;
            let field_ty = &f.ty;
            let mut nested = false;
            for attr in f.attrs.iter() {
                match attr.parse_meta().unwrap() {
                    // Find the duplicated idents
                    syn::Meta::Path(ref path) if path.get_ident().unwrap() == "config_or" => {
                        nested = true;
                    }
                    _ => (),
                }
            }
            match nested {
                true => Ok(
                    quote! {#field_name : self.#field_name.config_or( default.#field_name),
                    },
                ),
                false => match is_option(field_ty) {
                    true => Ok(
                        quote! {#field_name : self.#field_name.or(default.#field_name),
                        },
                    ),
                    false => Ok(quote! {#field_name : self.#field_name,
                    }),
                },
            }
        })
        .collect::<syn::Result<TokenStream>>()?;

    Ok(quote! {
    #[automatically_derived]
    impl ConfigOr for #name {
        fn config_or(self, default: #name) -> Self {
            #name {
                #field_token_stream
            }
            }
        }
    })
}

fn is_option(field_ty: &Type) -> bool {
    match field_ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => {
            // segments is of Type syn::punctuated::Punctuated<PathSegment, _>
            let filtered: Vec<String> = segments
                .clone()
                .into_pairs()
                .map(|path_pair| path_pair.value().ident.to_string())
                .collect();
            match filtered.join("::").as_str() {
                "Option" | "option::Option" | "std::option::Option" | "core::option::Option" => {
                    true
                }
                _ => false,
            }
        }
        _ => false,
    }
}

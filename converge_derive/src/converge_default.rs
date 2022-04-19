use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, Path, Type, TypePath};

pub fn impl_converge_derive(ast: &DeriveInput) -> syn::Result<TokenStream> {
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

            let meta: CombineMeta = f
                .attrs
                .iter()
                .filter(|attr| attr.path.is_ident("converge"))
                .try_fold(CombineMeta::default(), |meta, attr| {
                    let list: syn::punctuated::Punctuated<CombineMeta, syn::Token![,]> =
                        attr.parse_args_with(syn::punctuated::Punctuated::parse_terminated)?;

                    list.into_iter().try_fold(meta, CombineMeta::merge)
                })?;

            let field_name = &f.ident;
            let field_ty = &f.ty;

            match (meta.nest, meta.strategy) {
                // This should never happen if checked CombineMeta::merge
                (Some(_), Some(_)) => panic!("conflicting attribute argument"),
                (None, Some(strategy_fn)) => Ok(
                    quote! {#field_name : #strategy_fn(self.#field_name , default.#field_name),
                    },
                ),
                (Some(_), None) => Ok(
                    quote! {#field_name : self.#field_name.converge( default.#field_name),
                    },
                ),
                (None, None) => match is_option(field_ty) {
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
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    Ok(quote! {
    #[automatically_derived]
    impl #impl_generics Converge for #name  #ty_generics #where_clause {
        fn converge(self, default: #name #ty_generics) -> Self {
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

#[derive(Default)]
struct CombineMeta {
    nest: Option<proc_macro2::Span>,
    strategy: Option<Ident>,
}

impl CombineMeta {
    fn merge(self, other: CombineMeta) -> syn::Result<Self> {
        self.invalid()?;
        other.invalid()?;
        let output = Self {
            nest: self.nest.or(other.nest),
            strategy: self.strategy.or(other.strategy),
        };
        output.invalid()?;
        Ok(output)
    }
    fn invalid(&self) -> syn::Result<()> {
        match (self.nest.as_ref(), self.strategy.as_ref()) {
            (Some(a), Some(b)) => {
                let mut error = syn::Error::new(*a, "conflicting attribute argument");
                error.combine(syn::Error::new_spanned(
                    b,
                    "conflicting  attribute argument",
                ));
                Err(error)
            }
            _ => Ok(()),
        }
    }
}

impl syn::parse::Parse for CombineMeta {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::nest) {
            let val = input.parse();
            let nest: kw::nest = val?;
            Ok(Self {
                nest: Some(nest.span),
                strategy: None,
            })
        } else if lookahead.peek(kw::strategy) {
            let _: kw::strategy = input.parse()?;
            let _: syn::Token![=] = input.parse()?;
            let vis = input.parse()?;

            Ok(Self {
                nest: None,
                strategy: Some(vis),
            })
        } else {
            Err(lookahead.error())
        }
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(nest);
    custom_keyword!(strategy);
}

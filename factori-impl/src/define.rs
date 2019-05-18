use itertools::zip;
use proc_macro2::{Ident, TokenStream, TokenTree};
use proc_macro_rules::rules;
use quote::quote;
use syn::{Expr, Type};

use super::{ident_builder, ident_features_enum};

pub fn define_macro(input: TokenStream) -> TokenStream {
    rules!(input => {
        (
            $(
                $ty:ident, {
                    default {
                        $( $field_names:ident $(: $field_types:ty)? = $field_values:expr ),*
                        $(,)?
                    }

                    $(builder $builder:tt)?

                    $(
                        feature $feature_names:ident {
                            $( $feature_fields:ident = $feature_values:expr ),*
                            $(,)?
                        }
                    )*
                }
            )*
        ) => {
            let mut stream = TokenStream::new();

            for_repition!(
                (
                    ty, field_names, field_types, field_values, builder,
                    feature_names, feature_fields, feature_values
                ) => {
                    let definition = Definition {
                        ty,
                        builder,

                        field_names,
                        field_values,
                        field_types,

                        feature_names,
                        feature_fields,
                        feature_values,
                    };

                    if let Some(error) = definition.validate() {
                        return error;
                    }

                    stream.extend(definition.into_token_stream());
                }
            );

            stream.into()
        }
    })
}

struct Definition {
    ty: Ident,
    builder: Option<TokenTree>,

    field_names: Vec<Ident>,
    field_types: Vec<Option<Type>>,
    field_values: Vec<Expr>,

    feature_names: Vec<Ident>,
    feature_fields: Vec<Vec<Ident>>,
    feature_values: Vec<Vec<Expr>>,
}

impl Definition {
    fn validate(&self) -> Option<TokenStream> {
        let missing_type = zip(&self.field_names, &self.field_types)
            .filter(|(_, ty)| ty.is_none())
            .next();

        if let Some((name, _)) = missing_type {
            if self.builder.is_some() {
                let error = syn::Error::new(
                    name.span(),
                    "Type must be specified if using a custom `builder {}` block.",
                )
                .to_compile_error();

                return Some(error);
            }
        }

        None
    }

    fn generate_builder(&self) -> TokenStream {
        let ident_builder = ident_builder(&self.ty);

        let ty = &self.ty;
        let field_names = &self.field_names;
        let field_names2 = &self.field_names;
        let field_types = &self.field_types;
        let field_values = &self.field_values;

        match &self.builder {
            None => {
                quote! {
                    #[allow(non_camel_case_types)]
                    type #ident_builder = #ty;

                    impl factori::Default for #ident_builder {
                        fn default() -> Self {
                            #ty {
                                #( #field_names: #field_values ),*
                            }
                        }
                    }

                    impl factori::Builder for #ident_builder {
                        type Ty = #ty;

                        fn build(self) -> Self::Ty {
                            self
                        }
                    }
                }
            }

            Some(builder) => {
                quote! {
                    #[allow(non_camel_case_types, dead_code)]
                    pub struct #ident_builder {
                        #( #field_names: #field_types),*
                    }

                    impl factori::Default for #ident_builder {
                        fn default() -> Self {
                            #ident_builder {
                                #( #field_names: #field_values),*
                            }
                        }
                    }

                    impl factori::Builder for #ident_builder {
                        type Ty = #ty;

                        fn build(self) -> Self::Ty {
                            #(
                                #[allow(unused_variable)]
                                let #field_names = self.#field_names2;
                            )*

                            #builder
                        }
                    }
                }
            }
        }
    }

    fn generate_features(&self) -> TokenStream {
        let ident_builder = ident_builder(&self.ty);
        let ident_features_enum = ident_features_enum(&self.ty);

        // Repeat so we can refer to it inside quote!'s #()*:
        let idents_builder = std::iter::repeat(&ident_builder);
        let idents_features_enum = std::iter::repeat(&ident_features_enum);

        let feature_names = &self.feature_names;
        let feature_fields = &self.feature_fields;
        let feature_values = &self.feature_values;

        quote! {
            #[allow(non_camel_case_types)]
            pub enum #ident_features_enum {
                #( #feature_names ),*
            }

            impl factori::Feature<#ident_builder> for #ident_features_enum {
                fn default(self) -> #ident_builder {
                    self.extend(factori::Default::default())
                }

                #[allow(unused_variable)]
                fn extend(self, other: #ident_builder) -> #ident_builder {
                    match self {
                        #(
                            #idents_features_enum::#feature_names => {
                                #idents_builder {
                                    #(
                                        #feature_fields: #feature_values
                                    ),* ,
                                    .. other
                                }
                            }
                        ),*
                    }
                }
            }
        }
    }

    fn into_token_stream(&self) -> TokenStream {
        let builder = self.generate_builder();
        let features = self.generate_features();

        quote! {
            #builder
            #features
        }
    }
}

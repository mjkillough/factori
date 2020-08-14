use itertools::zip;
use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parse_macro_input, Expr, Token, Type};

use super::{ident_builder, ident_mixins_enum};

struct DefaultBlock {
    fields: Vec<Ident>,
    types: Vec<Option<Type>>,
    values: Vec<Expr>,
}

impl Parse for DefaultBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner;
        braced!(inner in input);

        let mut fields = Vec::new();
        let mut types = Vec::new();
        let mut values = Vec::new();

        loop {
            if inner.is_empty() {
                break;
            }

            fields.push(inner.parse()?);

            // Optional type. If it's specified for one field it needs to be specified for all.
            // Should be specified only if there is a builder {} block.
            // This is enforced in Definition::validate().
            if inner.peek(Token![:]) {
                inner.parse::<Token![:]>()?;
                types.push(Some(inner.parse()?));
            } else {
                types.push(None);
            }

            inner.parse::<Token![=]>()?;
            values.push(inner.parse()?);

            if inner.peek(Token![,]) {
                inner.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            fields,
            types,
            values,
        })
    }
}

struct MixinBlock {
    name: Ident,
    fields: Vec<Ident>,
    values: Vec<Expr>,
}

impl Parse for MixinBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;

        let inner;
        braced!(inner in input);

        let mut fields = Vec::new();
        let mut values = Vec::new();

        loop {
            if inner.is_empty() {
                break;
            }

            fields.push(inner.parse()?);
            inner.parse::<Token![=]>()?;
            values.push(inner.parse()?);

            if inner.peek(Token![,]) {
                inner.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            name,
            fields,
            values,
        })
    }
}

struct Definition {
    ty: Ident,

    default: DefaultBlock,
    builder: Option<TokenTree>,
    mixins: Vec<MixinBlock>,
}

impl Parse for Definition {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse()?;
        input.parse::<Token![,]>()?;

        let inner;
        braced!(inner in input);

        let mut default: Option<DefaultBlock> = None;
        let mut builder = None;
        let mut mixins = Vec::new();

        loop {
            if inner.is_empty() {
                break;
            }

            let key: Ident = inner.parse()?;
            if key == "default" {
                if default.is_some() {
                    return Err(inner.error("default {} block defined twice"));
                }
                default = Some(inner.parse()?);
            } else if key == "builder" {
                if builder.is_some() {
                    return Err(inner.error("builder {} block is defined twice"));
                }
                builder = Some(inner.parse()?);
            } else if key == "mixin" {
                mixins.push(inner.parse()?);
            }
        }

        let default = default.ok_or_else(|| inner.error("missing default {} block"))?;

        Ok(Self {
            ty,
            default,
            builder,
            mixins,
        })
    }
}

impl Definition {
    fn validate(&self) -> Option<TokenStream> {
        let missing_type = zip(&self.default.fields, &self.default.types)
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
        let fields = &self.default.fields;
        let types = &self.default.types;
        let values = &self.default.values;

        match &self.builder {
            None => {
                quote! {
                    #[allow(non_camel_case_types)]
                    pub type #ident_builder = #ty;

                    impl factori::Default for #ident_builder {
                        fn default() -> Self {
                            #ty {
                                #( #fields: #values ),*
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
                        #( pub #fields: #types ),*
                    }

                    impl factori::Default for #ident_builder {
                        fn default() -> Self {
                            #ident_builder {
                                #( #fields: #values ),*
                            }
                        }
                    }

                    impl factori::Builder for #ident_builder {
                        type Ty = #ty;

                        fn build(self) -> Self::Ty {
                            #(
                                #[allow(unused_variable)]
                                let #fields = self.#fields;
                            )*

                            #builder
                        }
                    }
                }
            }
        }
    }

    fn generate_mixins(&self) -> TokenStream {
        let ident_builder = ident_builder(&self.ty);
        let ident_mixins_enum = ident_mixins_enum(&self.ty);

        let idents_builder = &ident_builder;
        let idents_mixins_enum = &ident_mixins_enum;

        let mixin_names: Vec<_> = self.mixins.iter().map(|mixin| &mixin.name).collect();
        let mixin_fields: Vec<_> = self.mixins.iter().map(|mixin| &mixin.fields).collect();
        let mixin_values: Vec<_> = self.mixins.iter().map(|mixin| &mixin.values).collect();

        quote! {
            #[allow(non_camel_case_types)]
            pub enum #ident_mixins_enum {
                #( #mixin_names ),*
            }

            impl factori::Mixin<#ident_builder> for #ident_mixins_enum {
                fn default(self) -> #ident_builder {
                    self.extend(factori::Default::default())
                }

                #[allow(unused_variable)]
                fn extend(self, other: #ident_builder) -> #ident_builder {
                    match self {
                        #(
                            #idents_mixins_enum::#mixin_names => {
                                #idents_builder {
                                    #(
                                        #mixin_fields: #mixin_values
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
        let mixins = self.generate_mixins();

        quote! {
            #builder
            #mixins
        }
    }
}

struct MultipleDefinition {
    definitions: Vec<Definition>,
}

impl Parse for MultipleDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut definitions = Vec::new();

        loop {
            if input.is_empty() {
                break;
            }
            definitions.push(input.parse()?);
        }

        Ok(Self { definitions })
    }
}

pub fn define_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let MultipleDefinition { definitions } = parse_macro_input!(input);

    let mut stream = TokenStream::new();
    for definition in definitions {
        if let Some(error) = definition.validate() {
            return error.into();
        }
        stream.extend(definition.into_token_stream());
    }

    stream.into()
}

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, Ident, Token};

use super::{ident_builder, ident_mixins_enum};

// e.g. create!(ty, :mixin1, :mixin2, field1: value1, field2: value2)
//
// ... becomes:
//
// Create {
//     ty: 'ty',
//     mixins: vec!['mixin1', 'mixin2'],
//     fields: vec!['field1', 'field2'],
//     values: vec!['value1', 'value2'],
// }
struct Create {
    ty: Ident,
    mixins: Vec<Ident>,
    fields: Vec<Ident>,
    values: Vec<Expr>,
}

impl Parse for Create {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse()?;

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        let mut mixins = Vec::new();
        while input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            mixins.push(input.parse()?);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        let mut fields = Vec::new();
        let mut values = Vec::new();
        loop {
            if input.is_empty() {
                break;
            }

            fields.push(input.parse()?);
            input.parse::<Token![:]>()?;
            values.push(input.parse()?);
        }

        Ok(Create {
            ty,
            mixins,
            fields,
            values,
        })
    }
}

pub fn create_macro(input: TokenStream) -> TokenStream {
    let Create {
        ty,
        mixins,
        fields,
        values,
    } = parse_macro_input!(input);

    let ident_builder = ident_builder(&ty);
    let ident_mixins_enum = ident_mixins_enum(&ty);

    let mut mixins = mixins.iter();
    let value = if let Some(mixin) = mixins.next() {
        let initial = quote! {
            factori::Mixin::default(#ident_mixins_enum::#mixin)
        };
        mixins.fold(initial, |acc, mixin| {
            quote! {
                factori::Mixin::extend(#ident_mixins_enum::#mixin, #acc)
            }
        })
    } else {
        quote! { factori::Default::default () }
    };

    let quoted = quote! {
        factori::Builder::build(#ident_builder {
            #(
                #fields: #values,
            )*
            .. #value
        })
    };

    quoted.into()
}

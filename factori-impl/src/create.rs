use proc_macro2::TokenStream;
use proc_macro_rules::rules;
use quote::quote;

use super::{ident_builder, ident_mixins_enum};

pub fn create_macro(input: TokenStream) -> TokenStream {
    rules!(input => {
        (
            $ty:ident $(,)?
            $( : $mixins:ident ),* $(,)?
            $( $fields:ident: $values:expr ),* $(,)?
        ) => {
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

            quote! {
                factori::Builder::build(#ident_builder {
                    #(
                        #fields: #values,
                    )*
                    .. #value
                })
            }
        }
    })
}

use proc_macro2::TokenStream;
use proc_macro_rules::rules;
use quote::quote;

use super::{ident_builder, ident_features_enum};

pub fn create_macro(input: TokenStream) -> TokenStream {
    rules!(input => {
        (
            $ty:ident $(,)?
            $( : $features:ident ),* $(,)?
            $( $fields:ident: $values:expr ),* $(,)?
        ) => {
            let ident_builder = ident_builder(&ty);
            let ident_features_enum = ident_features_enum(&ty);

            let mut features = features.iter().rev();
            let value = if let Some(feature) = features.next() {
                let initial = quote! {
                    factori::Feature::default(#ident_features_enum::#feature)
                };
                features.fold(initial, |acc, feature| {
                    quote! {
                        factori::Feature::extend(#ident_features_enum::#feature, #acc)
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

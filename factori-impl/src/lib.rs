extern crate proc_macro;

#[macro_use]
mod utils;
mod create;
mod define;

use proc_macro2::Ident;
use proc_macro_hack::proc_macro_hack;

fn ident_builder(ty: &Ident) -> Ident {
    let ident = format!("_Factori_Builder_{}", ty);
    Ident::new(&ident, ty.span())
}

fn ident_mixins_enum(ty: &Ident) -> Ident {
    let ident = format!("_Factori_Mixins_{}", ty);
    Ident::new(&ident, ty.span())
}

#[proc_macro]
pub fn factori(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    define::define_macro(input.into()).into()
}

#[proc_macro_hack]
pub fn create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    create::create_macro(input.into()).into()
}

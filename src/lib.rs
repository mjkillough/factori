use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use factori_impl::create;
pub use factori_impl::factori;

#[doc(hidden)]
pub trait Builder {
    type Ty;

    fn build(self) -> Self::Ty;
}

#[doc(hidden)]
pub trait Default {
    fn default() -> Self;
}

#[doc(hidden)]
pub trait Feature<T> {
    fn default(self) -> T;
    fn extend(self, other: T) -> T;
}

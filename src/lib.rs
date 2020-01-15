use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(support_nested)]
pub use factori_impl::create;

#[doc(hidden)]
pub use factori_impl;

#[macro_export]
macro_rules! factori {
    // We define a simple macro so that the documentation doesn't state this
    // is a re-export from factori-impl. This also allows us to write docs here.
    ($($input:tt)*) => {
        $crate::factori_impl::define!($($input)*);
    }
}

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
pub trait Mixin<T> {
    fn default(self) -> T;
    fn extend(self, other: T) -> T;
}

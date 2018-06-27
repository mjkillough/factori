extern crate mashup;

#[doc(hidden)]
#[allow(unused_imports)]
use mashup::*;

#[macro_use]
mod create;
#[macro_use]
mod define;

#[doc(hidden)]
pub trait FactoriBuilder {
    type Built;

    fn build(self) -> Self::Built;
}

#[doc(hidden)]
pub trait FactoriDefault {
    fn default() -> Self;
}

#[doc(hidden)]
pub trait FactoriTrait<T> {
    fn default(self) -> T;
    fn expand(self, other: T) -> T;
}

extern crate mashup;

// We re-export the macros from mashup, to avoid dependent crates having to add
// it as a dependency.
pub use mashup::*;

#[macro_use]
mod create;
#[macro_use]
mod define;

pub trait FactoriBuilder {
    type Built;

    fn build(self) -> Self::Built;
}

pub trait FactoriDefault {
    fn default() -> Self;
}

pub trait FactoriTrait<T> {
    fn default(self) -> T;
    fn expand(self, other: T) -> T;
}

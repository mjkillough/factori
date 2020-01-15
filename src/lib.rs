//! A testing factory library for Rust, inspired by [FactoryBot].
//!
//! Factori aims to provide a clean, ergonomic syntax for instantiating test
//! objects, without sacrificing type-safety.
//!
//! This crate provides:
//!  - A [`factori!()`] macro which is used to define factories.
//!  - A [`create!()`] macro which is used to instantiate objects from
//!    factories.
//!
//! [FactoryBot]: https://github.com/thoughtbot/factory_bot
//! [`factori!()`]: macro.factori.html
//! [`create!()`]: macro.create.html
//!
//! ## Example
//!
//! ```
//! #[macro_use]
//! extern crate factori;
//!
//! pub struct Vehicle {
//!     number_wheels: u8,
//!     electric: bool,
//! }
//!
//! factori!(Vehicle, {
//!     default {
//!         number_wheels = 4,
//!         electric = false,
//!     }
//!
//!     mixin bike {
//!         number_wheels = 2,
//!     }
//! });
//!
//! fn main() {
//!     let default = create!(Vehicle);
//!     assert_eq!(default.number_wheels, 4);
//!     assert_eq!(default.electric, false);
//!
//!     // Its type is Vehicle, nothing fancy:
//!     let vehicle: Vehicle = default;
//!
//!     let three_wheels = create!(Vehicle, number_wheels: 3);
//!     assert_eq!(three_wheels.number_wheels, 3);
//!
//!     let electric_bike = create!(Vehicle, :bike, electric: true);
//!     assert_eq!(electric_bike.number_wheels, 2);
//!     assert_eq!(electric_bike.electric, true);
//! }
//! ```
//!
//! More examples are available in the [`tests/`] alongside the crate.
//!
//! [`tests/`]: https://github.com/mjkillough/factori/tree/master/tests
//!
//! ## How it works
//!
//! Behind the scenes, the [`factori!()`] macro generates some extra types to
//! encode the default values and mixins for each factory.
//!
//! The [`create!()`] macro expects the generated `_Factori` types to be in
//! scope. If the factory is instantiated in the same module that it is
//! defined, this will work as expected. If the factory is defined in a
//! separate module, then it is recommended that you do a glob import to bring
//! them into scope.
//!
//! In most projects, you should expect to have a (or a few) `factories`
//! modules which contain shared factories. In `tests` modules you can then use
//! a glob import to bring all of the required types into scope without
//! having them cluttering up your project's namespaces.
//!
//! ```
//! # #[macro_use] extern crate factori;
//! # fn main() { }
//! #
//! struct Vehicle {
//!     number_wheels: u8,
//! }
//!
//! mod factories {
//!     use super::Vehicle;
//!
//!     factori!(Vehicle, {
//!         default {
//!             number_wheels = 4
//!         }
//!     });
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::{Vehicle, factories::*};
//!
//!     #[test]
//!     fn some_test() {
//!         let vehicle = create!(Vehicle);
//!         assert_eq!(vehicle.number_wheels, 4);
//!     }
//! }
//! ```
//!
//! The implementation details of the [`factori!()`] and [`create!()`] macros
//! are considered private and you should not rely on any of the generated
//! types or their names. However, the implementation is quite simple and you
//! are encouraged to run [`cargo-expand`] in order to see the generated code.
//!
//! The generated types are all prefixed with `_Factori` and are unlikely to
//! clash with any types in your crate. It is a little gross but it is all
//! in the name of testing convenience.
//!
//! [`cargo-expand`]: https://github.com/dtolnay/cargo-expand
//!
//! ## Error messages
//!
//! The error messages coming from these macros are surprisingly good
//! considering what they're doing. However, if you encounter weird error
//! messages that aren't self-explanatory, please raise an issue on the GitHub
//! repository.

// Clippy seems to get confused when testing procedural macros in doctests:
#![allow(clippy::needless_doctest_main)]

use proc_macro_hack::proc_macro_hack;

/// A macro to instantiate an instance of a factory.
///
/// The type must already have had a factory defined using the [`factori!()`]
/// macro.
///
/// The `create!()` macro accepts:
///
///  - The type to be instantiated using its factory.
///  - Zero or more comma-separated mixins using the syntax `:name`.
///
///    These are applied in the order that they are passed to `create!()`,
///    which means that later mixins might override attributes already set by
///    earlier mixins.
///
///    You can think of the default values defined in the factory's `default`
///    block as an implicit mixin which is always included first in every call
///    to `create!()`.
///  - Zero or more named fields with values, `field: value`.
///
///    These override both the factory's default values and the provided
///    mixins. Each field from the `default` block can appear zero or one
///    times.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate factori;
/// #
/// struct Vehicle {
///     registration: &'static str,
///     number_wheels: u8,
///     number_seats: u8,
/// }
///
/// factori!(Vehicle, {
///     default {
///         registration = "",
///         number_wheels = 4,
///         number_seats = 5,
///     }
///
///     mixin motorbike {
///         number_wheels = 2,
///         number_seats = 1,
///     }
///
///     mixin trike {
///         number_wheels = 3
///     }
/// });
///
/// fn main () {
///     let trike = create!(Vehicle, :motorbike, :trike, registration: "J105 SRA");
///     assert_eq!(trike.number_wheels, 3);
///     assert_eq!(trike.number_seats, 1);
/// }
/// ```
///
/// [`factori!()`]: macro.factori.html
#[proc_macro_hack(support_nested)]
pub use factori_impl::create;

#[doc(hidden)]
pub use factori_impl;

/// A macro to define a factory for a type.
///
/// The macro accepts:
///
///  - The type to be constructed by the factory.
///  - A `default { }` block.
///
///    This provides default values for all fields in the struct.
///  - Zero or more `mixin name { }` blocks.
///
///    These provide values to override the default values of one or more
///    fields. They are typically used to define groups of values which allow
///    you to quickly create test objects which are in certain states.
///
///    Multiple mixin blocks can set the same attributes and the precedence is
///    determined by the order that they are included in calls to [`create!()`].
///
/// [`create!()`]: macro.create.html
///
/// ## Example
///
/// ```
/// # #[macro_use] extern crate factori;
/// #
/// struct Order {
///     id: u64,
///     shipped: bool,
/// }
///
/// factori!(Order, {
///     default {
///         id = 1,
///         shipped = false,
///     }
///
///     mixin shipped {
///         shipped = true,
///     }
/// });
///
/// fn main() {
///     let order = create!(Order, :shipped);
/// }
/// ```
///
/// ## Constructing complex types
///
/// Under the hood, the example above constructs `Vehicle` using the struct
/// literal syntax, passing the values defined in the `default` and `mixin`
/// blocks.
///
/// This isn't always possible, such as for types which can't be constructed
/// with struct literal syntax (enums and tuple structs) or types with private
/// fields. For these more complex types, a `builder` block can be provided to
/// tell `factori!()` how to turn the fields in the `default` and `mixin`
/// blocks into the factory's type.
///
/// When a `builder` block is provided, the fields in `default` define an
/// anonymous, temporary struct that is used during factory construction. To
/// achieve this, the types of fields must be provided inside the `default`
/// block.
///
/// ```
/// # #[macro_use] extern crate factori;
/// #
/// pub struct Order(u64, bool);
///
/// factori!(Order, {
///     default {
///         id: u64 = 1,
///         shipped: bool = false,
///     }
///
///     builder {
///         // All fields from default { } are in scope here with their values.
///         // We construct a tuple struct here, but we could easily call a
///         // method like Order::new().
///         Order(id, shipped)
///     }
///
///     mixin shipped {
///         shipped = true,
///     }
/// });
///
/// fn main() {
///     let order = create!(Order, :shipped, id: 2);
/// }
/// ```
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

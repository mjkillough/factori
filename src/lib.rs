extern crate mashup;

// We re-export the macros from mashup, to avoid dependent crates having to add
// it as a dependency.
pub use mashup::*;

pub trait FactoriDefault {
    fn default() -> Self;
}

pub trait FactoriTrait<T> {
    fn default(self) -> T;
    fn expand(self, other: T) -> T;
}

#[macro_export]
macro_rules! factori_define_traits {
    (
        $ty:ident,
        $enum:ident,
        $(
            trait $trait_name:ident {
                $( $trait_field:ident : $trait_value:expr ),*
                $(,)*
            }
        )*
    ) => {
        #[allow(non_camel_case_types)]
        enum $enum {
            $( $trait_name, )*
        }

        impl $crate::FactoriTrait<$ty> for $enum {
            fn default(self) -> $ty {
                self.expand($crate::FactoriDefault::default())
            }

            fn expand(self, other: $ty) -> $ty {
                match self {
                    $(
                        $enum::$trait_name => $ty {
                            $( $trait_field: $trait_value, )*
                            .. other
                        }
                    ),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! factori {
    ($ty:ident, {
        default {
            $( $field:ident : $value:expr ),* $(,)*
        }

        $(
            trait $trait_name:ident {
                $( $trait_field:ident : $trait_value:expr ),*
                $(,)*
            }
        )*

        $(,)*
    }) => {
        impl $crate::FactoriDefault for $ty {
            fn default() -> Self {
                $ty {
                    $($field : $value,)*
                }
            }
        }

        mashup! {
            m["traits_enum"] = $ty Traits;
        }

        m! {
            factori_define_traits!(
                $ty,
                "traits_enum",
                 $(
                    trait $trait_name {
                        $( $trait_field: $trait_value ),*
                    }
                )*
            );
        }
    };
}

#[macro_export]
macro_rules! factori_expand_traits {
    (
        $ty:ident,
        $enum:ident
        $(,)*
    ) => {
        $ty {
            .. $crate::FactoriDefault::default()
        }
    };

    (
        $ty:ident,
        $enum:ident,
        $trait:ident
        $(,)*
    ) => {
        $crate::FactoriTrait::default($enum::$trait)
    };

    (
        $ty:ident,
        $enum:ident,
        $trait:ident,
        $( $other_trait:ident ),*
        $(,)*
    ) => {
        $crate::FactoriTrait::expand(
            $enum::$trait,
            factori_expand_traits!(
                $ty,
                $enum,
                $( $other_trait ),*
            )
        )
    };
}

#[macro_export]
macro_rules! create {
    ($ty:ident) => {
        $ty {
            .. $crate::FactoriDefault::default ()
        }
    };

    (
        $ty:ident,
        $( :$trait:ident ),* $(,)*
        $( $field:ident: $value:expr ),*
        $(,)*
    ) => {{
        mashup! {
            m2["traits_enum"] = $ty Traits;
        }
        m2! {
            $ty {
                $($field: $value,)*
                .. factori_expand_traits!($ty, "traits_enum", $( $trait ),*)
            }
        }
    }};
}

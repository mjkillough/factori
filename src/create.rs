
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
macro_rules! factori_create_internal {
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
            // $ty is always a _Builder type our macros define, so there is no Factori_ prefix:
            m3["traits_enum"] = $ty _Traits;
        }
        m3! {
            $ty {
                $($field: $value,)*
                .. factori_expand_traits!($ty, "traits_enum", $( $trait ),*)
            }
        }
    }};
}

#[macro_export]
macro_rules! create {
    ($ty:ident) => {{
        mashup! {
            m4["builder"] = _Factori_ $ty _Builder;
        }
        m4! {
            $crate::FactoriBuilder::build(
                factori_create_internal!("builder")
            )
        }
    }};

    (
        $ty:ident,
        $( :$trait:ident ),* $(,)*
        $( $field:ident: $value:expr ),*
        $(,)*
    ) => {{
        mashup! {
            m5["builder"] = _Factori_ $ty _Builder;
        }
        m5! {
            $crate::FactoriBuilder::build(
                factori_create_internal!(
                    "builder",
                    $( :$trait ),*
                    $( $field: $value ),*
                )
            )
        }
    }};
}

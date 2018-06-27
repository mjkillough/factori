#[macro_export]
macro_rules! _factori_define_traits {
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
        pub enum $enum {
            $( $trait_name, )*
        }

        impl $crate::FactoriTrait<$ty> for $enum {
            fn default(self) -> $ty {
                self.expand($crate::FactoriDefault::default())
            }

            fn expand(self, _other: $ty) -> $ty {
                match self {
                    $(
                        $enum::$trait_name => $ty {
                            $( $trait_field: $trait_value, )*
                            .. _other
                        }
                    ),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! _facori_builder_internal {
    (
        $fields_struct:ident, $traits_enum:ident,
        {
            default {
                $( $field_name:ident: $field_type:ty = $field_value:expr ),*
            }

            $(
                trait $trait_name:ident {
                    $( $trait_field:ident : $trait_value:expr ),*
                    $(,)*
                }
            )*
        }
    ) => {
        #[allow(non_camel_case_types)]
        pub struct $fields_struct {
            $( pub $field_name: $field_type ),*
        }

        impl $crate::FactoriDefault for $fields_struct {
            fn default() -> Self {
                $fields_struct {
                    $($field_name : $field_value,)*
                }
            }
        }

        _factori_define_traits!(
            $fields_struct,
            $traits_enum,
                $(
                trait $trait_name {
                    $( $trait_field: $trait_value ),*
                }
            )*
        );
    };
}


#[macro_export]
macro_rules! _factori_multi_internal {
    () => {};

    (
        $ty:ident, $builder:ident, $trait_enum:ident, {
            default {
                $( $field_name:ident: $field_type:ty = $field_value:expr ),*
                $(,)*
            }

            builder $builder_body:tt

            $(
                trait $trait_name:ident {
                    $( $trait_field:ident : $trait_value:expr ),*
                    $(,)*
                }
            )*
        }

        $( $more:tt )*
    ) => {
        _facori_builder_internal!($builder, $trait_enum, {
            default {
                $( $field_name: $field_type = $field_value ),*
            }

            $(
                trait $trait_name {
                    $( $trait_field : $trait_value ),*
                }
            )*
        });

        impl $crate::FactoriBuilder for $builder {
            type Built = $ty;

            fn build(self) -> Self::Built {
                $(
                    #[allow(unused_variables)]
                    let $field_name = self.$field_name;
                )*
                $builder_body
            }
        }

        _factori_multi_internal!( $( $more )* );
    };

    (
        $ty:ident, $builder:ident, $trait_enum:ident, {
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
        }

        $( $more:tt )*
    ) => {
        impl $crate::FactoriDefault for $ty {
            fn default() -> Self {
                $ty {
                    $($field : $value,)*
                }
            }
        }

        #[allow(non_camel_case_types)]
        pub type $builder = $ty;

        impl $crate::FactoriBuilder for $builder {
            type Built = $ty;

            fn build(self) -> Self::Built {
                self
            }
        }

        _factori_define_traits!(
            $ty,
            $trait_enum,
                $(
                trait $trait_name {
                    $( $trait_field: $trait_value ),*
                }
            )*
        );

        _factori_multi_internal!( $( $more )* );
    }
}

#[macro_export]
macro_rules! factori {
    (
        $( $ty:ident, $def:tt )*
    ) => {
        mashup! {
            $(
                m6["builder" $ty] = _Factori_ $ty _Builder;
                m6["traits_enum" $ty] = _Factori_ $ty _Builder _Traits;
            )*
        }

        m6! {
            _factori_multi_internal!(
                $(
                    $ty,
                    "builder" $ty,
                    "traits_enum" $ty,
                    $def
                )*
            );
        }
    };
}

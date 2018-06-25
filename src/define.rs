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
        pub enum $enum {
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
            m1["builder"] = _Factori_ $ty _Builder;
            m1["traits_enum"] = _Factori_ $ty _Builder _Traits;
        }

        m1! {
            #[allow(non_camel_case_types)]
            pub type "builder" = $ty;

            impl $crate::FactoriBuilder for "builder" {
                type Built = $ty;

                fn build(self) -> Self::Built {
                    self
                }
            }

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
macro_rules! facori_builder_internal {
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

        factori_define_traits!(
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
macro_rules! factori_builder {
    (
        $ty:ident, {
            default {
                $( $field_name:ident: $field_type:ty = $field_value:expr ),*
                $(,)*
            }

            builder |$builder_fields:ident| $builder:tt

            $(
                trait $trait_name:ident {
                    $( $trait_field:ident : $trait_value:expr ),*
                    $(,)*
                }
            )*
        }
    ) => {
        mashup! {
            m2["fields_struct"] = _Factori_ $ty _Builder;
            m2["traits_enum"] = _Factori_ $ty _Builder _Traits;
        }

        m2! {
            facori_builder_internal!("fields_struct", "traits_enum", {
                default {
                    $( $field_name: $field_type = $field_value ),*
                }

                $(
                    trait $trait_name {
                        $( $trait_field : $trait_value ),*
                    }
                )*
            });
        }

        m2! {
            impl $crate::FactoriBuilder for "fields_struct" {
                type Built = $ty ;

                fn build(self) -> Self::Built {
                    let $builder_fields = self;
                    $builder
                }
            }
        }
    };
}

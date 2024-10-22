#[macro_export]
macro_rules! hal_enum {
    (
        $(#[$enum_meta:meta])+
        $enum_ty:ident: $base_ty:ident {
            default: $default_variant:ident,
            error: $err_ty:ident,
            $(
                $variant:ident = $value:literal$(,)?
            )+
        }
    ) => {
        paste::paste! {
            $(#[$enum_meta])+
            #[repr($base_ty)]
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub enum $enum_ty {
                $(
                    $variant = $value,
                )+
            }

            impl $enum_ty {
                #[doc = "Creates a new [" $enum_ty "]."]
                pub const fn new() -> Self {
                    Self::$default_variant
                }

                #[doc = "Attempts to convert a [`u32`] into a [" $enum_ty "]."]
                pub const fn from_u32(val: u32) -> Result<Self, $err_ty> {
                    match val {
                        $(
                            $value => Ok(Self::$variant),
                        )+
                        _ => Err($err_ty::InvalidVariant(val as usize)),
                    }
                }

                #[doc = "Converts a [" $enum_ty "] into a [`u32`]."]
                pub const fn into_u32(self) -> u32 {
                    self as u32
                }
            }

            impl Default for $enum_ty {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl TryFrom<u32> for $enum_ty {
                type Error = $err_ty;

                fn try_from(val: u32) -> Result<Self, Self::Error> {
                    Self::from_u32(val)
                }
            }

            impl From<$enum_ty> for u32 {
                fn from(val: $enum_ty) -> Self {
                    val.into_u32()
                }
            }
        }
    };
}

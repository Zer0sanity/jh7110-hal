/// Implements a function to determine if a `bitflag` has a set flag.
#[macro_export]
macro_rules! bitflag_is_set {
    ($ty:ident) => {
        impl $ty {
            /// Determines whether `oth` flag is set.
            pub fn is_set(self, oth: Self) -> bool {
                self & oth != Self::NONE
            }
        }
    };
}

/// Implements the [`From`](core::convert::From) trait for bitflag types.
///
/// **NOTE**: bitflag type must be a tuple struct around a [`u32`].
/// Also, the bitflag type must have a member `MASK` that represents the bitmask.
///
/// Example:
///
/// ```no_build,no_run
/// pub struct SomeFlag(u32);
///
/// bitflags! {
///     impl SomeFlag: u32 {
///         const SOME_FIELD: 0b01;
///         const SOME_OTH_FIELD: 0b11;
///         const MASK: 0b11;
///     }
/// }
///
/// bitflag_from_u32!(SomeFlag);
///
/// assert_eq!(SomeFlag::from(1u32), SomeFlag::SOME_FIELD);
/// ```
#[macro_export]
macro_rules! bitflag_from_u32 {
    ($flag:ident) => {
        impl From<u32> for $flag {
            fn from(val: u32) -> Self {
                Self(val & Self::MASK.bits())
            }
        }
    };
}

#[macro_export]
macro_rules! hal_enum {
    (
        $(#[$enum_meta:meta])+
        $enum_ty:ident: $base_ty:ident {
            default: $default_variant:ident,
            error: $err_ty:ident,
            $(
                $(#[$var_doc:meta])?
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
                    $(#[$var_doc])?
                    $variant = $value,
                )+
            }

            impl $enum_ty {
                #[doc = "Creates a new [" $enum_ty "]."]
                pub const fn new() -> Self {
                    Self::$default_variant
                }

                #[doc = "Attempts to convert a [`" $base_ty "`] into a [" $enum_ty "]."]
                pub const fn from_raw(val: $base_ty) -> Result<Self, $err_ty> {
                    match val {
                        $(
                            $value => Ok(Self::$variant),
                        )+
                        _ => Err($err_ty::InvalidVariant(val as usize)),
                    }
                }

                #[doc = "Converts a [" $enum_ty "] into a [`" $base_ty "`]."]
                pub const fn into_raw(self) -> $base_ty {
                    self as $base_ty
                }
            }

            impl Default for $enum_ty {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl TryFrom<$base_ty> for $enum_ty {
                type Error = $err_ty;

                fn try_from(val: $base_ty) -> Result<Self, Self::Error> {
                    Self::from_raw(val)
                }
            }

            impl From<$enum_ty> for $base_ty {
                fn from(val: $enum_ty) -> Self {
                    val.into_raw()
                }
            }

            #[cfg(test)]
            mod tests {
                use super::*;

                #[test]
                fn test_variants() {
                    let raw = [
                        $($value,)+
                    ];

                    let exp = [
                        $($enum_ty::$variant,)+
                    ];

                    raw.into_iter().zip(exp).for_each(|(r, e)| {
                        assert_eq!($enum_ty::from_raw(r), Ok(e));
                        assert_eq!($enum_ty::try_from(r), Ok(e));

                        assert_eq!(e.into_raw(), r);
                        assert_eq!($base_ty::from(e), r);
                    });
                }

                #[test]
                fn test_invalid_variants() {
                    let raw = [
                        $($value,)+
                    ];

                    (0..1024).filter(|r| !raw.iter().any(|v| v == r)).for_each(|r| {
                        assert_eq!($enum_ty::from_raw(r), Err($err_ty::InvalidVariant(r as usize)));
                        assert_eq!($enum_ty::try_from(r), Err($err_ty::InvalidVariant(r as usize)));
                    });
                }
            }
        }
    };
}

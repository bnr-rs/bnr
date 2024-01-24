/// Provides convenience functions to destructure an enum with new-type variants.
#[macro_export]
macro_rules! inner_enum {
    // macro variant for when the enum variant and its type are the same ident
    ($ty:ident, $var:ident) => {
        inner_enum!($ty, $var, $var);
    };

    // macro variant for when the enum variant and its type are potentially different
    ($ty:ident, $var:ident, $var_ty:ident) => {
        impl $ty {
            ::paste::paste! {
                #[doc = "Gets whether `" $ty "` is the variant `" $var "`."]
                pub fn [<is_ $var:snake>](&self) -> bool {
                    matches!(self, $ty::$var(_))
                }

                #[doc = "Gets a reference to `" $ty "` as the variant `" $var "`'s inner type `" $var_ty "`."]
                #[allow(unreachable_patterns)]
                pub fn [<as_ $var:snake>](&self) -> $crate::Result<&$var_ty> {
                    use $crate::Error;

                    match self {
                        $ty::$var(ty) => Ok(ty),
                        _ => Err(Error::Enum(format!("have variant: {self}, expected: {}", ::std::any::type_name::<$var_ty>()))),
                    }
                }

                #[doc = "Converts `" $ty "` into the variant `" $var "`'s inner type `" $var_ty "`."]
                #[allow(unreachable_patterns)]
                pub fn [<into_ $var:snake>](self) -> $crate::Result<$var_ty> {
                    use $crate::Error;

                    match self {
                        $ty::$var(ty) => Ok(ty),
                        _ => Err(Error::Enum(format!("have variant: {self}, expected: {}", ::std::any::type_name::<$var_ty>()))),
                    }
                }
            }
        }
    };
}

/// Implements the `Default` trait for types that have a `new` function.
#[macro_export]
macro_rules! impl_default {
    ($ty:ident) => {
        impl Default for $ty {
            fn default() -> Self {
                $ty::new()
            }
        }
    };
}

/// Implements traits common to XFS enum types.
#[macro_export]
macro_rules! impl_xfs_enum {
    ($ty:ident, $name:expr) => {
        impl $ty {
            /// Gets the [XfsMember](crate::xfs::xfs_struct::XfsMember) name.
            pub const fn xfs_name() -> &'static str {
                $name
            }
        }

        impl From<&u32> for $ty {
            fn from(val: &u32) -> Self {
                (*val).into()
            }
        }

        impl From<$ty> for u32 {
            fn from(val: $ty) -> Self {
                val as u32
            }
        }

        impl From<&$ty> for u32 {
            fn from(val: &$ty) -> Self {
                (*val).into()
            }
        }

        impl From<i32> for $ty {
            fn from(val: i32) -> Self {
                (val as u32).into()
            }
        }

        impl From<&i32> for $ty {
            fn from(val: &i32) -> Self {
                (*val).into()
            }
        }

        impl From<$ty> for i32 {
            fn from(val: $ty) -> Self {
                val as i32
            }
        }

        impl From<&$ty> for i32 {
            fn from(val: &$ty) -> Self {
                (*val).into()
            }
        }

        impl From<&$ty> for $crate::xfs::value::XfsValue {
            fn from(val: &$ty) -> Self {
                Self::new().with_i4(val.into())
            }
        }

        impl From<$ty> for $crate::xfs::value::XfsValue {
            fn from(val: $ty) -> Self {
                (&val).into()
            }
        }

        impl From<&$ty> for $crate::xfs::xfs_struct::XfsMember {
            fn from(val: &$ty) -> Self {
                $crate::xfs::xfs_struct::XfsMember::create($ty::xfs_name(), val.into())
            }
        }

        impl From<$ty> for $crate::xfs::xfs_struct::XfsMember {
            fn from(val: $ty) -> Self {
                (&val).into()
            }
        }

        impl TryFrom<&$crate::xfs::xfs_struct::XfsMember> for $ty {
            type Error = $crate::Error;

            fn try_from(val: &$crate::xfs::xfs_struct::XfsMember) -> $crate::Result<Self> {
                if val.name() == Self::xfs_name() && val.value().i4().is_some() {
                    Ok(val.value().i4().unwrap_or(&0i32).into())
                } else {
                    let name = $ty::xfs_name();
                    Err($crate::Error::Xfs(format!(
                        "Expected {name} XfsMember, have: {val}"
                    )))
                }
            }
        }

        impl TryFrom<$crate::xfs::xfs_struct::XfsMember> for $ty {
            type Error = $crate::Error;

            fn try_from(val: $crate::xfs::xfs_struct::XfsMember) -> $crate::Result<Self> {
                (&val).try_into()
            }
        }
    };
}

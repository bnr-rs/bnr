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
            /// Gets the inner representation.
            pub const fn inner(&self) -> u32 {
                *self as u32
            }

            /// Converts into the inner representation.
            pub fn into_inner(self) -> u32 {
                self as u32
            }
        }

        $crate::impl_xfs_i4!($ty, $name);
    };
}

/// Creates a new XFS `i4` type.
#[macro_export]
macro_rules! create_xfs_i4 {
    ($ty:ident, $name:expr, $doc:expr) => {
        ::paste::paste! {
            #[doc = $doc]
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ::serde::Deserialize, ::serde::Serialize)]
            pub struct $ty(u32);

            impl $ty {
                #[doc = "Creates a new `" $ty "`."]
                pub const fn new() -> Self {
                    Self(0)
                }

                #[doc = "Creates a new `" $ty "` from the provided parameter."]
                pub const fn create(val: u32) -> Self {
                    Self(val)
                }

                #[doc = "Gets the inner representation of `" $ty "`."]
                pub const fn inner(&self) -> u32 {
                    self.0
                }

                #[doc = "Sets the inner representation of `" $ty "`."]
                pub fn set_inner(&mut self, val: u32) {
                    self.0 = val;
                }

                #[doc = "Converts into the inner representation of `" $ty "`."]
                pub fn into_inner(self) -> u32 {
                    self.0
                }
            }

            $crate::impl_xfs_i4!($ty, $name);

            impl ::std::fmt::Display for $ty {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{}", self.inner())
                }
            }
        }
    }
}

/// Common functionality for XFS `i4` types.
#[macro_export]
macro_rules! impl_xfs_i4 {
    ($ty:ident, $name:expr) => {
        impl $ty {
            /// Gets the [XfsMember](crate::xfs::xfs_struct::XfsMember) name.
            pub const fn xfs_name() -> &'static str {
                $name
            }
        }

        impl From<u32> for $ty {
            fn from(val: u32) -> Self {
                Self::create(val)
            }
        }

        impl From<&u32> for $ty {
            fn from(val: &u32) -> Self {
                (*val).into()
            }
        }

        impl From<$ty> for u32 {
            fn from(val: $ty) -> Self {
                val.inner()
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
                val.inner() as i32
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

        impl TryFrom<&$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: &$crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                let name = $ty::xfs_name();
                Ok(val
                    .i4()
                    .ok_or($crate::Error::Xfs(format!(
                        "Expected {name} XfsValue, have: {val}"
                    )))?
                    .into())
            }
        }

        impl TryFrom<$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: $crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                (&val).try_into()
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
                let name = $ty::xfs_name();
                match (val.name(), val.value().i4()) {
                    (n, Some(v)) if n == name => Ok(v.into()),
                    _ => Err($crate::Error::Xfs(format!(
                        "Expected {name} XfsMember, have: {val}"
                    ))),
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

/// Common functionality for XFS `int` types.
#[macro_export]
macro_rules! impl_xfs_int {
    ($ty:ident, $name:expr) => {
        impl $ty {
            /// Gets the [XfsMember](crate::xfs::xfs_struct::XfsMember) name.
            pub const fn xfs_name() -> &'static str {
                $name
            }
        }

        impl From<u64> for $ty {
            fn from(val: u64) -> Self {
                Self::create(val)
            }
        }

        impl From<&u64> for $ty {
            fn from(val: &u64) -> Self {
                (*val).into()
            }
        }

        impl From<&$ty> for u64 {
            fn from(val: &$ty) -> Self {
                val.inner()
            }
        }

        impl From<$ty> for u64 {
            fn from(val: $ty) -> Self {
                val.into_inner()
            }
        }

        impl From<i64> for $ty {
            fn from(val: i64) -> Self {
                (val as u64).into()
            }
        }

        impl From<&i64> for $ty {
            fn from(val: &i64) -> Self {
                (*val).into()
            }
        }

        impl From<$ty> for i64 {
            fn from(val: $ty) -> Self {
                val.into_inner() as i64
            }
        }

        impl From<&$ty> for i64 {
            fn from(val: &$ty) -> Self {
                val.inner() as i64
            }
        }

        impl From<&$ty> for $crate::xfs::value::XfsValue {
            fn from(val: &$ty) -> Self {
                Self::new().with_int(val.into())
            }
        }

        impl From<$ty> for $crate::xfs::value::XfsValue {
            fn from(val: $ty) -> Self {
                Self::new().with_int(val.into())
            }
        }

        impl TryFrom<&$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: &$crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                let name = $ty::xfs_name();
                Ok(val
                    .int()
                    .ok_or($crate::Error::Xfs(format!(
                        "Expected {name} XfsValue, have: {val}"
                    )))?
                    .into())
            }
        }

        impl TryFrom<$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: $crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                (&val).try_into()
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
                let name = $ty::xfs_name();
                match (val.name(), val.value().int()) {
                    (n, Some(v)) if n == name => Ok(v.into()),
                    _ => Err($crate::Error::Xfs(format!(
                        "Expected {name} XfsMember, have: {val}"
                    ))),
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

/// Creates a new XFS boolean type.
#[macro_export]
macro_rules! create_xfs_bool {
    ($ty:ident, $name:expr, $doc:expr) => {
        ::paste::paste! {
            #[doc = $doc]
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq, ::serde::Deserialize, ::serde::Serialize)]
            pub struct $ty(bool);

            impl $ty {
                #[doc = "Creates a new `" $ty "`."]
                pub const fn new() -> Self {
                    Self(false)
                }

                #[doc = "Creates a new `" $ty "` from the provided parameter."]
                pub const fn create(val: bool) -> Self {
                    Self(val)
                }

                #[doc = "Gets the inner representation of `" $ty "`."]
                pub const fn inner(&self) -> bool {
                    self.0
                }

                #[doc = "Sets the inner representation of `" $ty "`."]
                pub fn set_inner(&mut self, val: bool) {
                    self.0 = val;
                }

                #[doc = "Converts into the inner representation of `" $ty "`."]
                pub fn into_inner(self) -> bool {
                    self.0
                }
            }

            $crate::impl_xfs_bool!($ty, $name);

            impl ::std::fmt::Display for $ty {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{}", self.inner())
                }
            }
        }
    }
}

/// Common functionality for XFS `boolean` types.
#[macro_export]
macro_rules! impl_xfs_bool {
    ($ty:ident, $name:expr) => {
        impl $ty {
            /// Gets the [XfsMember](crate::xfs::xfs_struct::XfsMember) name.
            pub const fn xfs_name() -> &'static str {
                $name
            }
        }

        impl From<bool> for $ty {
            fn from(val: bool) -> Self {
                Self::create(val)
            }
        }

        impl From<&bool> for $ty {
            fn from(val: &bool) -> Self {
                (*val).into()
            }
        }

        impl From<u8> for $ty {
            fn from(val: u8) -> Self {
                Self::create(val != 0)
            }
        }

        impl From<&u8> for $ty {
            fn from(val: &u8) -> Self {
                (*val).into()
            }
        }

        impl From<&$ty> for bool {
            fn from(val: &$ty) -> Self {
                val.inner()
            }
        }

        impl From<$ty> for bool {
            fn from(val: $ty) -> Self {
                (&val).into()
            }
        }

        impl From<&$ty> for u8 {
            fn from(val: &$ty) -> Self {
                val.inner() as u8
            }
        }

        impl From<$ty> for u8 {
            fn from(val: $ty) -> Self {
                (&val).into()
            }
        }

        impl From<&$ty> for $crate::xfs::value::XfsValue {
            fn from(val: &$ty) -> Self {
                Self::new().with_boolean(val.into())
            }
        }

        impl From<$ty> for $crate::xfs::value::XfsValue {
            fn from(val: $ty) -> Self {
                (&val).into()
            }
        }

        impl TryFrom<&$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: &$crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                let name = $ty::xfs_name();
                Ok(val
                    .boolean()
                    .ok_or($crate::Error::Xfs(format!(
                        "Expected {name} XfsValue, have: {val}"
                    )))?
                    .into())
            }
        }

        impl TryFrom<$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: $crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                (&val).try_into()
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
                let name = $ty::xfs_name();
                match (val.name(), val.value().boolean()) {
                    (n, Some(v)) if n == name => Ok(v.into()),
                    _ => Err($crate::Error::Xfs(format!(
                        "Expected {name} XfsMember, have: {val}"
                    ))),
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

/// Creates an XFS `struct` type.
///
/// ## Parameters:
///
/// - `$ty`: the type name of the Rust struct.
/// - `$name`: the XFS name of the Rust struct.
/// - `[$field_name: $field_ty]`: list of the Rust struct's field names and types.
///
/// **NOTE** all fields must implement `xfs_name`, and convert to/from
/// [XfsMember](crate::xfs::xfs_struct::XfsMember).
#[macro_export]
macro_rules! create_xfs_struct {
    ($ty:ident, $name:expr, [$($field_name:ident: $field_ty:ident),*], $doc:expr) => {
        ::paste::paste! {
            #[doc = $doc]
            #[repr(C)]
            #[derive(Clone, Debug, PartialEq)]
            pub struct $ty {
                $($field_name: $field_ty),*
            }

            impl $ty {
                #[doc = "Creates a new [" $ty "]."]
                pub const fn new() -> Self {
                    Self {
                        $($field_name: $field_ty::new()),*
                    }
                }

                $(
                #[doc = "Gets the [" $field_ty "] for [" $ty "]."]
                pub const fn $field_name(&self) -> &$field_ty {
                    &self.$field_name
                }

                #[doc = "Sets the [" $field_ty "] for [" $ty "]."]
                pub fn [<set_ $field_name>](&mut self, val: $field_ty) {
                    self.$field_name = val;
                }

                #[doc = "Builder function that sets the [" $field_ty "] for [" $ty "]."]
                pub fn [<with_ $field_name>](mut self, val: $field_ty) -> Self {
                    self.[<set_ $field_name>](val);
                    self
                }
                )*
            }

            impl ::std::fmt::Display for $ty {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{{")?;
                    write!(f, r#""name": "{}""#, $name)?;
                    $(
                    write!(f, r#","{}": {}"#, stringify!($field_name), self.$field_name)?;
                    )*
                    write!(f, "}}")
                }
            }

            impl Default for $ty {
                fn default() -> Self {
                    $ty::new()
                }
            }

            $crate::impl_xfs_struct!($ty, $name, [$($field_name: $field_ty),*]);
        }
    }
}

/// Common functionality for XFS `struct` types.
///
/// ## Parameters:
///
/// - `$ty`: the type name of the Rust struct.
/// - `$name`: the XFS name of the Rust struct.
/// - `[$field_name: $field_ty]`: list of the Rust struct's field names and types.
///
/// **NOTE** all fields must implement `xfs_name`, and convert to/from
/// [XfsMember](crate::xfs::xfs_struct::XfsMember).
#[macro_export]
macro_rules! impl_xfs_struct {
    ($ty:ident, $name:expr, [$($field_name:ident: $field_ty:ident),*]) => {
        impl $ty {
            /// Gets the [XfsMember](crate::xfs::xfs_struct::XfsMember) name.
            pub const fn xfs_name() -> &'static str {
                $name
            }
        }

        impl From<&$ty> for $crate::xfs::xfs_struct::XfsStruct {
            fn from(val: &$ty) -> Self {
                Self::create([
                    $(val.$field_name.clone().into(),)*
                ])
            }
        }

        impl From<$ty> for $crate::xfs::xfs_struct::XfsStruct {
            fn from(val: $ty) -> Self {
                (&val).into()
            }
        }

        impl TryFrom<&$crate::xfs::xfs_struct::XfsStruct> for $ty {
            type Error = $crate::Error;

            #[allow(clippy::needless_update)]
            fn try_from(val: &$crate::xfs::xfs_struct::XfsStruct) -> $crate::Result<Self> {
                Ok(Self {
                    $(
                        $field_name: match val.find_member($field_ty::xfs_name()) {
                            Ok(m) => m.try_into()?,
                            Err(_err) => {
                                ::log::warn!("Missing member {} from {}", stringify!($field_name), stringify!($ty));
                                $field_ty::new()
                            }
                        },
                    )*
                    ..Default::default()
                })
            }
        }

        impl TryFrom<$crate::xfs::xfs_struct::XfsStruct> for $ty {
            type Error = $crate::Error;

            fn try_from(val: $crate::xfs::xfs_struct::XfsStruct) -> $crate::Result<Self> {
                (&val).try_into()
            }
        }

        impl From<&$ty> for $crate::xfs::value::XfsValue {
            fn from(val: &$ty) -> Self {
                Self::new().with_xfs_struct(val.into())
            }
        }

        impl From<$ty> for $crate::xfs::value::XfsValue {
            fn from(val: $ty) -> Self {
                (&val).into()
            }
        }

        impl TryFrom<&$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: &$crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                let name = $ty::xfs_name();
                val
                    .xfs_struct()
                    .ok_or($crate::Error::Xfs(format!(
                        "Expected {name} XfsValue, have: {val}"
                    )))?
                    .try_into()
            }
        }

        impl TryFrom<$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: $crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                (&val).try_into()
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
                let name = $ty::xfs_name();
                match (val.name(), val.value().xfs_struct()) {
                    (n, Some(v)) if n == name => v.try_into(),
                    _ => Err($crate::Error::Xfs(format!(
                        "Expected {name} XfsMember, have: {val}"
                    ))),
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

#[macro_export]
macro_rules! create_xfs_array {
    ($ty:ident, $name:expr, $item:ident, $len:expr, $default:expr, $doc:expr) => {
        ::paste::paste! {
            #[doc = $doc]
            #[repr(C)]
            #[derive(Clone, Debug, PartialEq)]
            pub struct $ty{
                size: usize,
                items: [$item; $len],
            }

            impl $ty {
                #[doc = "Creates a new [" $ty "]."]
                pub const fn new() -> Self {
                    Self {
                        size: 0,
                        items: [$default; $len],
                    }
                }

                #[doc = "Gets the maximum number of [" $ty "] items."]
                pub const fn max_size() -> usize {
                    $len
                }

                #[doc = "Gets the size of the [" $ty "]."]
                pub const fn size(&self) -> usize {
                    self.size
                }

                #[doc = "Sets the size of the [" $ty "]."]
                #[doc = ""]
                #[doc = "No-op if `val` is larger than [" $ty "]."]
                pub fn set_size(&mut self, val: u32) {
                    let size = val as usize;
                    if size <= $len {
                        self.size = size;
                    }
                }

                #[doc = "Builder function that sets the size of the [" $ty "]."]
                #[doc = ""]
                #[doc = "No-op if `val` is larger than [" $ty "]."]
                pub fn with_size(mut self, val: u32) -> Self {
                    self.set_size(val);
                    self
                }

                #[doc = "Gets a reference to the list of set [" $ty "] items."]
                pub fn items(&self) -> &[$item] {
                    let len = ::std::cmp::min(self.size, $len);
                    self.items[..len].as_ref()
                }

                #[doc = "Sets the list of [" $ty "] items."]
                pub fn set_items(&mut self, val: &[$item]) {
                    let len = ::std::cmp::min(val.len(), $len);
                    self.items[..len]
                        .iter_mut()
                        .zip(val[..len].iter())
                        .for_each(|(dst, src)| *dst = src.clone());
                    self.items[len..]
                        .iter_mut()
                        .for_each(|item| *item = $item::new());
                    self.size = len;
                }

                #[doc = "Builder function that sets the list of [" $ty "] items."]
                pub fn with_items(mut self, val: &[$item]) -> Self {
                    self.set_items(val);
                    self
                }

                #[doc = "Pushes a [" $ty "] onto the end of the list."]
                #[doc = ""]
                #[doc = "No-op if the list is at capacity."]
                pub fn push_item(&mut self, val: $item) {
                    if self.size < $len {
                        self.items[self.size] = val;
                        self.size += 1;
                    }
                }
            }

            impl Default for $ty {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl ::std::fmt::Display for $ty {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, r#"{{"items":["#)?;
                    for (i, item) in self.items().iter().enumerate() {
                        if i != 0 {
                            write!(f, ",")?;
                        }
                        write!(f, "{item}")?;
                    }
                    write!(f, "]}}")
                }
            }

            $crate::impl_xfs_array!($ty, $name);
        }
    };
}

/// Common functionality for XFS `struct` types.
///
/// ## Parameters:
///
/// - `$ty`: the type name of the Rust struct.
/// - `$name`: the XFS name of the Rust struct.
///
/// **NOTE** inner items must convert to/from [XfsValue](crate::xfs::value::XfsValue).
#[macro_export]
macro_rules! impl_xfs_array {
    ($ty:ident, $name:expr) => {
        impl $ty {
            /// Gets the [XfsMember](crate::xfs::xfs_struct::XfsMember) name.
            pub const fn xfs_name() -> &'static str {
                $name
            }
        }

        impl From<&$ty> for $crate::xfs::array::XfsArray {
            fn from(val: &$ty) -> Self {
                Self::create(val.items().iter().map($crate::xfs::value::XfsValue::from))
            }
        }

        impl From<$ty> for $crate::xfs::array::XfsArray {
            fn from(val: $ty) -> Self {
                (&val).into()
            }
        }

        impl TryFrom<&$crate::xfs::array::XfsArray> for $ty {
            type Error = $crate::Error;

            fn try_from(val: &$crate::xfs::array::XfsArray) -> $crate::Result<Self> {
                let data = val.data();
                let mut res = $ty::new();

                // FIXME: this could be an issue for dynamic-length collection types (Vec, etc.).
                // Dynamic-length types initialize to zero length, and would require pushing items from `data`.
                // Currently, all types converting to/from XfsArray are fixed-length.
                let len = ::std::cmp::min($ty::max_size(), data.len());
                for (i, (dst, src)) in res.items[..len]
                    .iter_mut()
                    .zip(data[..len].iter().map(|m| m.inner()))
                    .enumerate()
                {
                    match src.try_into() {
                        Ok(d) => *dst = d,
                        Err(err) => log::warn!("Failed to convert item[{i}]: {err}"),
                    }
                }

                res.set_size(len as u32);

                Ok(res)
            }
        }

        impl TryFrom<$crate::xfs::array::XfsArray> for $ty {
            type Error = $crate::Error;

            fn try_from(val: $crate::xfs::array::XfsArray) -> $crate::Result<Self> {
                (&val).try_into()
            }
        }

        impl From<&$ty> for $crate::xfs::value::XfsValue {
            fn from(val: &$ty) -> Self {
                Self::new().with_array(val.into())
            }
        }

        impl From<$ty> for $crate::xfs::value::XfsValue {
            fn from(val: $ty) -> Self {
                (&val).into()
            }
        }

        impl TryFrom<&$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: &$crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                let name = $ty::xfs_name();
                val.array()
                    .ok_or($crate::Error::Xfs(format!(
                        "Expected {name} XfsValue, have: {val}"
                    )))?
                    .try_into()
            }
        }

        impl TryFrom<$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: $crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                (&val).try_into()
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
                let name = $ty::xfs_name();
                match (val.name(), val.value().array()) {
                    (n, Some(v)) if n == name => v.try_into(),
                    _ => Err($crate::Error::Xfs(format!(
                        "Expected {name} XfsMember, have: {val}"
                    ))),
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

#[macro_export]
macro_rules! create_xfs_string {
    ($ty:ident, $name:expr, $doc:expr) => {
        ::paste::paste! {
            #[doc = $doc]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct $ty(String);

            impl $ty {
                #[doc = "Creates a new [" $ty "]."]
                pub const fn new() -> Self {
                    Self(String::new())
                }

                #[doc = "Gets a reference to the [" $ty "] inner representation."]
                pub fn inner(&self) -> &str {
                    self.0.as_str()
                }

                #[doc = "Sets the [" $ty "] inner representation value."]
                pub fn set_inner(&mut self, val: &str) {
                    self.0 = val.into();
                }

                #[doc = "Builder function that sets the [" $ty "] inner representation value."]
                pub fn with_inner(mut self, val: &str) -> Self {
                    self.set_inner(val);
                    self
                }

                #[doc = "Converts the [" $ty "] into the inner representation."]
                pub fn into_inner(self) -> String {
                    self.0
                }
            }
        }

        impl Default for $ty {
            fn default() -> Self {
                Self::new()
            }
        }

        impl ::std::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, r#""{}""#, self.0)
            }
        }

        $crate::impl_xfs_string!($ty, $name);
    };
}

/// Common functionality for XFS `string` types.
#[macro_export]
macro_rules! impl_xfs_string {
    ($ty:ident, $name:expr) => {
        impl $ty {
            /// Gets the [XfsMember](crate::xfs::xfs_struct::XfsMember) name.
            pub const fn xfs_name() -> &'static str {
                $name
            }
        }

        impl From<&str> for $ty {
            fn from(val: &str) -> Self {
                Self(val.into())
            }
        }

        impl From<String> for $ty {
            fn from(val: String) -> Self {
                Self(val)
            }
        }

        impl From<&$ty> for String {
            fn from(val: &$ty) -> Self {
                val.inner().into()
            }
        }

        impl From<$ty> for String {
            fn from(val: $ty) -> Self {
                val.into_inner()
            }
        }

        impl From<&$ty> for $crate::xfs::value::XfsValue {
            fn from(val: &$ty) -> Self {
                Self::new().with_string(val.inner())
            }
        }

        impl From<$ty> for $crate::xfs::value::XfsValue {
            fn from(val: $ty) -> Self {
                Self::new().with_string(val.into_inner())
            }
        }

        impl TryFrom<&$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: &$crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                Ok(val.string().unwrap_or("").into())
            }
        }

        impl TryFrom<$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: $crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                (&val).try_into()
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
                let name = $ty::xfs_name();
                match (val.name(), val.value().string()) {
                    (n, Some(v)) if n == name => Ok(v.into()),
                    (n, None) if n == name => Ok("".into()),
                    _ => Err($crate::Error::Xfs(format!(
                        "Expected {name} XfsMember, have: {val}"
                    ))),
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

#[macro_export]
macro_rules! create_xfs_date_time {
    ($ty:ident, $name:expr, $doc:expr) => {
        ::paste::paste! {
            #[doc = $doc]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct $ty(String);

            impl $ty {
                #[doc = "Creates a new [" $ty "]."]
                pub const fn new() -> Self {
                    Self(String::new())
                }

                #[doc = "Gets a reference to the [" $ty "] inner representation."]
                pub fn inner(&self) -> &str {
                    self.0.as_str()
                }

                #[doc = "Sets the [" $ty "] inner representation value."]
                pub fn set_inner(&mut self, val: &str) {
                    self.0 = val.into();
                }

                #[doc = "Builder function that sets the [" $ty "] inner representation value."]
                pub fn with_inner(mut self, val: &str) -> Self {
                    self.set_inner(val);
                    self
                }

                #[doc = "Converts the [" $ty "] into the inner representation."]
                pub fn into_inner(self) -> String {
                    self.0
                }
            }
        }

        impl Default for $ty {
            fn default() -> Self {
                Self::new()
            }
        }

        impl TryFrom<&$ty> for ::time::OffsetDateTime {
            type Error = $crate::Error;

            fn try_from(val: &$ty) -> $crate::Result<Self> {
                let date_str = val.inner();
                if date_str.len() < 17 {
                    Err($crate::Error::DateTime(format!(
                        "invalid ISO-8601 DateTime, too short: {date_str}"
                    )))
                } else {
                    let mut date_string = date_str.to_string();

                    date_string.insert(6, '-');
                    date_string.insert(4, '-');

                    date_string += "+00:00";

                    ::log::debug!("Formatted DateTime: {date_string}");

                    Ok(Self::parse(
                        date_string.as_str(),
                        &::time::format_description::well_known::Iso8601::DATE_TIME,
                    )?)
                }
            }
        }

        impl ::std::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, r#""{}""#, self.0)
            }
        }

        $crate::impl_xfs_date_time!($ty, $name);
    };
}

/// Common functionality for XFS `dateTime` types.
#[macro_export]
macro_rules! impl_xfs_date_time {
    ($ty:ident, $name:expr) => {
        impl $ty {
            /// Gets the [XfsMember](crate::xfs::xfs_struct::XfsMember) name.
            pub const fn xfs_name() -> &'static str {
                $name
            }
        }

        impl From<&str> for $ty {
            fn from(val: &str) -> Self {
                Self(val.into())
            }
        }

        impl From<String> for $ty {
            fn from(val: String) -> Self {
                Self(val)
            }
        }

        impl From<&$ty> for String {
            fn from(val: &$ty) -> Self {
                val.inner().into()
            }
        }

        impl From<$ty> for String {
            fn from(val: $ty) -> Self {
                val.into_inner()
            }
        }

        impl From<&$ty> for $crate::xfs::value::XfsValue {
            fn from(val: &$ty) -> Self {
                Self::new().with_date_time(val.inner())
            }
        }

        impl From<$ty> for $crate::xfs::value::XfsValue {
            fn from(val: $ty) -> Self {
                Self::new().with_date_time(val.into_inner())
            }
        }

        impl TryFrom<&$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: &$crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                Ok(val.date_time().unwrap_or("").into())
            }
        }

        impl TryFrom<$crate::xfs::value::XfsValue> for $ty {
            type Error = $crate::Error;

            fn try_from(val: $crate::xfs::value::XfsValue) -> $crate::Result<Self> {
                (&val).try_into()
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
                let name = $ty::xfs_name();
                match (val.name(), val.value().date_time()) {
                    (n, Some(v)) if n == name => Ok(v.into()),
                    (n, None) if n == name => Ok("".into()),
                    _ => Err($crate::Error::Xfs(format!(
                        "Expected {name} XfsMember, have: {val}"
                    ))),
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

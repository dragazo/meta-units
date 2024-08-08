#![forbid(unsafe_code)]
#![no_implicit_prelude]
#![no_std]

/// This type is only needed to bypass a limitation of the current (stable) trait bound system.
/// See: https://github.com/taiki-e/pin-project/issues/102#issuecomment-540472282
pub struct Wrapper<'a, T>(T, ::core::marker::PhantomData<&'a ()>);
impl<'a, T> Wrapper<'a, T> {
    pub fn new(v: T) -> Self {
        Self(v, ::core::marker::PhantomData)
    }
    pub fn raw(&self) -> &T {
        &self.0
    }
}
impl<T: ::core::clone::Clone> ::core::clone::Clone for Wrapper<'_, T> {
    fn clone(&self) -> Self {
        Self(::core::clone::Clone::clone(&self.0), ::core::marker::PhantomData)
    }
}
impl<T: ::core::marker::Copy> ::core::marker::Copy for Wrapper<'_, T> {}
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Wrapper<'_, T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.0, f)
    }
}
impl<T: ::core::fmt::Display> ::core::fmt::Display for Wrapper<'_, T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.0, f)
    }
}
impl<U, T: ::core::cmp::PartialEq<U>> ::core::cmp::PartialEq<Wrapper<'_, U>> for Wrapper<'_, T> {
    fn eq(&self, other: &Wrapper<'_, U>) -> bool {
        ::core::cmp::PartialEq::eq(&self.0, &other.0)
    }
}
impl<T: ::core::cmp::Eq> ::core::cmp::Eq for Wrapper<'_, T> {}
impl<U, T: ::core::cmp::PartialOrd<U>> ::core::cmp::PartialOrd<Wrapper<'_, U>> for Wrapper<'_, T> {
    fn partial_cmp(&self, other: &Wrapper<'_, U>) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}
impl<T: ::core::cmp::Ord> ::core::cmp::Ord for Wrapper<'_, T> {
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}

/// An abstract unit of measure.
pub trait Unit: ::core::clone::Clone {
    /// The raw type used to represent the numeric value of this unit.
    type Raw;
    /// The type this unit is based on (possibly itself).
    type Base: Unit<Raw = Self::Raw, Base = Self::Base>;

    /// Constructs a new instance of this unit from a raw numeric value.
    fn new(v: Self::Raw) -> Self;
    /// Gets the raw numeric value of this unit.
    fn raw(&self) -> &Self::Raw;

    /// Constructs a derived unit from its base unit.
    fn from_base(v: Self::Base) -> Self;
    /// Converts a derived unit into its base unit.
    fn into_base(self) -> Self::Base;

    /// Converts this unit into a compatible unit.
    fn convert<U: Unit<Raw = Self::Raw, Base = Self::Base>>(self) -> U {
        U::from_base(self.into_base())
    }
}

#[macro_export]
macro_rules! make_unit {
    ($vis:vis $name:ident : $t:ty) => {
        $crate::make_unit! { $vis $name : $t, base = $name, from_base = |x| x, into_base = |x| x }
    };
    ($vis:vis $name:ident : $t:ty, base = $base:ty, from_base = $from_base:expr, into_base = $into_base:expr $(,)?) => {
        $vis struct $name($crate::Wrapper<'static, $t>);
        impl $crate::Unit for $name {
            type Raw = $t;
            type Base = $base;

            fn new(v: $t) -> Self {
                Self($crate::Wrapper::new(v))
            }
            fn raw(&self) -> &$t {
                &self.0.raw()
            }

            fn from_base(v: $base) -> Self {
                $from_base(v)
            }
            fn into_base(self) -> $base {
                $into_base(self)
            }
        }
        impl ::core::clone::Clone for $name where for<'a> $crate::Wrapper<'a, $t>: ::core::clone::Clone {
            fn clone(&self) -> Self {
                Self(::core::clone::Clone::clone(&self.0))
            }
        }
        impl ::core::marker::Copy for $name where for<'a> $crate::Wrapper<'a, $t>: ::core::marker::Copy { }
        impl ::core::fmt::Debug for $name where for<'a> $crate::Wrapper<'a, $t>: ::core::fmt::Debug {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl ::core::fmt::Display for $name where for<'a> $crate::Wrapper<'a, $t>: ::core::fmt::Display {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }
        impl<U: $crate::Unit<Raw = $t, Base = $base>> ::core::cmp::PartialEq<U> for $name where for<'a> $crate::Wrapper<'a, $t>: ::core::cmp::PartialEq {
            fn eq(&self, other: &U) -> bool {
                <Self as $crate::Unit>::into_base(::core::clone::Clone::clone(self)).0.eq(&<U as $crate::Unit>::into_base(::core::clone::Clone::clone(other)).0)
            }
        }
    };
}

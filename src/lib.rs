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
impl<T: Clone> ::core::clone::Clone for Wrapper<'_, T> {
    fn clone(&self) -> Self {
        Self(::core::clone::Clone::clone(&self.0), ::core::marker::PhantomData)
    }
}
impl<T: Copy> ::core::marker::Copy for Wrapper<'_, T> {}
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
    fn partial_cmp(&self, other: &Wrapper<'_, U>) -> Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}
impl<T: ::core::cmp::Ord> ::core::cmp::Ord for Wrapper<'_, T> {
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}

/// An abstract unit of measure.
pub trait Unit: ::core::clone::Clone + ::core::convert::From<Self::Base> {
    /// The raw type used to represent the numeric value of this unit.
    type Raw;
    /// The type this unit is based on (possibly itself).
    type Base: Unit + ::core::convert::From<Self>;

    /// Constructs a new instance of this unit from a raw numeric value.
    fn new(v: Self::Raw) -> Self;
    /// Gets the raw numeric value of this unit.
    fn raw(&self) -> &Self::Raw;
}

#[macro_export]
macro_rules! make_unit {
    ($vis:vis $name:ident : $t:ty) => {
        $crate::make_unit! { $vis $name : $t, base = $name }
    };
    ($vis:vis $name:ident : $t:ty, base = $base:ty) => {
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
        }
        impl ::core::clone::Clone for $name where for<'a> $crate::Wrapper<'a, $t>: ::core::clone::Clone {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
        impl ::core::marker::Copy for $name where for<'a> $crate::Wrapper<'a, $t>: ::core::marker::Copy { }
        impl ::core::fmt::Debug for $name where for<'a> $crate::Wrapper<'a, $t>: ::core::fmt::Debug {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.0.fmt(f)
            }
        }
        impl ::core::fmt::Display for $name where for<'a> $crate::Wrapper<'a, $t>: ::core::fmt::Display {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.0.fmt(f)
            }
        }
        // impl<U: $crate::Unit<Raw = $t, Base = $base>> ::core::cmp::PartialEq<U> for $name where for<'a> $crate::Wrapper<'a, $t>: ::core::cmp::PartialEq {
        //     fn eq(&self, other: &U) -> bool {
        //         $crate::Wrapper::new(<$base>::from(self.clone())).eq(&$crate::Wrapper::new(<<U as $crate::Unit>::Base as From<U>>::from(other.clone())))
        //     }
        // }
    };
    ($vis:vis $name:ident : $base:ident, from_base = $from_base:expr, to_base = $to_base:expr $(,)?) => {
        $crate::make_unit! { $vis $name : <$base as $crate::Unit>::Raw, base = <$base as $crate::Unit>::Base }
        impl ::core::convert::From<$base> for $name {
            fn from(v: $base) -> $name {
                $from_base(v)
            }
        }
        impl ::core::convert::From<$name> for $base {
            fn from(v: $name) -> $base {
                $to_base(v)
            }
        }
    };
}

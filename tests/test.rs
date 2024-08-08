#![no_implicit_prelude]

::meta_units::make_unit! { Meters : f64 }
::meta_units::make_unit! {
    Kilometers : f64,
    base = Meters,
    from_base = |x: Meters| <Kilometers as ::meta_units::Unit>::new(::meta_units::Unit::raw(&x) / 1e3),
    into_base = |x: Kilometers| <Meters as ::meta_units::Unit>::new(::meta_units::Unit::raw(&x) * 1e3),
}
::meta_units::make_unit! {
    Megameters : f64,
    base = Meters,
    from_base = |x: Meters| <Megameters as ::meta_units::Unit>::new(::meta_units::Unit::raw(&x) / 1e6),
    into_base = |x: Megameters| <Meters as ::meta_units::Unit>::new(::meta_units::Unit::raw(&x) * 1e6),
}

#[test]
fn test_no_traits() {
    #[derive(Clone)]
    struct Thing;
    ::meta_units::make_unit! { Meters : Thing }
}

#[test]
fn test_basic_1() {
    let x = <Meters as ::meta_units::Unit>::new(10.0);
    let y = ::core::clone::Clone::clone(&x);
    ::std::println!("{} {}", ::meta_units::Unit::raw(&x), ::meta_units::Unit::raw(&y));

    let x = <Meters as ::meta_units::Unit>::new(10.0);
    let y = x;
    ::std::println!("{} {}", ::meta_units::Unit::raw(&x), ::meta_units::Unit::raw(&y));

    ::core::assert_eq!(::std::format!("{x}"), ::std::format!("{}", ::meta_units::Unit::raw(&x)));
    ::core::assert_eq!(::std::format!("{x:?}"), ::std::format!("{:?}", ::meta_units::Unit::raw(&x)));

    ::core::assert_eq!(x, y);
    ::core::assert_eq!(x, <Meters as ::meta_units::Unit>::new(10.0));
    ::core::assert_ne!(x, <Meters as ::meta_units::Unit>::new(10.1));
}

#[test]
fn test_basic_2() {
    let x = <Kilometers as ::meta_units::Unit>::new(10.0);
    let y = ::core::clone::Clone::clone(&x);
    ::std::println!("{} {}", ::meta_units::Unit::raw(&x), ::meta_units::Unit::raw(&y));

    let x = <Kilometers as ::meta_units::Unit>::new(10.0);
    let y = x;
    ::std::println!("{} {}", ::meta_units::Unit::raw(&x), ::meta_units::Unit::raw(&y));

    ::core::assert_eq!(::std::format!("{x}"), ::std::format!("{}", ::meta_units::Unit::raw(&x)));
    ::core::assert_eq!(::std::format!("{x:?}"), ::std::format!("{:?}", ::meta_units::Unit::raw(&x)));

    ::core::assert_eq!(x, y);
    ::core::assert_eq!(x, <Kilometers as ::meta_units::Unit>::new(10.0));
    ::core::assert_ne!(x, <Kilometers as ::meta_units::Unit>::new(10.1));
}

#[test]
fn test_conversion_1() {
    let x = <Meters as ::meta_units::Unit>::new(2000.0);
    let y: Kilometers = <Meters as ::meta_units::Unit>::convert(x);
    let z: Meters = <Kilometers as ::meta_units::Unit>::convert(y);
    ::core::assert_eq!(*::meta_units::Unit::raw(&x), 2000.0);
    ::core::assert_eq!(*::meta_units::Unit::raw(&y), 2.0);
    ::core::assert_eq!(*::meta_units::Unit::raw(&z), 2000.0);

    let x = <Kilometers as ::meta_units::Unit>::new(2000.0);
    let y: Megameters = <Kilometers as ::meta_units::Unit>::convert(x);
    let z: Kilometers = <Megameters as ::meta_units::Unit>::convert(y);
    ::core::assert_eq!(*::meta_units::Unit::raw(&x), 2000.0);
    ::core::assert_eq!(*::meta_units::Unit::raw(&y), 2.0);
    ::core::assert_eq!(*::meta_units::Unit::raw(&z), 2000.0);

    ::core::assert_eq!(<Meters as ::meta_units::Unit>::new(2500.0), <Kilometers as ::meta_units::Unit>::new(2.5));
    ::core::assert_eq!(<Kilometers as ::meta_units::Unit>::new(2500.0), <Megameters as ::meta_units::Unit>::new(2.5));

    ::core::assert_ne!(<Meters as ::meta_units::Unit>::new(2500.0), <Kilometers as ::meta_units::Unit>::new(2.51));
    ::core::assert_ne!(<Kilometers as ::meta_units::Unit>::new(2500.0), <Megameters as ::meta_units::Unit>::new(2.51));
}

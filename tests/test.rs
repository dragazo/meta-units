meta_units::make_unit! { Meters : f64 }
meta_units::make_unit! {
    Kilometers : Meters,
    from_base = |x: Meters| <Kilometers as meta_units::Unit>::new(meta_units::Unit::raw(&x) / 1000.0),
    to_base = |x: Kilometers| <Meters as meta_units::Unit>::new(meta_units::Unit::raw(&x) * 1000.0),
}

#[test]
fn test_no_traits() {
    #[derive(Clone)]
    struct Thing;
    meta_units::make_unit! { Meters : Thing }
}

#[test]
fn test_basic_1() {
    let x = <Meters as meta_units::Unit>::new(10.0);
    let y = x.clone();
    println!("{} {}", meta_units::Unit::raw(&x), meta_units::Unit::raw(&y));

    let x = <Meters as meta_units::Unit>::new(10.0);
    let y = x;
    println!("{} {}", meta_units::Unit::raw(&x), meta_units::Unit::raw(&y));

    assert_eq!(format!("{x}"), format!("{}", meta_units::Unit::raw(&x)));
    assert_eq!(format!("{x:?}"), format!("{:?}", meta_units::Unit::raw(&x)));

    // assert_eq!(x, y);
    // assert_eq!(x, <Meters as meta_units::Unit>::new(10.0));
    // assert_ne!(x, <Meters as meta_units::Unit>::new(10.1));
}

#[test]
fn test_basic_2() {
    let x = <Kilometers as meta_units::Unit>::new(10.0);
    let y = x.clone();
    println!("{} {}", meta_units::Unit::raw(&x), meta_units::Unit::raw(&y));

    let x = <Kilometers as meta_units::Unit>::new(10.0);
    let y = x;
    println!("{} {}", meta_units::Unit::raw(&x), meta_units::Unit::raw(&y));

    assert_eq!(format!("{x}"), format!("{}", meta_units::Unit::raw(&x)));
    assert_eq!(format!("{x:?}"), format!("{:?}", meta_units::Unit::raw(&x)));

    // assert_eq!(x, y);
    // assert_eq!(x, <Kilometers as meta_units::Unit>::new(10.0));
    // assert_ne!(x, <Kilometers as meta_units::Unit>::new(10.1));
}

#[test]
fn test_conversion_1() {
    let x = <Meters as meta_units::Unit>::new(2000.0);
    let y: Kilometers = x.into();
    let z: Meters = y.into();
    assert_eq!(*meta_units::Unit::raw(&x), 2000.0);
    assert_eq!(*meta_units::Unit::raw(&y), 2.0);
    assert_eq!(*meta_units::Unit::raw(&z), 2000.0);

    // assert_eq!(<Meters as meta_units::Unit>::new(2500.0), <Kilometers as meta_units::Unit>::new(2.5));
}

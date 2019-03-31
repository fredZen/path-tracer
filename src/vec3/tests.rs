use super::*;

#[test]
fn test_zero() {
    assert_eq!(Vec3([0., 0., 0.]), Vec3::zero());
}

#[test]
fn test_new() {
    assert_eq!(Vec3([1., 2., 3.]), Vec3::new(1., 2., 3.));
}

#[test]
fn test_squared_length() {
    assert_eq!(3., Vec3::new(1., 1., 1.).squared_length());
}

#[test]
fn test_length() {
    assert_eq!(5., Vec3::new(3., 4., 0.).length());
    assert_eq!(5., Vec3::new(0., 3., 4.).length());
}

#[test]
fn test_make_unit_vector() {
    let mut v = Vec3::new(5., 0., 0.);
    v.make_unit_vector();
    assert_eq!(Vec3::new(1., 0., 0.), v);
}

#[test]
fn test_parse() {
    assert_eq!(Ok(Vec3::new(1., 2., 3.)), " 1  2 3".parse::<Vec3>());
    assert_eq!(Ok(Vec3::new(1.5, 2.5, 3.5)), "1.5 2.5 3.5".parse::<Vec3>());
}

#[test]
fn test_format() {
    assert_eq!("1 2 3", format!("{}", Vec3::new(1., 2., 3.)));
}

#[test]
fn test_neg() {
    assert_eq!(Vec3::new(8., 6., 4.), -Vec3::new(-8., -6., -4.));
}

#[test]
fn test_index() {
    let v = Vec3::new(8., 5., 2.);
    assert_eq!(8., v[0]);
    assert_eq!(5., v[1]);
    assert_eq!(2., v[2]);
}

#[test]
fn test_index_mut() {
    let mut v = Vec3::zero();
    v[0] = -1.;
    v[1] = 17.;
    v[2] = -15.;
    assert_eq!(Vec3::new(-1., 17., -15.), v);
}

#[test]
fn test_add() {
    assert_eq!(
        Vec3::new(14., -9., -3.),
        Vec3::new(2713., 2., -1.) + Vec3::new(-2699., -11., -2.)
    );
}

#[test]
fn test_sub() {
    assert_eq!(
        Vec3::new(4., -2., 5.),
        Vec3::new(43., 18., -7.) - Vec3::new(39., 20., -12.)
    );
}

#[test]
fn test_mul() {
    assert_eq!(
        Vec3::new(18., -12., 14.),
        Vec3::new(-6., -4., 7.) * Vec3::new(-3., 3., 2.)
    );
    assert_eq!(Vec3::new(5., 3., -1.), 0.5 * Vec3::new(10., 6., -2.));
    assert_eq!(Vec3::new(6., 21., 54.), Vec3::new(2., 7., 18.) * 3.);
}

#[test]
fn test_div() {
    assert_eq!(
        Vec3::new(0.5, 12., -7.),
        Vec3::new(26., -6., -21.) / Vec3::new(52., -0.5, 3.)
    );
    assert_eq!(Vec3::new(17., 2., 43.), Vec3::new(119., 14., 301.) / 7.);
}

#[test]
fn test_dot() {
    assert_eq!(32., Vec3::new(1., 2., 3.).dot(Vec3::new(4., 5., 6.)));
}

#[test]
fn test_cross() {
    assert_eq!(
        Vec3::new(0., 0., 1.),
        Vec3::new(1., 0., 0.).cross(Vec3::new(0., 1., 0.))
    );
    assert_eq!(
        Vec3::new(1., 0., 0.),
        Vec3::new(0., 1., 0.).cross(Vec3::new(0., 0., 1.))
    );
    assert_eq!(
        Vec3::new(0., 1., 0.),
        Vec3::new(0., 0., 1.).cross(Vec3::new(1., 0., 0.))
    );
}

#[test]
fn test_add_assign() {
    let mut v = Vec3::new(-15., 3., 7.);
    v += Vec3::new(33., 1., -5.);
    assert_eq!(Vec3::new(18., 4., 2.), v);
}

#[test]
fn test_sub_assign() {
    let mut v = Vec3::new(3., -7., 92.);
    v -= Vec3::new(10., 17., 22.);
    assert_eq!(Vec3::new(-7., -24., 70.), v);
}

#[test]
fn test_mul_assign() {
    let mut v = Vec3::new(2., 3., 4.);
    v *= Vec3::new(0.5, 8., -1.);
    assert_eq!(Vec3::new(1., 24., -4.), v);
    v *= 3.;
    assert_eq!(Vec3::new(3., 72., -12.), v);
}

#[test]
fn test_div_assign() {
    let mut v = Vec3::new(18., 21., 30.);
    v /= Vec3::new(2., 7., 5.);
    assert_eq!(Vec3::new(9., 3., 6.), v);
    v /= 3.;
    assert_eq!(Vec3::new(3., 1., 2.), v);
}

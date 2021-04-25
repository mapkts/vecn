use super::*;

#[test]
fn vec_const_lanes() {
    assert_eq!(T2::<f32>::LANES, 2);
    assert_eq!(T3::<f32>::LANES, 3);
    assert_eq!(T4::<f32>::LANES, 4);
    assert_eq!(T8::<f32>::LANES, 8);
    assert_eq!(N2::<f32>::LANES, 2);
    assert_eq!(N3::<f32>::LANES, 3);
    assert_eq!(N4::<f32>::LANES, 4);
    assert_eq!(N8::<f32>::LANES, 8);

    assert_eq!(T2xf32::LANES, 2);
    assert_eq!(T3xf32::LANES, 3);
    assert_eq!(T4xf32::LANES, 4);
    assert_eq!(T8xf32::LANES, 8);
    assert_eq!(N2xf32::LANES, 2);
    assert_eq!(N3xf32::LANES, 3);
    assert_eq!(N4xf32::LANES, 4);
    assert_eq!(N8xf32::LANES, 8);
}

#[test]
fn vec_const_zero() {
    assert_eq!(T2xf32::ZERO, new!(T2xf32 @ 0.));
    assert_eq!(T3xf32::ZERO, new!(T3xf32 @ 0.));
    assert_eq!(T4xf32::ZERO, new!(T4xf32 @ 0.));
    assert_eq!(T8xf32::ZERO, new!(T8xf32 @ 0.));
    assert_eq!(N2xf32::ZERO, new!(N2xf32 @ 0.));
    assert_eq!(N3xf32::ZERO, new!(N3xf32 @ 0.));
    assert_eq!(N4xf32::ZERO, new!(N4xf32 @ 0.));
    assert_eq!(N8xf32::ZERO, new!(N8xf32 @ 0.));
}

#[test]
fn vec_const_one() {
    assert_eq!(T2xf32::ONE, new!(T2xf32 @ 1.));
    assert_eq!(T3xf32::ONE, new!(T3xf32 @ 1.));
    assert_eq!(T4xf32::ONE, new!(T4xf32 @ 1.));
    assert_eq!(T8xf32::ONE, new!(T8xf32 @ 1.));
    assert_eq!(N2xf32::ONE, new!(N2xf32 @ 1.));
    assert_eq!(N3xf32::ONE, new!(N3xf32 @ 1.));
    assert_eq!(N4xf32::ONE, new!(N4xf32 @ 1.));
    assert_eq!(N8xf32::ONE, new!(N8xf32 @ 1.));
}

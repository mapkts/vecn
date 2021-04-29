use super::*;

#[test]
fn vec_fn_new() {
    assert_eq!(
        T8::<u32>::new(1, 2, 3, 4, 5, 6, 7, 8),
        T8(1, 2, 3, 4, 5, 6, 7, 8)
    );
    assert_eq!(
        N8::<u32>::new(1, 2, 3, 4, 5, 6, 7, 8),
        N8 {
            x: 1,
            y: 2,
            z: 3,
            w: 4,
            a: 5,
            b: 6,
            c: 7,
            d: 8
        }
    );

    assert_eq!(
        T8xu32::new(1, 2, 3, 4, 5, 6, 7, 8),
        T8xu32(1, 2, 3, 4, 5, 6, 7, 8)
    );
    assert_eq!(
        N8xu32::new(1, 2, 3, 4, 5, 6, 7, 8),
        N8xu32 {
            x: 1,
            y: 2,
            z: 3,
            w: 4,
            a: 5,
            b: 6,
            c: 7,
            d: 8
        }
    );
}

#[test]
fn vec_fn_splat() {
    assert_eq!(T8::<u32>::splat(1), T8(1, 1, 1, 1, 1, 1, 1, 1));
    assert_eq!(
        N8::<u32>::splat(1),
        N8 {
            x: 1,
            y: 1,
            z: 1,
            w: 1,
            a: 1,
            b: 1,
            c: 1,
            d: 1
        }
    );

    assert_eq!(T8xu32::splat(1), T8xu32(1, 1, 1, 1, 1, 1, 1, 1));
    assert_eq!(
        N8xu32::splat(1),
        N8xu32 {
            x: 1,
            y: 1,
            z: 1,
            w: 1,
            a: 1,
            b: 1,
            c: 1,
            d: 1
        }
    );
}

#[test]
fn vec_fn_unit() {
    assert_eq!(T4::<u32>::unit_0(), T4::new(1, 0, 0, 0));
    assert_eq!(T4::<u32>::unit_1(), T4::new(0, 1, 0, 0));
    assert_eq!(T4::<u32>::unit_2(), T4::new(0, 0, 1, 0));
    assert_eq!(T4::<u32>::unit_3(), T4::new(0, 0, 0, 1));

    assert_eq!(N4::<u32>::unit_x(), N4::new(1, 0, 0, 0));
    assert_eq!(N4::<u32>::unit_y(), N4::new(0, 1, 0, 0));
    assert_eq!(N4::<u32>::unit_z(), N4::new(0, 0, 1, 0));
    assert_eq!(N4::<u32>::unit_w(), N4::new(0, 0, 0, 1));

    assert_eq!(T4xu32::unit_0(), T4xu32::new(1, 0, 0, 0));
    assert_eq!(T4xu32::unit_1(), T4xu32::new(0, 1, 0, 0));
    assert_eq!(T4xu32::unit_2(), T4xu32::new(0, 0, 1, 0));
    assert_eq!(T4xu32::unit_3(), T4xu32::new(0, 0, 0, 1));

    assert_eq!(N4xu32::unit_x(), N4xu32::new(1, 0, 0, 0));
    assert_eq!(N4xu32::unit_y(), N4xu32::new(0, 1, 0, 0));
    assert_eq!(N4xu32::unit_z(), N4xu32::new(0, 0, 1, 0));
    assert_eq!(N4xu32::unit_w(), N4xu32::new(0, 0, 0, 1));
}

#[test]
fn vec_fn_sum() {
    assert_eq!(T8::<u32>::new(1, 2, 3, 4, 5, 6, 7, 8).sum(), 36);
    assert_eq!(N8::<u32>::new(1, 2, 3, 4, 5, 6, 7, 8).sum(), 36);

    assert_eq!(T8xu32::new(1, 2, 3, 4, 5, 6, 7, 8).sum(), 36);
    assert_eq!(N8xu32::new(1, 2, 3, 4, 5, 6, 7, 8).sum(), 36);
}

#[test]
fn vec_fn_product() {
    assert_eq!(T8::<u32>::new(1, 2, 3, 4, 5, 6, 7, 8).product(), 40320);
    assert_eq!(N8::<u32>::new(1, 2, 3, 4, 5, 6, 7, 8).product(), 40320);

    assert_eq!(T8xu32::new(1, 2, 3, 4, 5, 6, 7, 8).product(), 40320);
    assert_eq!(N8xu32::new(1, 2, 3, 4, 5, 6, 7, 8).product(), 40320);
}

#[test]
fn vec_fn_dot() {
    assert_eq!(
        T8::<u32>::new(1, 2, 3, 4, 5, 6, 7, 8).dot(T8::<u32>::new(1, 2, 3, 4, 5, 6, 7, 8)),
        204
    );
    assert_eq!(
        N8::<u32>::new(1, 2, 3, 4, 5, 6, 7, 8).dot(N8::<u32>::new(1, 2, 3, 4, 5, 6, 7, 8)),
        204
    );

    assert_eq!(
        T8xu32::new(1, 2, 3, 4, 5, 6, 7, 8).dot(T8xu32::new(1, 2, 3, 4, 5, 6, 7, 8)),
        204
    );
    assert_eq!(
        N8xu32::new(1, 2, 3, 4, 5, 6, 7, 8).dot(N8xu32::new(1, 2, 3, 4, 5, 6, 7, 8)),
        204
    );
}

#[test]
fn vec_fn_length_squared() {
    assert_eq!(
        T4::<u32>::new(1, 2, 3, 4).length_squared(),
        1 + (2 * 2) + (3 * 3) + (4 * 4)
    );
    assert_eq!(
        N4::<u32>::new(1, 2, 3, 4).length_squared(),
        1 + (2 * 2) + (3 * 3) + (4 * 4)
    );

    assert_eq!(
        T4xu32::new(1, 2, 3, 4).length_squared(),
        1 + (2 * 2) + (3 * 3) + (4 * 4)
    );
    assert_eq!(
        N4xu32::new(1, 2, 3, 4).length_squared(),
        1 + (2 * 2) + (3 * 3) + (4 * 4)
    );

    assert_eq!(T2::<u32>::new(3, 4).length_squared(), (3 * 3) + (4 * 4));
    assert_eq!(N2::<u32>::new(3, 4).length_squared(), (3 * 3) + (4 * 4));

    assert_eq!(T2xu32::new(3, 4).length_squared(), (3 * 3) + (4 * 4));
    assert_eq!(N2xu32::new(3, 4).length_squared(), (3 * 3) + (4 * 4));
}

#[test]
fn vec_fn_length() {
    assert!((T4::<f32>::new(2., 4., 4., 0.).length() - 6.).abs() < f32::EPSILON);
    assert!((N4::<f32>::new(2., 4., 4., 0.).length() - 6.).abs() < f32::EPSILON);
    assert!((T4xf32::new(2., 4., 4., 0.).length() - 6.).abs() < f32::EPSILON);
    assert!((N4xf32::new(2., 4., 4., 0.).length() - 6.).abs() < f32::EPSILON);

    assert!((T3::<f32>::new(2., 4., 4.).length() - 6.).abs() < f32::EPSILON);
    assert!((N3::<f32>::new(2., 4., 4.).length() - 6.).abs() < f32::EPSILON);
    assert!((T3xf32::new(2., 4., 4.).length() - 6.).abs() < f32::EPSILON);
    assert!((N3xf32::new(2., 4., 4.).length() - 6.).abs() < f32::EPSILON);

    assert!((N2::<f32>::new(3., 4.).length() - 5.).abs() < f32::EPSILON);
    assert!((T2::<f32>::new(3., 4.).length() - 5.).abs() < f32::EPSILON);
    assert!((T2xf32::new(3., 4.).length() - 5.).abs() < f32::EPSILON);
    assert!((N2xf32::new(3., 4.).length() - 5.).abs() < f32::EPSILON);
}

#[test]
fn vec_fn_length_recip() {
    assert!((T4::<f32>::new(2., 4., 4., 0.).length_recip() - 1. / 6.).abs() < f32::EPSILON);
    assert!((N4::<f32>::new(2., 4., 4., 0.).length_recip() - 1. / 6.).abs() < f32::EPSILON);
    assert!((T4xf32::new(2., 4., 4., 0.).length_recip() - 1. / 6.).abs() < f32::EPSILON);
    assert!((N4xf32::new(2., 4., 4., 0.).length_recip() - 1. / 6.).abs() < f32::EPSILON);

    assert!((T3::<f32>::new(2., 4., 4.).length_recip() - 1. / 6.).abs() < f32::EPSILON);
    assert!((N3::<f32>::new(2., 4., 4.).length_recip() - 1. / 6.).abs() < f32::EPSILON);
    assert!((T3xf32::new(2., 4., 4.).length_recip() - 1. / 6.).abs() < f32::EPSILON);
    assert!((N3xf32::new(2., 4., 4.).length_recip() - 1. / 6.).abs() < f32::EPSILON);

    assert!((N2::<f32>::new(3., 4.).length_recip() - 1. / 5.).abs() < f32::EPSILON);
    assert!((T2::<f32>::new(3., 4.).length_recip() - 1. / 5.).abs() < f32::EPSILON);
    assert!((T2xf32::new(3., 4.).length_recip() - 1. / 5.).abs() < f32::EPSILON);
    assert!((N2xf32::new(3., 4.).length_recip() - 1. / 5.).abs() < f32::EPSILON);
}

#[test]
fn vec_fn_distance_squared() {
    assert!(
        (T4::<f32>::new(1., 2., 3., 4.).distance_squared(T4::<f32>::new(2., 3., 4., 5.)) - 4.)
            .abs()
            < f32::EPSILON
    );
    assert!(
        (N4::<f32>::new(1., 2., 3., 4.).distance_squared(N4::<f32>::new(2., 3., 4., 5.)) - 4.)
            .abs()
            < f32::EPSILON
    );

    assert!(
        (T4xf32::new(1., 2., 3., 4.).distance_squared(T4xf32::new(2., 3., 4., 5.)) - 4.).abs()
            < f32::EPSILON
    );
    assert!(
        (N4xf32::new(1., 2., 3., 4.).distance_squared(N4xf32::new(2., 3., 4., 5.)) - 4.).abs()
            < f32::EPSILON
    );

    assert!(
        (T2::<f32>::new(3., 4.).distance_squared(T2::<f32>::new(0., 5.)) - 10.).abs()
            < f32::EPSILON
    );
    assert!(
        (N2::<f32>::new(3., 4.).distance_squared(N2::<f32>::new(0., 5.)) - 10.).abs()
            < f32::EPSILON
    );

    assert!((T2xf32::new(3., 4.).distance_squared(T2xf32::new(0., 5.)) - 10.).abs() < f32::EPSILON);
    assert!((N2xf32::new(3., 4.).distance_squared(N2xf32::new(0., 5.)) - 10.).abs() < f32::EPSILON);
}

#[test]
fn vec_fn_distance() {
    assert!(
        (T4::<f32>::new(1., 2., 3., 4.).distance(T4::<f32>::new(2., 3., 4., 5.)) - 2.).abs()
            < f32::EPSILON
    );
    assert!(
        (N4::<f32>::new(1., 2., 3., 4.).distance(N4::<f32>::new(2., 3., 4., 5.)) - 2.).abs()
            < f32::EPSILON
    );

    assert!(
        (T4xf32::new(1., 2., 3., 4.).distance(T4xf32::new(2., 3., 4., 5.)) - 2.).abs()
            < f32::EPSILON
    );
    assert!(
        (N4xf32::new(1., 2., 3., 4.).distance(N4xf32::new(2., 3., 4., 5.)) - 2.).abs()
            < f32::EPSILON
    );

    assert!(
        (T2::<f32>::new(3., 4.).distance(T2::<f32>::new(0., 5.)) - f32::sqrt(10.)).abs()
            < f32::EPSILON
    );
    assert!(
        (N2::<f32>::new(3., 4.).distance(N2::<f32>::new(0., 5.)) - f32::sqrt(10.)).abs()
            < f32::EPSILON
    );

    assert!(
        (T2xf32::new(3., 4.).distance(T2xf32::new(0., 5.)) - f32::sqrt(10.)).abs() < f32::EPSILON
    );
    assert!(
        (N2xf32::new(3., 4.).distance(N2xf32::new(0., 5.)) - f32::sqrt(10.)).abs() < f32::EPSILON
    );
}

#[test]
fn vec_fn_min_elem() {
    assert_eq!(T8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3).min_elem(), 1);
    assert_eq!(N8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3).min_elem(), 1);

    assert_eq!(T8xu32::new(4, 6, 5, 7, 1, 8, 2, 3).min_elem(), 1);
    assert_eq!(N8xu32::new(4, 6, 5, 7, 1, 8, 2, 3).min_elem(), 1);
}

#[test]
fn vec_fn_max_elem() {
    assert_eq!(T8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3).max_elem(), 8);
    assert_eq!(N8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3).max_elem(), 8);

    assert_eq!(T8xu32::new(4, 6, 5, 7, 1, 8, 2, 3).max_elem(), 8);
    assert_eq!(N8xu32::new(4, 6, 5, 7, 1, 8, 2, 3).max_elem(), 8);
}

#[test]
fn vec_fn_min() {
    assert_eq!(
        T8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3).min(T8::<u32>::new(2, 4, 7, 1, 3, 5, 1, 3)),
        T8::<u32>::new(2, 4, 5, 1, 1, 5, 1, 3)
    );
    assert_eq!(
        N8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3).min(N8::<u32>::new(2, 4, 7, 1, 3, 5, 1, 3)),
        N8::<u32>::new(2, 4, 5, 1, 1, 5, 1, 3)
    );

    assert_eq!(
        T8xu32::new(4, 6, 5, 7, 1, 8, 2, 3).min(T8xu32::new(2, 4, 7, 1, 3, 5, 1, 3)),
        T8xu32::new(2, 4, 5, 1, 1, 5, 1, 3)
    );
    assert_eq!(
        N8xu32::new(4, 6, 5, 7, 1, 8, 2, 3).min(N8xu32::new(2, 4, 7, 1, 3, 5, 1, 3)),
        N8xu32::new(2, 4, 5, 1, 1, 5, 1, 3)
    );
}

#[test]
fn vec_fn_max() {
    assert_eq!(
        T8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3).max(T8::<u32>::new(2, 4, 7, 1, 3, 5, 1, 3)),
        T8::<u32>::new(4, 6, 7, 7, 3, 8, 2, 3)
    );
    assert_eq!(
        N8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3).max(N8::<u32>::new(2, 4, 7, 1, 3, 5, 1, 3)),
        N8::<u32>::new(4, 6, 7, 7, 3, 8, 2, 3)
    );

    assert_eq!(
        T8xu32::new(4, 6, 5, 7, 1, 8, 2, 3).max(T8xu32::new(2, 4, 7, 1, 3, 5, 1, 3)),
        T8xu32::new(4, 6, 7, 7, 3, 8, 2, 3)
    );
    assert_eq!(
        N8xu32::new(4, 6, 5, 7, 1, 8, 2, 3).max(N8xu32::new(2, 4, 7, 1, 3, 5, 1, 3)),
        N8xu32::new(4, 6, 7, 7, 3, 8, 2, 3)
    );
}

#[test]
fn vec_fn_map() {
    assert_eq!(
        T8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3).map(|x| x + 1),
        T8::<u32>::new(5, 7, 6, 8, 2, 9, 3, 4)
    );
    assert_eq!(
        N8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3).map(|x| x + 1),
        N8::<u32>::new(5, 7, 6, 8, 2, 9, 3, 4)
    );
}

#[test]
fn vec_fn_apply() {
    let mut t8 = T8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3);
    let mut n8 = N8::<u32>::new(4, 6, 5, 7, 1, 8, 2, 3);
    t8.apply(|x| x * 2);
    n8.apply(|x| x * 2);
    assert_eq!(t8, T8::<u32>::new(8, 12, 10, 14, 2, 16, 4, 6));
    assert_eq!(n8, N8::<u32>::new(8, 12, 10, 14, 2, 16, 4, 6));

    let mut t8 = T8xu32::new(4, 6, 5, 7, 1, 8, 2, 3);
    let mut n8 = N8xu32::new(4, 6, 5, 7, 1, 8, 2, 3);
    t8.apply(|x| x * 2);
    n8.apply(|x| x * 2);
    assert_eq!(t8, T8xu32::new(8, 12, 10, 14, 2, 16, 4, 6));
    assert_eq!(n8, N8xu32::new(8, 12, 10, 14, 2, 16, 4, 6));
}

#[test]
fn vec_fn_abs() {
    let t4 = T4::<i32>::new(-1, 4, -5, 8);
    let n4 = N4::<i32>::new(-1, 4, -5, 8);
    assert_eq!(t4.abs(), T4::<i32>::new(1, 4, 5, 8));
    assert_eq!(n4.abs(), N4::<i32>::new(1, 4, 5, 8));

    let t4 = T4xi32::new(-1, 4, -5, 8);
    let n4 = N4xi32::new(-1, 4, -5, 8);
    assert_eq!(t4.abs(), T4xi32::new(1, 4, 5, 8));
    assert_eq!(n4.abs(), N4xi32::new(1, 4, 5, 8));
}

#[test]
fn vec_fn_normalize() {
    let t4 = T4::<f32>::new(1., 4., 5., 8.);
    let n4 = N4::<f32>::new(1., 4., 5., 8.);
    assert!((t4.normalize() - t4 / t4.length()).sum().abs() < f32::EPSILON);
    assert!((n4.normalize() - n4 / n4.length()).sum().abs() < f32::EPSILON);

    let t4 = T4xf32::new(1., 4., 5., 8.);
    let n4 = N4xf32::new(1., 4., 5., 8.);
    assert!((t4.normalize() - t4 / t4.length()).sum().abs() < f32::EPSILON);
    assert!((n4.normalize() - n4 / n4.length()).sum().abs() < f32::EPSILON);
}

#[test]
fn vec_fn_clamp() {
    let t4 = T4::<f32>::new(-1., 5., -5., 8.);
    let min = T4::<f32>::new(1., 3., -3., 5.);
    let max = T4::<f32>::new(2., 4., 0., 10.);
    let clamped = T4::<f32>::new(1., 4., -3., 8.);
    assert_eq!(t4.clamp(min, max), clamped);

    let n4 = N4::<f32>::new(-1., 5., -5., 8.);
    let min = N4::<f32>::new(1., 3., -3., 5.);
    let max = N4::<f32>::new(2., 4., 0., 10.);
    let clamped = N4::<f32>::new(1., 4., -3., 8.);
    assert_eq!(n4.clamp(min, max), clamped);

    let t4 = T4xf32::new(-1., 5., -5., 8.);
    let min = T4xf32::new(1., 3., -3., 5.);
    let max = T4xf32::new(2., 4., 0., 10.);
    let clamped = T4xf32::new(1., 4., -3., 8.);
    assert_eq!(t4.clamp(min, max), clamped);

    let n4 = N4xf32::new(-1., 5., -5., 8.);
    let min = N4xf32::new(1., 3., -3., 5.);
    let max = N4xf32::new(2., 4., 0., 10.);
    let clamped = N4xf32::new(1., 4., -3., 8.);
    assert_eq!(n4.clamp(min, max), clamped);

    let t4 = T4::<i32>::new(-1, 5, -5, 8);
    let min = T4::<i32>::new(1, 3, -3, 5);
    let max = T4::<i32>::new(2, 4, 0, 10);
    let clamped = T4::<i32>::new(1, 4, -3, 8);
    assert_eq!(t4.clamp(min, max), clamped);

    let n4 = N4::<i32>::new(-1, 5, -5, 8);
    let min = N4::<i32>::new(1, 3, -3, 5);
    let max = N4::<i32>::new(2, 4, 0, 10);
    let clamped = N4::<i32>::new(1, 4, -3, 8);
    assert_eq!(n4.clamp(min, max), clamped);

    let t4 = T4xi32::new(-1, 5, -5, 8);
    let min = T4xi32::new(1, 3, -3, 5);
    let max = T4xi32::new(2, 4, 0, 10);
    let clamped = T4xi32::new(1, 4, -3, 8);
    assert_eq!(t4.clamp(min, max), clamped);

    let n4 = N4xi32::new(-1, 5, -5, 8);
    let min = N4xi32::new(1, 3, -3, 5);
    let max = N4xi32::new(2, 4, 0, 10);
    let clamped = N4xi32::new(1, 4, -3, 8);
    assert_eq!(n4.clamp(min, max), clamped);

    let t4 = T4xu32::new(1, 5, 5, 8);
    let min = T4xu32::new(1, 3, 3, 5);
    let max = T4xu32::new(2, 4, 5, 10);
    let clamped = T4xu32::new(1, 4, 5, 8);
    assert_eq!(t4.clamp(min, max), clamped);

    let n4 = N4xu32::new(1, 5, 5, 8);
    let min = N4xu32::new(1, 3, 3, 5);
    let max = N4xu32::new(2, 4, 5, 10);
    let clamped = N4xu32::new(1, 4, 5, 8);
    assert_eq!(n4.clamp(min, max), clamped);

    let t4 = T4xu32::new(1, 5, 5, 8);
    let min = T4xu32::new(1, 3, 3, 5);
    let max = T4xu32::new(2, 4, 5, 10);
    let clamped = T4xu32::new(1, 4, 5, 8);
    assert_eq!(t4.clamp(min, max), clamped);

    let n4 = N4xu32::new(1, 5, 5, 8);
    let min = N4xu32::new(1, 3, 3, 5);
    let max = N4xu32::new(2, 4, 5, 10);
    let clamped = N4xu32::new(1, 4, 5, 8);
    assert_eq!(n4.clamp(min, max), clamped);
}

#[test]
fn vec_cross() {
    let t1 = T3::<i32>::new(2, 4, 4);
    let t2 = T3::<i32>::new(1, 2, 3);

    let n1 = N3::<i32>::new(2, 4, 4);
    let n2 = N3::<i32>::new(1, 2, 3);

    let cross1: T3<i32> = (4 * 3 - 4 * 2, 4 * 1 - 2 * 3, 2 * 2 - 4 * 1).into();
    let cross2: N3<i32> = (4 * 3 - 4 * 2, 4 * 1 - 2 * 3, 2 * 2 - 4 * 1).into();

    assert_eq!(t1.cross(t2), cross1);
    assert_eq!(n1.cross(n2), cross2);

    let t1 = T3xi32::new(2, 4, 4);
    let t2 = T3xi32::new(1, 2, 3);

    let n1 = N3xi32::new(2, 4, 4);
    let n2 = N3xi32::new(1, 2, 3);

    let cross1: T3xi32 = (4 * 3 - 4 * 2, 4 * 1 - 2 * 3, 2 * 2 - 4 * 1).into();
    let cross2: N3xi32 = (4 * 3 - 4 * 2, 4 * 1 - 2 * 3, 2 * 2 - 4 * 1).into();

    assert_eq!(t1.cross(t2), cross1);
    assert_eq!(n1.cross(n2), cross2);
}

#[test]
fn is_nan() {
    assert!(T8::<f32>::new(1., 2., 3., f32::NAN, 5., 6., 7., 8.).is_nan());
    assert!(N8::<f32>::new(1., 2., 3., f32::NAN, 5., 6., 7., 8.).is_nan());
    assert!(!T8::<f32>::new(1., 2., 3., 4., 5., 6., 7., 8.).is_nan());
    assert!(!N8::<f32>::new(1., 2., 3., 4., 5., 6., 7., 8.).is_nan());

    assert!(T8xf32::new(1., 2., 3., f32::NAN, 5., 6., 7., 8.).is_nan());
    assert!(N8xf32::new(1., 2., 3., f32::NAN, 5., 6., 7., 8.).is_nan());
    assert!(!T8xf32::new(1., 2., 3., 4., 5., 6., 7., 8.).is_nan());
    assert!(!N8xf32::new(1., 2., 3., 4., 5., 6., 7., 8.).is_nan());
}

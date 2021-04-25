use vecn::vector;

#[test]
fn accept_generic_tuple_struct() {
    #[vector]
    struct Vec2<T>(T, T);
    #[vector]
    struct Vec3<T>(T, T, T);
    #[vector]
    struct Vec4<T>(T, T, T, T);
    #[vector]
    struct Vec8<T>(T, T, T, T, T, T, T, T);
}

#[test]
fn accept_primitive_tuple_struct() {
    #[vector]
    struct Vec2(f32, f32);
    #[vector]
    struct Vec3(f32, f32, f32);
    #[vector]
    struct Vec4(f32, f32, f32, f32);
    #[vector]
    struct Vec8(f32, f32, f32, f32, f32, f32, f32, f32);
}

#[test]
fn accept_generic_named_struct() {
    #[vector]
    struct Vec2<T> {
        x: T,
        y: T,
    }

    #[vector]
    struct Vec3<T> {
        x: T,
        y: T,
        z: T,
    }

    #[vector]
    struct Vec4<T> {
        x: T,
        y: T,
        z: T,
        w: T,
    }

    #[vector]
    struct Vec8<T> {
        a: T,
        b: T,
        c: T,
        d: T,
        e: T,
        f: T,
        g: T,
        h: T,
    }
}

#[test]
fn accept_primitive_named_struct() {
    #[vector]
    struct Vec2 {
        x: f32,
        y: f32,
    }

    #[vector]
    struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
    }

    #[vector]
    struct Vec4 {
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    }

    #[vector]
    struct Vec8 {
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
        g: f32,
        h: f32,
    }
}

#[test]
fn accept_pub_keywords() {
    #[vector]
    struct Vec1<T>(pub T, T);

    #[vector]
    struct Vec2<T> {
        pub x: T,
        y: T,
    }
}

#[test]
fn accept_trait_bounds() {
    #[vector]
    struct Vec1<T>
    where
        T: Copy + Clone,
    {
        x: T,
        y: T,
    }

    #[vector]
    struct Vec2<T>
    where
        T: Copy + Clone,
    {
        x: T,
        y: T,
    }
}

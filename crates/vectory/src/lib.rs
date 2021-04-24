use vecn::vector;

#[vector]
/// A generic 2D vector type that represented by tuple struct.
pub struct Vec2<T>(T, T);

#[vector]
/// A generic 3D vector type that represented by named struct.
pub struct Vec3<T>
where
    T: Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

#[vector]
/// A generic 4D vector type that represented by named struct.
pub struct Vec4<T>
where
    T: Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[vector]
/// A 2D vector type with elements of type `u8`.
pub struct V2xu8(pub u8, pub u8);

#[vector]
/// A 3D vector type with elements of type `i32`.
pub struct V3xi32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[vector]
/// A 4D vector type with elements of type `f64`.
pub struct V4xf64 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

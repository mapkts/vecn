// This file is for testing only.
use vecn::vector;

fn main() {}

#[vector]
pub struct Vec1<T>
where
    T: Copy + Clone,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

#[vector]
pub struct Vec2<T>(T, T);

#[vector]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[vector]
pub struct Vec4(u8, u8);

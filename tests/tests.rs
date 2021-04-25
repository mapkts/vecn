#![allow(clippy::too_many_arguments)]

mod compiletest;
mod accepted;
mod consts;
mod methods;

use vecn::vector;

#[macro_export]
macro_rules! new {
    ($ident:ident, $($num:literal),*) => {
        $ident::new($($num),*)
    };
    ($ident:ident @ $num:literal) => {
        $ident::splat($num)
    };
}

macro_rules! tuple_struct {
    ($($ident:ident<$generic:tt>, $($fields:tt),+)*) => {
        $(   
            #[vector]
            #[derive(Debug, Default)]
            pub struct $ident<$generic>($(pub $fields),+);
        )*  
    };
    ($($ident:ident, $($fields:tt),+)*) => {
        $(
            #[vector]
            #[derive(Debug, Default)]
            pub struct $ident($(pub $fields),+);
        )*
    }
}

macro_rules! named_struct {
    ($($ident:ident<$generic:tt>, $($field:tt: $type:tt),+)*) => {
        $(   
            #[vector]
            #[derive(Debug, Default)]
            pub struct $ident<$generic>{$(pub $field: $type),+}
        )*  
    };
    ($($ident:ident, $($field:ident: $type:tt),+)*) => {
        $(
            #[vector]
            #[derive(Debug, Default)]
            pub struct $ident{$(pub $field: $type),+}
        )*
    }
}

tuple_struct! {
    T2<T>, T, T
    T3<T>, T, T, T
    T4<T>, T, T, T, T
    T8<T>, T, T, T, T, T, T, T, T
}

tuple_struct! {
    T2xf32, f32, f32
    T3xf32, f32, f32, f32
    T4xf32, f32, f32, f32, f32
    T8xf32, f32, f32, f32, f32, f32, f32, f32, f32
}

tuple_struct! {
    T2xi32, i32, i32
    T3xi32, i32, i32, i32
    T4xi32, i32, i32, i32, i32
    T8xi32, i32, i32, i32, i32, i32, i32, i32, i32
}

tuple_struct! {
    T2xu32, u32, u32
    T3xu32, u32, u32, u32
    T4xu32, u32, u32, u32, u32
    T8xu32, u32, u32, u32, u32, u32, u32, u32, u32
}

named_struct! {
    N2<T>, x: T, y: T
    N3<T>, x: T, y: T, z: T
    N4<T>, x: T, y: T, z: T, w: T
    N8<T>, x: T, y: T, z: T, w: T, a: T, b: T, c: T, d: T
}

named_struct! {
    N2xf32, x: f32, y: f32
    N3xf32, x: f32, y: f32, z: f32
    N4xf32, x: f32, y: f32, z: f32, w: f32
    N8xf32, x: f32, y: f32, z: f32, w: f32, a: f32, b: f32, c: f32, d: f32
}

named_struct! {
    N2xu32, x: u32, y: u32
    N3xu32, x: u32, y: u32, z: u32
    N4xu32, x: u32, y: u32, z: u32, w: u32
    N8xu32, x: u32, y: u32, z: u32, w: u32, a: u32, b: u32, c: u32, d: u32
}

named_struct! {
    N2xi32, x: i32, y: i32
    N3xi32, x: i32, y: i32, z: i32
    N4xi32, x: i32, y: i32, z: i32, w: i32
    N8xi32, x: i32, y: i32, z: i32, w: i32, a: i32, b: i32, c: i32, d: i32
}


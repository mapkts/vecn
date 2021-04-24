#[vecn::vector]
struct TupleStruct(&f32 , &f32);

#[vecn::vector]
struct NamedStruct {
    x: &f32,
    y: &f32,
}

fn main() {}

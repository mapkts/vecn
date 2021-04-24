#[vecn::vector]
struct TupleStruct<'a>(&'a f32);

#[vecn::vector]
struct NamedStruct<'a> {
    x: &'a f32,
    y: &'a f32,
}

fn main() {}

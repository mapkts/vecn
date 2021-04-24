#[vecn::vector]
struct TupleStruct<const T: usize>(T);

#[vecn::vector]
struct NamedStruct<const T: usize> {
    x: T,
}

fn main() {}

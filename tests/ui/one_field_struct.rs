#[vecn::vector]
struct TupleStruct<T>(T);

#[vecn::vector]
struct NamedStruct<T> {
    x: T,
}

fn main() {}

#[vecn::vector]
struct TupleStruct1<T, U>((T, U), (T, U), (T, U));

#[vecn::vector]
struct NamedStruct1<T, U> {
    x: (T, U),
    y: (T, U),
}

#[vecn::vector]
struct TupleStruct2<T, U>(&(T, U), &(T, U), &(T, U));

#[vecn::vector]
struct NamedStruct2<T, U> {
    x: &(T, U),
    y: &(T, U),
}

fn main() {}

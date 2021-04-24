#[vecn::vector]
struct TupleStruct1(usize, usize);

#[vecn::vector]
struct TupleStruct2(isize, isize);

#[vecn::vector]
struct TupleStruct3(bool, bool);

#[vecn::vector]
struct NamedStruct1 {
    x: usize,
    y: usize,
}

#[vecn::vector]
struct NamedStruct2 {
    x: isize,
    y: isize,
}

#[vecn::vector]
struct NamedStruct3 {
    x: bool,
    y: bool,
}

fn main() {}

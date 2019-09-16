// first item

pub trait TupleFirst {
    type Output;
}

impl<A, B> TupleFirst for (A, B) {
    type Output = A;
}

pub type TupleFirstOutput<T> = <T as TupleFirst>::Output;

// second item

pub trait TupleSecond {
    type Output;
}

impl<A, B> TupleSecond for (A, B) {
    type Output = B;
}

pub type TupleSecondOutput<T> = <T as TupleSecond>::Output;

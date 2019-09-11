pub trait First {
    type Output;
}

impl<A> First for (A,) {
    type Output = A;
}

impl<A, B> First for (A, B) {
    type Output = A;
}

impl<A, B, C> First for (A, B, C) {
    type Output = A;
}

pub trait Second {
    type Output;
}

impl<A, B> Second for (A, B) {
    type Output = B;
}

impl<A, B, C> Second for (A, B, C) {
    type Output = B;
}

pub trait Third {
    type Output;
}

impl <A, B, C> Third for (A, B, C) {
    type Output = C;
}

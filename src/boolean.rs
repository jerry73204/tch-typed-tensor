use typenum::{Cmp, False, IsLess, IsLessOrEqual, True};

// primitives

pub trait Boolean {
    const VALUE: bool;
}

impl Boolean for True {
    const VALUE: bool = true;
}

impl Boolean for False {
    const VALUE: bool = false;
}

// if branch

pub trait If<Condition>
where
    Condition: Boolean,
{
    type Output;
}

impl<Result> If<True> for Result {
    type Output = Result;
}

// if less than

pub trait IfLess<Lhs, Rhs>
where
    Lhs: Cmp<Rhs>,
{
    type Output;
}

impl<Lhs, Rhs, Out> IfLess<Lhs, Rhs> for Out
where
    Lhs: IsLess<Rhs> + Cmp<Rhs>,
    Out: If<<Lhs as IsLess<Rhs>>::Output>,
    <Lhs as IsLess<Rhs>>::Output: Boolean,
{
    type Output = <Out as If<<Lhs as IsLess<Rhs>>::Output>>::Output;
}

// if less than or equal

pub trait IfLessOrEqual<Lhs, Rhs>
where
    Lhs: Cmp<Rhs>,
{
    type Output;
}

impl<Lhs, Rhs, Out> IfLessOrEqual<Lhs, Rhs> for Out
where
    Lhs: IsLessOrEqual<Rhs> + Cmp<Rhs>,
    Out: If<<Lhs as IsLessOrEqual<Rhs>>::Output>,
    <Lhs as IsLessOrEqual<Rhs>>::Output: Boolean,
{
    type Output = <Out as If<<Lhs as IsLessOrEqual<Rhs>>::Output>>::Output;
}

// if-else branch

pub trait IfElse<A, B> {
    type Output;
}

impl<A, B> IfElse<A, B> for True {
    type Output = A;
}

impl<A, B> IfElse<A, B> for False {
    type Output = B;
}

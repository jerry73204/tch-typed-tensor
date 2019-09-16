use crate::tuple::TupleSecondOutput;
use typenum::{False, IsLess, IsLessOrEqual, True};

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

// if exist

pub type IfExistsOutput<Out, Condition> = TupleSecondOutput<(Condition, Out)>;

// if branch

pub trait IfBoolean<Condition>
where
    Condition: Boolean,
{
    type Output;
}

impl<Result> IfBoolean<True> for Result {
    type Output = Result;
}

pub type IfBooleanOutput<Out, Predicate> = <Out as IfBoolean<Predicate>>::Output;

// is less

pub trait BoolIsLess<Rhs>
where
    Self::Output: Boolean,
{
    type Output;
}

impl<Lhs, Rhs> BoolIsLess<Rhs> for Lhs
where
    Lhs: IsLess<Rhs>,
    <Lhs as IsLess<Rhs>>::Output: Boolean,
{
    type Output = <Lhs as IsLess<Rhs>>::Output;
}

pub type BoolIsLessOutput<Lhs, Rhs> = <Lhs as BoolIsLess<Rhs>>::Output;

// is less or equal

pub trait BoolIsLessOrEqual<Rhs>
where
    Self::Output: Boolean,
{
    type Output;
}

impl<Lhs, Rhs> BoolIsLessOrEqual<Rhs> for Lhs
where
    Lhs: IsLessOrEqual<Rhs>,
    <Lhs as IsLessOrEqual<Rhs>>::Output: Boolean,
{
    type Output = <Lhs as IsLessOrEqual<Rhs>>::Output;
}

pub type BoolIsLessOrEqualOutput<Lhs, Rhs> = <Lhs as BoolIsLessOrEqual<Rhs>>::Output;

// if less than

pub trait IfLess<Lhs, Rhs> {
    type Output;
}

impl<Lhs, Rhs, Out> IfLess<Lhs, Rhs> for Out
where
    Lhs: BoolIsLess<Rhs>,
    Out: IfBoolean<BoolIsLessOutput<Lhs, Rhs>>,
{
    type Output = IfBooleanOutput<Out, BoolIsLessOutput<Lhs, Rhs>>;
}

pub type IfLessOutput<Out, Lhs, Rhs> = <Out as IfLess<Lhs, Rhs>>::Output;

// if less than or equal

pub trait IfLessOrEqual<Lhs, Rhs> {
    type Output;
}

impl<Lhs, Rhs, Out> IfLessOrEqual<Lhs, Rhs> for Out
where
    Lhs: BoolIsLessOrEqual<Rhs>,
    Out: IfBoolean<BoolIsLessOrEqualOutput<Lhs, Rhs>>,
{
    type Output = IfBooleanOutput<Out, BoolIsLessOrEqualOutput<Lhs, Rhs>>;
}

pub type IfLessOrEqualOutput<Out, Lhs, Rhs> = <Out as IfLessOrEqual<Lhs, Rhs>>::Output;

// primitives

pub trait TBoolean {
    const VALUE: bool;
}

pub struct TTrue;

impl TBoolean for TTrue {
    const VALUE: bool = true;
}

pub struct TFalse;

impl TBoolean for TFalse {
    const VALUE: bool = false;
}

// boolean not

pub trait Not {
    type Output;
}

impl Not for TTrue {
    type Output = TFalse;
}

impl Not for TFalse {
    type Output = TTrue;
}

// boolean and

pub trait And<Rhs: TBoolean> {
    type Output;
}

impl And<TTrue> for TTrue {
    type Output = TTrue;
}

impl And<TTrue> for TFalse {
    type Output = TFalse;
}

impl And<TFalse> for TTrue {
    type Output = TFalse;
}

impl And<TFalse> for TFalse {
    type Output = TFalse;
}

// boolean or

pub trait Or<Rhs: TBoolean> {
    type Output;
}

impl Or<TTrue> for TTrue {
    type Output = TTrue;
}

impl Or<TTrue> for TFalse {
    type Output = TTrue;
}

impl Or<TFalse> for TTrue {
    type Output = TTrue;
}

impl Or<TFalse> for TFalse {
    type Output = TFalse;
}

// boolean xor

pub trait Xor<Rhs: TBoolean> {
    type Output;
}

impl Xor<TTrue> for TTrue {
    type Output = TFalse;
}

impl Xor<TTrue> for TFalse {
    type Output = TTrue;
}

impl Xor<TFalse> for TTrue {
    type Output = TTrue;
}

impl Xor<TFalse> for TFalse {
    type Output = TFalse;
}

// boolean iff

pub trait Iff<Rhs: TBoolean> {
    type Output;
}

impl<Lhs, Rhs> Iff<Rhs> for Lhs
where
    Lhs: TBoolean + Xor<Rhs>,
    Rhs: TBoolean,
    <Lhs as Xor<Rhs>>::Output: Not,
{
    type Output = <<Lhs as Xor<Rhs>>::Output as Not>::Output;
}

// boolean nand

pub trait Nand<Rhs: TBoolean> {
    type Output;
}

impl<Lhs, Rhs> Nand<Rhs> for Lhs
where
    Lhs: TBoolean + And<Rhs>,
    Rhs: TBoolean,
    <Lhs as And<Rhs>>::Output: Not,
{
    type Output = <<Lhs as And<Rhs>>::Output as Not>::Output;
}

// boolean nand

pub trait Nor<Rhs: TBoolean> {
    type Output;
}

impl<Lhs, Rhs> Nor<Rhs> for Lhs
where
    Lhs: TBoolean + Or<Rhs>,
    Rhs: TBoolean,
    <Lhs as Or<Rhs>>::Output: Not,
{
    type Output = <<Lhs as Or<Rhs>>::Output as Not>::Output;
}

// if branch

pub trait If<A> {
    type Output;
}

impl<A> If<A> for TTrue {
    type Output = A;
}

// if-else branch

pub trait IfElse<A, B> {
    type Output;
}

impl<A, B> IfElse<A, B> for TTrue {
    type Output = A;
}

impl<A, B> IfElse<A, B> for TFalse {
    type Output = B;
}

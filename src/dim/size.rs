use crate::utils::ToInt;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};
use typenum::{Prod, Sum, Unsigned};

// dimension size

/// Represents the size of a dimension
pub trait DimSize {}

/// Represents a known dimensions size.
pub struct Known<Size>
where
    Size: Unsigned,
{
    _phantom: PhantomData<Size>,
}

impl<Size> Known<Size>
where
    Size: Unsigned,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Size> DimSize for Known<Size> where Size: Unsigned {}

/// Represents an unknown dimension size.
pub struct Unknown;

impl Unknown {
    pub fn new() -> Self {
        Self
    }
}

impl DimSize for Unknown {}

// add

impl Add<Unknown> for Unknown {
    type Output = Unknown;

    fn add(self, _rhs: Unknown) -> Self::Output {
        Self::Output::new()
    }
}

impl<Size> Add<Known<Size>> for Unknown
where
    Size: Unsigned,
{
    type Output = Unknown;

    fn add(self, _rhs: Known<Size>) -> Self::Output {
        Self::Output::new()
    }
}

impl<Size> Add<Unknown> for Known<Size>
where
    Size: Unsigned,
{
    type Output = Unknown;

    fn add(self, _rhs: Unknown) -> Self::Output {
        Self::Output::new()
    }
}

impl<LSize, RSize> Add<Known<RSize>> for Known<LSize>
where
    LSize: Unsigned + Add<RSize>,
    RSize: Unsigned,
    Sum<LSize, RSize>: Unsigned,
{
    type Output = Known<Sum<LSize, RSize>>;

    fn add(self, _rhs: Known<RSize>) -> Self::Output {
        Self::Output::new()
    }
}

// mul

impl Mul<Unknown> for Unknown {
    type Output = Unknown;

    fn mul(self, _rhs: Unknown) -> Self::Output {
        Self::Output::new()
    }
}

impl<Size> Mul<Known<Size>> for Unknown
where
    Size: Unsigned,
{
    type Output = Unknown;

    fn mul(self, _rhs: Known<Size>) -> Self::Output {
        Self::Output::new()
    }
}

impl<Size> Mul<Unknown> for Known<Size>
where
    Size: Unsigned,
{
    type Output = Unknown;

    fn mul(self, _rhs: Unknown) -> Self::Output {
        Self::Output::new()
    }
}

impl<LSize, RSize> Mul<Known<RSize>> for Known<LSize>
where
    LSize: Unsigned + Mul<RSize>,
    RSize: Unsigned,
    Prod<LSize, RSize>: Unsigned,
{
    type Output = Known<Prod<LSize, RSize>>;

    fn mul(self, _rhs: Known<RSize>) -> Self::Output {
        Self::Output::new()
    }
}

// to option

/// A trait that converts a [DimSize] to a concrete [Option] of integer type.
pub trait SizeToOption<Output>
where
    Self: DimSize,
{
    fn to_option() -> Option<Output>;
}

impl<Size, Output> SizeToOption<Output> for Known<Size>
where
    Size: Unsigned + ToInt<Output>,
{
    fn to_option() -> Option<Output> {
        Some(Size::to_int())
    }
}

impl<Output> SizeToOption<Output> for Unknown {
    fn to_option() -> Option<Output> {
        None
    }
}

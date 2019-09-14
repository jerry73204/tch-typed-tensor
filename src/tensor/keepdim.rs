use crate::boolean::{TBoolean, TFalse, TTrue};

pub trait KeepDim {}

pub struct DoKeepDim {}

impl KeepDim for DoKeepDim {}

pub struct NoKeepDim {}

impl KeepDim for NoKeepDim {}

pub trait KeepDimOrNot
where
    Self::Output: TBoolean,
{
    type Output;
}

impl KeepDimOrNot for DoKeepDim {
    type Output = TTrue;
}

impl KeepDimOrNot for NoKeepDim {
    type Output = TFalse;
}

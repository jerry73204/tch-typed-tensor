use crate::boolean::Boolean;
use typenum::{False, True};

pub trait KeepDim {}

pub struct DoKeepDim {}

impl KeepDim for DoKeepDim {}

pub struct NoKeepDim {}

impl KeepDim for NoKeepDim {}

pub trait KeepDimOrNot
where
    Self::Output: Boolean,
{
    type Output;
}

impl KeepDimOrNot for DoKeepDim {
    type Output = True;
}

impl KeepDimOrNot for NoKeepDim {
    type Output = False;
}

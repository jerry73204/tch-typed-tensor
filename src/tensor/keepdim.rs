use type_freak::boolean::Boolean;
use typenum::{False, True};

// definitions

pub trait KeepDim {}

pub struct DoKeepDim {}

impl KeepDim for DoKeepDim {}

pub struct NoKeepDim {}

impl KeepDim for NoKeepDim {}

// map to boolean

pub trait KeepDimOrNot
where
    Self: KeepDim,
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

pub type KeepDimOrNotOutput<Keep> = <Keep as KeepDimOrNot>::Output;

mod access;
mod broadcast;
mod flatten;
mod insert;
mod macros;
pub mod marker;
mod matmul;
mod misc;
mod remove;
mod size;

pub use access::*;
pub use broadcast::*;
pub use flatten::*;
pub use insert::*;
pub use macros::*;
pub use matmul::*;
pub use misc::*;
pub use remove::*;
pub use size::*;

use std::marker::PhantomData;
use type_freak::kvlist::{KVCons, KVList, KVNil};

// dimension list

/// Represents the name of dimension.
pub trait DimName {}

/// Represents a list of ordered dimensions.
pub trait DimList
where
    Self::List: KVList,
{
    type List;
}

// end of dim list

/// Represents the end of [DimList].
pub struct DNil;

impl DimList for DNil {
    type List = KVNil;
}

// node of dim list

/// Represents a component of [DimList].
pub struct DCons<Name, Size, Tail>
where
    Name: DimName,
    Size: DimSize,
    Tail: DimList,
{
    _phantom: PhantomData<(Name, Size, Tail)>,
}

impl<Name, Size, Tail> DimList for DCons<Name, Size, Tail>
where
    Name: DimName,
    Size: DimSize,
    Tail: DimList,
{
    type List = KVCons<Name, Size, <Tail as DimList>::List>;
}

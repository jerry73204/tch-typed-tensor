use super::{DCons, DNil, DimList, DimName, DimSize};

// scalar marker

/// Represents a [DimList] with zero length.
pub trait ScalarDim: DimList {}

impl ScalarDim for DNil {}

/// Represents a [DimList] with non-zero length.
pub trait NonScalarDim: DimList {}

impl<Name, Size, Tail> NonScalarDim for DCons<Name, Size, Tail>
where
    Name: DimName,
    Size: DimSize,
    Tail: DimList,
{
}

// vector marker

/// Represents a [DimList] with length of one.
pub trait VectorDim: DimList {}

impl<Name, Size> VectorDim for DCons<Name, Size, DNil>
where
    Name: DimName,
    Size: DimSize,
{
}

/// Represents a [DimList] which length is not one.
pub trait NonVectorDim: DimList {}

impl NonVectorDim for DNil {}

impl<Name1, Size1, Name2, Size2, Tail> NonVectorDim
    for DCons<Name1, Size1, DCons<Name2, Size2, Tail>>
where
    Name1: DimName,
    Size1: DimSize,
    Name2: DimName,
    Size2: DimSize,
    Tail: DimList,
{
}

// matrix marker

/// Represents a [DimList] with length of two.
pub trait MatrixDim: DimList {}

impl<Name1, Size1, Name2, Size2> MatrixDim for DCons<Name1, Size1, DCons<Name2, Size2, DNil>>
where
    Name1: DimName,
    Size1: DimSize,
    Name2: DimName,
    Size2: DimSize,
{
}

/// Represents a [DimList] which length is not two.
pub trait NonMatrixDim: DimList {}

impl NonMatrixDim for DNil {}

impl<Name, Size> NonMatrixDim for DCons<Name, Size, DNil>
where
    Name: DimName,
    Size: DimSize,
{
}

impl<Name1, Size1, Name2, Size2, Name3, Size3, Tail> MatrixDim
    for DCons<Name1, Size1, DCons<Name2, Size2, DCons<Name3, Size3, Tail>>>
where
    Name1: DimName,
    Size1: DimSize,
    Name2: DimName,
    Size2: DimSize,
    Name3: DimName,
    Size3: DimSize,
    Tail: DimList,
{
}

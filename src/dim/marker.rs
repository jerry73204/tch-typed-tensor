use super::{DCons, DNil, Dim, DimList};
use typenum::Unsigned;

// scalar marker

pub trait ScalarDim: DimList {}

impl ScalarDim for DNil {}

pub trait NonScalarDim: DimList {}

impl<D, S, T> NonScalarDim for DCons<D, S, T>
where
    D: Dim,
    S: Unsigned,
    T: DimList,
{
}

// vector marker

pub trait VectorDim: DimList {}

impl<D, S> VectorDim for DCons<D, S, DNil>
where
    D: Dim,
    S: Unsigned,
{
}

pub trait NonVectorDim: DimList {}

impl NonVectorDim for DNil {}

impl<D1, S1, D2, S2, T> NonVectorDim for DCons<D1, S1, DCons<D2, S2, T>>
where
    D1: Dim,
    S1: Unsigned,
    D2: Dim,
    S2: Unsigned,
    T: DimList,
{
}

// vector marker

pub trait MatrixDim: DimList {}

impl<D1, S1, D2, S2> MatrixDim for DCons<D1, S1, DCons<D2, S2, DNil>>
where
    D1: Dim,
    S1: Unsigned,
    D2: Dim,
    S2: Unsigned,
{
}

pub trait NonMatrixDim: DimList {}

impl NonMatrixDim for DNil {}

impl<D, S> NonMatrixDim for DCons<D, S, DNil>
where
    D: Dim,
    S: Unsigned,
{
}

impl<D1, S1, D2, S2, D3, S3, T> MatrixDim for DCons<D1, S1, DCons<D2, S2, DCons<D3, S3, T>>>
where
    D1: Dim,
    S1: Unsigned,
    D2: Dim,
    S2: Unsigned,
    D3: Dim,
    S3: Unsigned,
    T: DimList,
{
}

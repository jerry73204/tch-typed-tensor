mod keepdim;
mod reduction;

use crate::{
    device::TensorDevice,
    dim::{ConcatAt, Dim, DimList, Permute},
    kind::TensorKind,
    list::{TList, Where},
};
pub use keepdim::*;
pub use reduction::*;
use std::marker::PhantomData;
use tch::{Device as TchDevice, Kind as TchKind, Tensor};

// convenient trait to obtain typed properties

pub trait NamedTensorTrait<Dims, Kind, Dev>
where
    Dims: DimList,
    Kind: TensorKind,
    Dev: TensorDevice,
    Self::Dimension: DimList,
    Self::Kind: TensorKind,
    Self::Device: TensorDevice,
{
    type Dimension;
    type Kind;
    type Device;
}

impl<Dims, Kind, Dev> NamedTensorTrait<Dims, Kind, Dev> for NamedTensor<Dims, Kind, Dev>
where
    Dims: DimList,
    Kind: TensorKind,
    Dev: TensorDevice,
{
    type Dimension = Dims;
    type Kind = Kind;
    type Device = Dev;
}

// named tensor struct

pub struct NamedTensor<Dims: DimList, Kind: TensorKind, Dev: TensorDevice> {
    tensor: Tensor,
    _phantom: PhantomData<(Dims, Kind, Dev)>,
}

impl<Dims, Kind, Dev> NamedTensor<Dims, Kind, Dev>
where
    Dims: DimList,
    Kind: TensorKind,
    Dev: TensorDevice,
{
    const DEVICE: TchDevice = Dev::DEVICE;
    const KIND: TchKind = Kind::KIND;

    pub(self) fn from_tch_tensor(tensor: Tensor) -> Self {
        let ret = Self {
            tensor,
            _phantom: PhantomData,
        };

        debug_assert_eq!(ret.size(), ret.tensor.size());
        debug_assert_eq!(Self::KIND, ret.tensor.kind());
        debug_assert_eq!(Self::DEVICE, ret.tensor.device());

        ret
    }

    pub fn device(&self) -> TchDevice {
        Self::DEVICE
    }

    pub fn kind(&self) -> TchKind {
        Self::KIND
    }

    pub fn size(&self) -> Vec<i64> {
        Dims::shape().into_iter().map(|size| size as i64).collect()
    }

    pub fn zeros() -> Self {
        let shape = Dims::shape()
            .into_iter()
            .map(|val| val as i64)
            .collect::<Vec<_>>();
        NamedTensor::from_tch_tensor(Tensor::zeros(&shape, (Self::KIND, Self::DEVICE)))
    }

    pub fn zeros_like(&self) -> Self {
        let shape = Dims::shape()
            .into_iter()
            .map(|val| val as i64)
            .collect::<Vec<_>>();
        NamedTensor::from_tch_tensor(Tensor::zeros(&shape, (Self::KIND, Self::DEVICE)))
    }

    pub fn to_kind<NewKind>(&self) -> NamedTensor<Dims, NewKind, Dev>
    where
        NewKind: TensorKind,
    {
        NamedTensor::from_tch_tensor(self.tensor.to_kind(Self::KIND))
    }

    pub fn transpose<NewDims, Indexes>(
        &self,
    ) -> NamedTensor<<Dims as Permute<Indexes, NewDims>>::Output, Kind, Dev>
    where
        Indexes: TList,
        NewDims: TList,
        Dims: Permute<Indexes, NewDims>,
    {
        let indexes = <Dims as Permute<Indexes, NewDims>>::permute_index()
            .into_iter()
            .map(|idx| idx as i64)
            .collect::<Vec<_>>();

        NamedTensor::from_tch_tensor(self.tensor.permute(&indexes))
    }

    pub fn concat<Target, Index, RDimList>(
        &self,
        rhs: &NamedTensor<RDimList, Kind, Dev>,
    ) -> NamedTensor<<Dims as ConcatAt<RDimList, Target, Index>>::Output, Kind, Dev>
    where
        Dims: ConcatAt<RDimList, Target, Index>,
        Target: Dim,
        Index: Where,
        RDimList: DimList,
    {
        let index = <Dims as ConcatAt<RDimList, Target, Index>>::INDEX;
        let tensor = Tensor::cat(&[&self.tensor, &rhs.tensor], index as i64);
        NamedTensor::from_tch_tensor(tensor)
    }
}

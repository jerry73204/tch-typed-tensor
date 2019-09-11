use crate::{
    device::TensorDevice,
    dim::{DimList, Reorder},
    kind::TensorKind,
    list::TList,
};
use std::marker::PhantomData;
use tch::{Device, Kind, Tensor};

// convenient trait to obtain typed properties

pub trait NamedTensorTrait<L, K, D>
where
    L: DimList,
    K: TensorKind,
    D: TensorDevice,
{
    type Dimension;
    type Kind;
    type Device;
}

impl<L, K, D> NamedTensorTrait<L, K, D> for NamedTensor<L, K, D>
where
    L: DimList,
    K: TensorKind,
    D: TensorDevice,
{
    type Dimension = L;
    type Kind = K;
    type Device = D;
}

// named tensor struct

pub struct NamedTensor<L: DimList, K: TensorKind, D: TensorDevice> {
    tensor: Tensor,
    _phantom: PhantomData<(L, K, D)>,
}

impl<L, K, D> NamedTensor<L, K, D>
where
    L: DimList,
    K: TensorKind,
    D: TensorDevice,
{
    const DEVICE: Device = D::DEVICE;
    const KIND: Kind = K::KIND;

    pub fn device(&self) -> Device {
        Self::DEVICE
    }

    pub fn kind(&self) -> Kind {
        Self::KIND
    }

    pub fn size(&self) -> Vec<i64> {
        L::shape().into_iter().map(|size| size as i64).collect()
    }

    pub fn zeros() -> NamedTensor<L, K, D> {
        let shape = L::shape()
            .into_iter()
            .map(|val| val as i64)
            .collect::<Vec<_>>();
        let tensor = NamedTensor {
            tensor: Tensor::zeros(&shape, (K::KIND, D::DEVICE)),
            _phantom: PhantomData,
        };

        debug_assert_eq!(tensor.size(), tensor.tensor.size());
        tensor
    }

    pub fn zeros_like(&self) -> NamedTensor<L, K, D> {
        let shape = L::shape()
            .into_iter()
            .map(|val| val as i64)
            .collect::<Vec<_>>();
        let tensor = NamedTensor {
            tensor: Tensor::zeros(&shape, (K::KIND, D::DEVICE)),
            _phantom: PhantomData,
        };

        debug_assert_eq!(tensor.size(), tensor.tensor.size());
        tensor
    }

    pub fn transpose<IL, NL>(&self) -> NamedTensor<<L as Reorder<IL, NL>>::Output, K, D>
    where
        IL: TList,
        NL: TList,
        L: Reorder<IL, NL>,
        <L as Reorder<IL, NL>>::Output: DimList,
    {
        let indexes = <L as Reorder<IL, NL>>::permute_index()
            .into_iter()
            .map(|idx| idx as i64)
            .collect::<Vec<_>>();

        let tensor = NamedTensor::<<L as Reorder<IL, NL>>::Output, K, D> {
            tensor: self.tensor.permute(&indexes),
            _phantom: PhantomData,
        };

        debug_assert_eq!(tensor.size(), tensor.tensor.size());
        tensor
    }
}

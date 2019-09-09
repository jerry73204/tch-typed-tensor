use crate::{
    device::TensorDevice,
    dim::{Dim, DimList},
    kind::TensorKind,
};
use frunk::hlist::HList;
use std::marker::PhantomData;
use tch::{Device, Kind, Tensor};

pub struct NamedTensor<L: DimList, K: TensorKind, D: TensorDevice> {
    tensor: Tensor,
    phantom: PhantomData<(L, K, D)>,
}

impl<L: DimList, K: TensorKind, D: TensorDevice> NamedTensor<L, K, D> {
    const DEVICE: Device = D::DEVICE;
    const KIND: Kind = K::KIND;

    pub fn device(&self) -> Device {
        Self::DEVICE
    }

    pub fn kind(&self) -> Kind {
        Self::KIND
    }

    pub fn zeros() -> NamedTensor<L, K, D> {
        let shape = L::shape()
            .into_iter()
            .map(|val| val as i64)
            .collect::<Vec<_>>();
        NamedTensor {
            tensor: Tensor::zeros(&shape, (K::KIND, D::DEVICE)),
            phantom: PhantomData,
        }
    }

    pub fn zeros_like(&self) -> NamedTensor<L, K, D> {
        let shape = L::shape()
            .into_iter()
            .map(|val| val as i64)
            .collect::<Vec<_>>();
        NamedTensor {
            tensor: Tensor::zeros(&shape, (K::KIND, D::DEVICE)),
            phantom: PhantomData,
        }
    }
}

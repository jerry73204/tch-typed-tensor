use super::{NamedTensor, NamedTensorTrait};
use crate::{device::TensorDevice, dim::DimList, kind::TensorKind};
use tch::Scalar as TchScalar;

pub trait TensorPow<Kind>
where
    Kind: TensorKind,
    Self: NamedTensorTrait,
    Kind::Type: Into<TchScalar>,
{
    fn pow(&self, exponent: Kind::Type) -> Self;
}

impl<Dims, Kind, Dev> TensorPow<Kind> for NamedTensor<Dims, Kind, Dev>
where
    Dims: DimList,
    Kind: TensorKind,
    Dev: TensorDevice,
    Kind::Type: Into<TchScalar>,
{
    fn pow(&self, value: Kind::Type) -> Self {
        Self::from_tch_tensor(self.tensor.pow(value))
    }
}

use super::{NamedTensor, NamedTensorTrait};
use crate::{device::TensorDevice, dim::DimList, kind::TensorKind};
use tch::{Scalar as TchScalar, Tensor};

pub trait TensorFull<Kind>
where
    Kind: TensorKind,
    Self: NamedTensorTrait,
    Kind::Type: Into<TchScalar>,
{
    fn full(value: Kind::Type) -> Self;
}

impl<Dims, Kind, Dev> TensorFull<Kind> for NamedTensor<Dims, Kind, Dev>
where
    Dims: DimList,
    Kind: TensorKind,
    Dev: TensorDevice,
    Kind::Type: Into<TchScalar>,
{
    fn full(value: Kind::Type) -> Self {
        let size = Dims::shape_i64();
        let tensor = Tensor::full(&size, value, (Self::KIND, Self::DEVICE));
        Self::from_tch_tensor(tensor)
    }
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        device::Cpu,
        kind::{Double, Int64},
        make_dims, DimListType,
    };
    use typenum::consts::*;

    make_dims! {A, B, C}
    type Dims = DimListType! {(A, U3), (B, U2), (C, U1)};

    #[test]
    fn tensor_full_op_test() {
        let _ = NamedTensor::<Dims, Double, Cpu>::full(3.0);
        let _ = NamedTensor::<Dims, Int64, Cpu>::full(-2);
    }
}

use super::NamedTensor;
use crate::{
    device::TensorDevice,
    dim::DimList,
    index::{IndexList, IsIndexInBounded, IsIndexInBoundedOutput},
    kind::TensorKind,
};
use type_freak::control::IfOutput;

pub trait TensorValueAt<ValueType, Dims>
where
    Dims: DimList,
{
    fn value_at<Indexes>(&self) -> IfOutput<ValueType, IsIndexInBoundedOutput<Indexes, Dims>>
    where
        Indexes: IndexList + IsIndexInBounded<Dims>;
}

impl<Dims, Kind, Dev> TensorValueAt<f64, Dims> for NamedTensor<Dims, Kind, Dev>
where
    Dims: DimList,
    Kind: TensorKind,
    Dev: TensorDevice,
{
    fn value_at<Indexes>(&self) -> f64
    where
        Indexes: IndexList,
    {
        self.tensor.double_value(&Indexes::to_vec())
    }
}

impl<Dims, Kind, Dev> TensorValueAt<i64, Dims> for NamedTensor<Dims, Kind, Dev>
where
    Dims: DimList,
    Kind: TensorKind,
    Dev: TensorDevice,
{
    fn value_at<Indexes>(&self) -> i64
    where
        Indexes: IndexList,
    {
        self.tensor.int64_value(&Indexes::to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{device::Cpu, kind::Double, make_dims, DimListType, IndexListType};
    use typenum::consts::*;

    make_dims! {A, B, C, D}

    type Dims = DimListType! {(A, U3), (B, U2), (C, U1), (D, U2)};
    type Indexes = IndexListType! {(A, +U2), (B, -U2), (C, +U0), (D, -U1)};

    #[test]
    fn tensor_value_at_test() {
        let tensor = NamedTensor::<Dims, Double, Cpu>::zeros();
        let _: i64 = tensor.value_at::<Indexes>();
    }
}

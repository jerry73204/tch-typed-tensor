mod full_op;
mod keepdim;
mod reduction;
mod value_at;

use crate::{
    boolean::{IfLess, IfLessOutput},
    counter::Where,
    device::TensorDevice,
    dim::{
        DConcatAt, DConcatAtOutput, DPermute, DPermuteOutput, DRemoveAt, DRemoveAtOutput, DSizeAt,
        DSizeAtOutput, Dim, DimList,
    },
    kind::TensorKind,
    list::TList,
};
pub use full_op::*;
pub use keepdim::*;
pub use reduction::*;
use std::marker::PhantomData;
use tch::{Device as TchDevice, Kind as TchKind, Tensor};
use typenum::Unsigned;
pub use value_at::*;

// convenient trait to obtain typed properties

pub trait NamedTensorTrait
where
    Self::Dimension: DimList,
    Self::Kind: TensorKind,
    Self::Device: TensorDevice,
{
    type Dimension;
    type Kind;
    type Device;
}

impl<Dims, Kind, Dev> NamedTensorTrait for NamedTensor<Dims, Kind, Dev>
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
        Dims::shape_i64()
    }

    pub fn zeros() -> Self {
        let shape = Dims::shape_i64();
        Self::from_tch_tensor(Tensor::zeros(&shape, (Self::KIND, Self::DEVICE)))
    }

    pub fn zeros_like(&self) -> Self {
        Self::from_tch_tensor(self.tensor.zeros_like())
    }

    pub fn randn() -> Self {
        let shape = Dims::shape_i64();
        Self::from_tch_tensor(Tensor::randn(&shape, (Self::KIND, Self::DEVICE)))
    }

    pub fn randn_like(&self) -> Self {
        Self::from_tch_tensor(self.tensor.randn_like())
    }

    pub fn to_kind<NewKind>(&self) -> NamedTensor<Dims, NewKind, Dev>
    where
        NewKind: TensorKind,
    {
        NamedTensor::from_tch_tensor(self.tensor.to_kind(Self::KIND))
    }

    pub fn to_device<NewDevice>(&self) -> NamedTensor<Dims, Kind, NewDevice>
    where
        NewDevice: TensorDevice,
    {
        NamedTensor::from_tch_tensor(self.tensor.to_device(Self::DEVICE))
    }

    pub fn transpose<NewDims, Indexes>(
        &self,
    ) -> NamedTensor<DPermuteOutput<Dims, NewDims, Indexes>, Kind, Dev>
    where
        Indexes: TList,
        NewDims: TList,
        Dims: DPermute<NewDims, Indexes>,
    {
        let indexes = <Dims as DPermute<NewDims, Indexes>>::permute_index()
            .into_iter()
            .map(|idx| idx as i64)
            .collect::<Vec<_>>();

        NamedTensor::from_tch_tensor(self.tensor.permute(&indexes))
    }

    pub fn concat<Target, Index, RDimList>(
        &self,
        rhs: &NamedTensor<RDimList, Kind, Dev>,
    ) -> NamedTensor<DConcatAtOutput<Dims, RDimList, Target, Index>, Kind, Dev>
    where
        Dims: DConcatAt<RDimList, Target, Index>,
        Target: Dim,
        Index: Where,
        RDimList: DimList,
    {
        let index = <Dims as DConcatAt<RDimList, Target, Index>>::INDEX;
        let tensor = Tensor::cat(&[&self.tensor, &rhs.tensor], index as i64);
        NamedTensor::from_tch_tensor(tensor)
    }

    pub fn select<SelectedIndex, Target, TargetIndex>(
        &self,
    ) -> NamedTensor<
        IfLessOutput<
            DRemoveAtOutput<Dims, Target, TargetIndex>,
            SelectedIndex,
            DSizeAtOutput<Dims, Target, TargetIndex>,
        >,
        Kind,
        Dev,
    >
    where
        Dims: DRemoveAt<Target, TargetIndex> + DSizeAt<Target, TargetIndex>,
        SelectedIndex: Unsigned,
        Target: Dim,
        TargetIndex: Where,
        DRemoveAtOutput<Dims, Target, TargetIndex>:
            IfLess<SelectedIndex, DSizeAtOutput<Dims, Target, TargetIndex>>,
        IfLessOutput<
            DRemoveAtOutput<Dims, Target, TargetIndex>,
            SelectedIndex,
            DSizeAtOutput<Dims, Target, TargetIndex>,
        >: DimList,
    {
        let target_index = TargetIndex::COUNT_I64;
        NamedTensor::from_tch_tensor(
            self.tensor
                .select(target_index as i64, SelectedIndex::to_i64()),
        )
    }
}

// tests

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::{list::LAssertEqualOutput, make_dims, DimListType, TListType};
    // use typenum::consts::*;

    #[test]
    fn named_tensor_test() {
        // TODO
    }
}

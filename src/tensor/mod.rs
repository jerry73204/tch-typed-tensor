mod full_op;
mod keepdim;
mod pow_op;
mod reduction;
mod value_at;

use type_freak::{
    control::{IfLess, IfLessOutput},
    counter::{Count, CountOutput, Counter},
    list::TList,
};

use crate::{
    device::TensorDevice,
    dim::{
        BroadcastMatcher, DConcatAt, DConcatAtOutput, DFlatten, DFlattenBeginIndex,
        DFlattenEndIndex, DFlattenOutput, DIndexOfMany, DMatMul, DMatMulBroadcasted,
        DMatMulBroadcastedOutput, DMatMulOutput, DPermute, DPermuteOutput, DRemoveAt,
        DRemoveAtOutput, DSizeAt, DSizeAtOutput, Dim, DimList, MatrixDim,
    },
    kind::TensorKind,
};
pub use full_op::*;
pub use keepdim::*;
pub use pow_op::*;
pub use reduction::*;
use std::marker::PhantomData;
use tch::{Device as TchDevice, Kind as TchKind, Tensor};
use typenum::{IsLess, Unsigned};
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

    pub fn neg(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.neg())
    }

    pub fn reciprocal(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.reciprocal())
    }

    pub fn rsqrt(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.rsqrt())
    }

    pub fn sqrt(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.sqrt())
    }

    pub fn round(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.round())
    }

    pub fn floor(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.floor())
    }

    pub fn ceil(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.ceil())
    }

    pub fn sin(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.sin())
    }

    pub fn cos(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.cos())
    }

    pub fn tan(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.tan())
    }

    pub fn sinh(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.sinh())
    }

    pub fn cosh(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.cosh())
    }

    pub fn tanh(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.tanh())
    }

    pub fn sign(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.sign())
    }

    pub fn trunc(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.trunc())
    }

    pub fn sigmoid(&self) -> Self {
        NamedTensor::from_tch_tensor(self.tensor.sigmoid())
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

    pub fn flip<SelectedDims, Indexes>(&self) -> Self
    where
        Indexes: TList,
        SelectedDims: TList,
        Dims: DIndexOfMany<SelectedDims, Indexes>,
    {
        let indexes = <Dims as DIndexOfMany<SelectedDims, Indexes>>::indexes()
            .into_iter()
            .map(|idx| idx as i64)
            .collect::<Vec<_>>();

        NamedTensor::from_tch_tensor(self.tensor.flip(&indexes))
    }

    pub fn concat<Target, Index, RDimList>(
        &self,
        rhs: &NamedTensor<RDimList, Kind, Dev>,
    ) -> NamedTensor<DConcatAtOutput<Dims, RDimList, Target, Index>, Kind, Dev>
    where
        Dims: DConcatAt<RDimList, Target, Index>,
        Target: Dim,
        Index: Counter,
        RDimList: DimList,
    {
        let index = <Dims as DConcatAt<RDimList, Target, Index>>::INDEX;
        let tensor = Tensor::cat(&[&self.tensor, &rhs.tensor], index as i64);
        NamedTensor::from_tch_tensor(tensor)
    }

    pub fn flatten<NewDim, BeginDim, EndDim, BeginIndex, EndIndex>(
        &self,
    ) -> NamedTensor<DFlattenOutput<Dims, NewDim, BeginDim, EndDim, BeginIndex, EndIndex>, Kind, Dev>
    where
        NewDim: Dim,
        BeginDim: Dim,
        EndDim: Dim,
        BeginIndex: Counter,
        EndIndex: Counter,
        Dims: DFlatten<NewDim, BeginDim, EndDim, BeginIndex, EndIndex>,
    {
        let begin_index =
            DFlattenBeginIndex::<Dims, NewDim, BeginDim, EndDim, BeginIndex, EndIndex>::I64;
        let end_index =
            DFlattenEndIndex::<Dims, NewDim, BeginDim, EndDim, BeginIndex, EndIndex>::I64;

        NamedTensor::from_tch_tensor(self.tensor.flatten(begin_index, end_index))
    }

    pub fn mm<RhsDims>(
        &self,
        rhs: NamedTensor<RhsDims, Kind, Dev>,
    ) -> NamedTensor<DMatMulOutput<Dims, RhsDims>, Kind, Dev>
    where
        Dims: MatrixDim + DMatMul<RhsDims>,
        RhsDims: MatrixDim,
    {
        NamedTensor::from_tch_tensor(self.tensor.mm(&rhs.tensor))
    }

    pub fn matmul<RhsDims, Matcher>(
        &self,
        rhs: NamedTensor<RhsDims, Kind, Dev>,
    ) -> NamedTensor<DMatMulBroadcastedOutput<Dims, RhsDims, Matcher>, Kind, Dev>
    where
        Dims: DMatMulBroadcasted<RhsDims, Matcher>,
        RhsDims: DimList,
        Matcher: BroadcastMatcher,
    {
        NamedTensor::from_tch_tensor(self.tensor.matmul(&rhs.tensor))
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
        SelectedIndex: Unsigned + IsLess<DSizeAtOutput<Dims, Target, TargetIndex>>,
        Target: Dim,
        TargetIndex: Counter + Count,
        DRemoveAtOutput<Dims, Target, TargetIndex>:
            IfLess<SelectedIndex, DSizeAtOutput<Dims, Target, TargetIndex>>,
        IfLessOutput<
            DRemoveAtOutput<Dims, Target, TargetIndex>,
            SelectedIndex,
            DSizeAtOutput<Dims, Target, TargetIndex>,
        >: DimList,
    {
        let target_index = CountOutput::<TargetIndex>::I64;
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

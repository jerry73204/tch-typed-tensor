use super::{DoKeepDim, KeepDim, KeepDimOrNot, NamedTensor, NoKeepDim, TensorDevice, TensorKind};
use crate::{
    boolean::Boolean,
    dim::{
        DReduceManyToOne, DReduceManyToOneOutput, DRemoveMany, DRemoveManyOutput, DimList,
        NonScalarDim,
    },
    list::NonEmptyList,
};

// reduction op

pub trait Reduction<Keep, Targets, Indexes>
where
    Indexes: NonEmptyList,
    Targets: NonEmptyList,
    Keep: KeepDim + KeepDimOrNot,
    Self::OutDim: DimList,
{
    type OutDim;

    fn reduced_indexes() -> Vec<usize>;
}

impl<InDim, InKind, Device, Indexes, Targets> Reduction<NoKeepDim, Targets, Indexes>
    for NamedTensor<InDim, InKind, Device>
where
    InDim: NonScalarDim + DRemoveMany<Targets, Indexes>,
    InKind: TensorKind,
    Device: TensorDevice,
    Indexes: NonEmptyList,
    Targets: NonEmptyList,
{
    type OutDim = DRemoveManyOutput<InDim, Targets, Indexes>;

    fn reduced_indexes() -> Vec<usize> {
        <InDim as DRemoveMany<Targets, Indexes>>::indexes()
    }
}

impl<InDim, InKind, Device, Indexes, Targets> Reduction<DoKeepDim, Targets, Indexes>
    for NamedTensor<InDim, InKind, Device>
where
    InDim: NonScalarDim + DReduceManyToOne<Targets, Indexes>,
    InKind: TensorKind,
    Device: TensorDevice,
    Indexes: NonEmptyList,
    Targets: NonEmptyList,
{
    type OutDim = DReduceManyToOneOutput<InDim, Targets, Indexes>;

    fn reduced_indexes() -> Vec<usize> {
        <InDim as DReduceManyToOne<Targets, Indexes>>::indexes()
    }
}

pub type ReductionOutDim<Tensor, Keep, Targets, Indexes> =
    <Tensor as Reduction<Keep, Targets, Indexes>>::OutDim;

// reduce sum

pub trait ReduceSum<InDim, Device>
where
    InDim: DimList,
    Device: TensorDevice,
{
    fn sum<Keep, OutKind, Targets, Indexes>(
        &self,
    ) -> NamedTensor<ReductionOutDim<Self, Keep, Targets, Indexes>, OutKind, Device>
    where
        Indexes: NonEmptyList,
        Targets: NonEmptyList,
        Keep: KeepDim + KeepDimOrNot,
        OutKind: TensorKind,
        Self: Reduction<Keep, Targets, Indexes>;
}

impl<InDim, InKind, Device> ReduceSum<InDim, Device> for NamedTensor<InDim, InKind, Device>
where
    InDim: NonScalarDim,
    InKind: TensorKind,
    Device: TensorDevice,
{
    fn sum<Keep, OutKind, Targets, Indexes>(
        &self,
    ) -> NamedTensor<ReductionOutDim<Self, Keep, Targets, Indexes>, OutKind, Device>
    where
        Indexes: NonEmptyList,
        Targets: NonEmptyList,
        Keep: KeepDim + KeepDimOrNot,
        OutKind: TensorKind,
        Self: Reduction<Keep, Targets, Indexes>,
    {
        let indexes = <Self as Reduction<Keep, Targets, Indexes>>::reduced_indexes()
            .into_iter()
            .map(|idx| idx as i64)
            .collect::<Vec<_>>();

        NamedTensor::from_tch_tensor(self.tensor.sum1(
            &indexes,
            <Keep as KeepDimOrNot>::Output::VALUE,
            OutKind::KIND,
        ))
    }
}

// reduce mean

pub trait ReduceMean<InDim, Device>
where
    InDim: DimList,
    Device: TensorDevice,
{
    fn mean<Keep, OutKind, Targets, Indexes>(
        &self,
    ) -> NamedTensor<ReductionOutDim<Self, Keep, Targets, Indexes>, OutKind, Device>
    where
        Indexes: NonEmptyList,
        Targets: NonEmptyList,
        Keep: KeepDim + KeepDimOrNot,
        OutKind: TensorKind,
        Self: Reduction<Keep, Targets, Indexes>;
}

impl<InDim, InKind, Device> ReduceMean<InDim, Device> for NamedTensor<InDim, InKind, Device>
where
    InDim: NonScalarDim,
    InKind: TensorKind,
    Device: TensorDevice,
{
    fn mean<Keep, OutKind, Targets, Indexes>(
        &self,
    ) -> NamedTensor<ReductionOutDim<Self, Keep, Targets, Indexes>, OutKind, Device>
    where
        Indexes: NonEmptyList,
        Targets: NonEmptyList,
        Keep: KeepDim + KeepDimOrNot,
        OutKind: TensorKind,
        Self: Reduction<Keep, Targets, Indexes>,
    {
        let indexes = <Self as Reduction<Keep, Targets, Indexes>>::reduced_indexes()
            .into_iter()
            .map(|idx| idx as i64)
            .collect::<Vec<_>>();

        NamedTensor::from_tch_tensor(self.tensor.mean1(
            &indexes,
            <Keep as KeepDimOrNot>::Output::VALUE,
            OutKind::KIND,
        ))
    }
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        device::Cpu,
        kind::{Double, Float},
        make_dims, DimListType, TListType,
    };
    use typenum::consts::*;

    make_dims! {A, B, C}

    type SomeDims = DimListType! {(A, U3), (B, U2), (C, U4)};
    type SomeTensor = NamedTensor<SomeDims, Double, Cpu>;

    type NoKeepACDims = DimListType! {(B, U2)};
    type DoKeepACDims = DimListType! {(A, U1), (B, U2), (C, U1)};

    #[test]
    fn tensor_reduction_test() {
        let tensor = SomeTensor::zeros();

        let _: NamedTensor<NoKeepACDims, Double, Cpu> =
            tensor.sum::<NoKeepDim, Double, TListType! {A, C}, _>();

        let _: NamedTensor<DoKeepACDims, Double, Cpu> =
            tensor.sum::<DoKeepDim, Double, TListType! {A, C}, _>();

        let _: NamedTensor<NoKeepACDims, Float, Cpu> =
            tensor.mean::<NoKeepDim, Float, TListType! {A, C}, _>();

        let _: NamedTensor<DoKeepACDims, Float, Cpu> =
            tensor.mean::<DoKeepDim, Float, TListType! {A, C}, _>();
    }
}

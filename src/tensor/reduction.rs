use super::{DoKeepDim, KeepDim, KeepDimOrNot, NamedTensor, NoKeepDim, TensorDevice, TensorKind};
use crate::{
    boolean::TBoolean,
    dim::{DimList, NonScalarDim, ReduceManyToOne, RemoveMany},
    list::NonEmptyList,
};

// reduction op

pub trait Reduction<Indexes: NonEmptyList, Targets: NonEmptyList, Keep: KeepDim + KeepDimOrNot>
where
    Self::OutDim: DimList,
{
    type OutDim;

    fn reduced_indexes() -> Vec<usize>;
}

impl<InDim, InKind, Device, Indexes, Targets> Reduction<Indexes, Targets, NoKeepDim>
    for NamedTensor<InDim, InKind, Device>
where
    InDim: NonScalarDim + RemoveMany<Indexes, Targets>,
    InKind: TensorKind,
    Device: TensorDevice,
    Indexes: NonEmptyList,
    Targets: NonEmptyList,
{
    type OutDim = <InDim as RemoveMany<Indexes, Targets>>::Output;

    fn reduced_indexes() -> Vec<usize> {
        <InDim as RemoveMany<Indexes, Targets>>::indexes()
    }
}

impl<InDim, InKind, Device, Indexes, Targets> Reduction<Indexes, Targets, DoKeepDim>
    for NamedTensor<InDim, InKind, Device>
where
    InDim: NonScalarDim + ReduceManyToOne<Indexes, Targets>,
    InKind: TensorKind,
    Device: TensorDevice,
    Indexes: NonEmptyList,
    Targets: NonEmptyList,
{
    type OutDim = <InDim as ReduceManyToOne<Indexes, Targets>>::Output;

    fn reduced_indexes() -> Vec<usize> {
        <InDim as ReduceManyToOne<Indexes, Targets>>::indexes()
    }
}

// reduce sum

pub trait ReduceSum<InDim, Device>
where
    InDim: DimList,
    Device: TensorDevice,
{
    fn sum<Indexes, Targets, Keep, OutKind>(
        &self,
    ) -> NamedTensor<<Self as Reduction<Indexes, Targets, Keep>>::OutDim, OutKind, Device>
    where
        Indexes: NonEmptyList,
        Targets: NonEmptyList,
        Keep: KeepDim + KeepDimOrNot,
        OutKind: TensorKind,
        Self: Reduction<Indexes, Targets, Keep>;
}

impl<InDim, InKind, Device> ReduceSum<InDim, Device> for NamedTensor<InDim, InKind, Device>
where
    InDim: NonScalarDim,
    InKind: TensorKind,
    Device: TensorDevice,
{
    fn sum<Indexes, Targets, Keep, OutKind>(
        &self,
    ) -> NamedTensor<<Self as Reduction<Indexes, Targets, Keep>>::OutDim, OutKind, Device>
    where
        Indexes: NonEmptyList,
        Targets: NonEmptyList,
        Keep: KeepDim + KeepDimOrNot,
        OutKind: TensorKind,
        Self: Reduction<Indexes, Targets, Keep>,
    {
        let indexes = <Self as Reduction<Indexes, Targets, Keep>>::reduced_indexes()
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
    fn mean<Indexes, Targets, Keep, OutKind>(
        &self,
    ) -> NamedTensor<<Self as Reduction<Indexes, Targets, Keep>>::OutDim, OutKind, Device>
    where
        Indexes: NonEmptyList,
        Targets: NonEmptyList,
        Keep: KeepDim + KeepDimOrNot,
        OutKind: TensorKind,
        Self: Reduction<Indexes, Targets, Keep>;
}

impl<InDim, InKind, Device> ReduceMean<InDim, Device> for NamedTensor<InDim, InKind, Device>
where
    InDim: NonScalarDim,
    InKind: TensorKind,
    Device: TensorDevice,
{
    fn mean<Indexes, Targets, Keep, OutKind>(
        &self,
    ) -> NamedTensor<<Self as Reduction<Indexes, Targets, Keep>>::OutDim, OutKind, Device>
    where
        Indexes: NonEmptyList,
        Targets: NonEmptyList,
        Keep: KeepDim + KeepDimOrNot,
        OutKind: TensorKind,
        Self: Reduction<Indexes, Targets, Keep>,
    {
        let indexes = <Self as Reduction<Indexes, Targets, Keep>>::reduced_indexes()
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

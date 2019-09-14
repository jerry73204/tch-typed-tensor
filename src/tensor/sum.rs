use super::{Keep, KeepDim, NamedTensor, NoKeep, TensorDevice, TensorKind};
use crate::{
    dim::{DCons, Dim, DimList, ReduceManyToOne, RemoveMany},
    list::TList,
};
use typenum::Unsigned;

// sum op

pub trait Sum<
    OutKind: TensorKind,
    Device: TensorDevice,
    DoKeep: KeepDim,
    Indexes: TList,
    Targets: TList,
> where
    Self::OutDim: DimList,
{
    type OutDim;

    fn sum(&self) -> NamedTensor<Self::OutDim, OutKind, Device>;
}

impl<FirstDim, FirstSize, InDimRemain, InKind, OutKind, Device, Indexes, Targets>
    Sum<OutKind, Device, NoKeep, Indexes, Targets>
    for NamedTensor<DCons<FirstDim, FirstSize, InDimRemain>, InKind, Device>
where
    FirstDim: Dim,
    FirstSize: Unsigned,
    InDimRemain: DimList,
    InKind: TensorKind,
    OutKind: TensorKind,
    Device: TensorDevice,
    Indexes: TList,
    Targets: TList,
    DCons<FirstDim, FirstSize, InDimRemain>: RemoveMany<Indexes, Targets>,
    <DCons<FirstDim, FirstSize, InDimRemain> as RemoveMany<Indexes, Targets>>::Output: DimList,
{
    type OutDim = <DCons<FirstDim, FirstSize, InDimRemain> as RemoveMany<Indexes, Targets>>::Output;

    fn sum(&self) -> NamedTensor<Self::OutDim, OutKind, Device> {
        let indexes =
            <DCons<FirstDim, FirstSize, InDimRemain> as RemoveMany<Indexes, Targets>>::indexes()
                .into_iter()
                .map(|idx| idx as i64)
                .collect::<Vec<_>>();

        NamedTensor::from_tch_tensor(self.tensor.sum4(&indexes, false, OutKind::KIND))
    }
}

impl<FirstDim, FirstSize, InDimRemain, InKind, OutKind, Device, Indexes, Targets>
    Sum<OutKind, Device, Keep, Indexes, Targets>
    for NamedTensor<DCons<FirstDim, FirstSize, InDimRemain>, InKind, Device>
where
    FirstDim: Dim,
    FirstSize: Unsigned,
    InDimRemain: DimList,
    InKind: TensorKind,
    OutKind: TensorKind,
    Device: TensorDevice,
    Indexes: TList,
    Targets: TList,
    DCons<FirstDim, FirstSize, InDimRemain>: ReduceManyToOne<Indexes, Targets>,
    <DCons<FirstDim, FirstSize, InDimRemain> as ReduceManyToOne<Indexes, Targets>>::Output: DimList,
{
    type OutDim =
        <DCons<FirstDim, FirstSize, InDimRemain> as ReduceManyToOne<Indexes, Targets>>::Output;

    fn sum(&self) -> NamedTensor<Self::OutDim, OutKind, Device> {
        let indexes = <DCons<FirstDim, FirstSize, InDimRemain> as ReduceManyToOne<
            Indexes,
            Targets,
        >>::indexes()
        .into_iter()
        .map(|idx| idx as i64)
        .collect::<Vec<_>>();

        NamedTensor::from_tch_tensor(self.tensor.sum4(&indexes, true, OutKind::KIND))
    }
}

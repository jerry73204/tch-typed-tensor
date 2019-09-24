use super::{DCons, DMarkedCons, Dim, DimList};
use type_freak::{
    counter::{Count, CountOutput, Counter, Current, Next},
    list::{LCons, LNil, TList},
};
use typenum::Unsigned;

// mark node

pub trait DMark<Target, Index>
where
    Target: Dim,
    Index: Counter,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

pub type DMarkOutput<List, Target, Index> = <List as DMark<Target, Index>>::Output;

impl<Target, Size, Tail> DMark<Target, Current> for DCons<Target, Size, Tail>
where
    Target: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    type Output = DMarkedCons<Target, Size, Tail>;
}

impl<Target, Index, NonTarget, Size, Tail> DMark<Target, Next<Index>>
    for DCons<NonTarget, Size, Tail>
where
    Target: Dim,
    Index: Counter,
    NonTarget: Dim,
    Size: Unsigned,
    Tail: DimList + DMark<Target, Index>,
{
    type Output = DCons<NonTarget, Size, DMarkOutput<Tail, Target, Index>>;
}

impl<Target, Index, NonTarget, Size, Tail> DMark<Target, Next<Index>>
    for DMarkedCons<NonTarget, Size, Tail>
where
    Target: Dim,
    Index: Counter,
    NonTarget: Dim,
    Size: Unsigned,
    Tail: DimList + DMark<Target, Index>,
{
    type Output = DMarkedCons<NonTarget, Size, DMarkOutput<Tail, Target, Index>>;
}

// mark multiple nodes

pub trait DMarkMany<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;

    fn indexes() -> Vec<usize>;
    fn append_indexes(prev: &mut Vec<usize>);
}

pub type DMarkManyOutput<List, Targets, Indexes> = <List as DMarkMany<Targets, Indexes>>::Output;

impl<List> DMarkMany<LNil, LNil> for List
where
    List: DimList,
{
    type Output = List;

    fn indexes() -> Vec<usize> {
        vec![]
    }

    fn append_indexes(_prev: &mut Vec<usize>) {}
}

impl<Target, TRemain, Index, IRemain, List> DMarkMany<LCons<Target, TRemain>, LCons<Index, IRemain>>
    for List
where
    Target: Dim,
    TRemain: TList,
    Index: Counter + Count,
    IRemain: TList,
    List: DimList + DMark<Target, Index>,
    DMarkOutput<List, Target, Index>: DMarkMany<TRemain, IRemain>,
{
    type Output = DMarkManyOutput<DMarkOutput<List, Target, Index>, TRemain, IRemain>;

    fn indexes() -> Vec<usize> {
        let mut indexes = vec![];
        <List as DMarkMany<LCons<Target, TRemain>, LCons<Index, IRemain>>>::append_indexes(
            &mut indexes,
        );
        indexes
    }

    fn append_indexes(prev: &mut Vec<usize>) {
        prev.push(CountOutput::<Index>::USIZE);
        <DMarkOutput<List, Target, Index> as DMarkMany<TRemain, IRemain>>::append_indexes(prev);
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::{make_dims, DimListType};
    // use type_freak::{control::IfSameOutput, TListType};
    // use typenum::consts::*;

    // make_dims! {A, B, C, D, E}

    #[test]
    fn dim_makr_test() {
        // TODO
    }
}

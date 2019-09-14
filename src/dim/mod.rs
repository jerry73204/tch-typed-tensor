mod broadcast;
mod marker;

pub use broadcast::*;
pub use marker::*;

use crate::list::{
    Cons, Here, IndexOf as TIndexOf, Nil, SetEqual as TSetEqual, TList, There, Where,
};
use std::marker::PhantomData;
use typenum::marker_traits::Unsigned;

// dimension list

pub trait Dim {}

pub trait DimList: TList {
    fn shape() -> Vec<usize>;
}

// end of dim list

pub struct DNil {}

impl DimList for DNil {
    fn shape() -> Vec<usize> {
        vec![]
    }
}

impl TList for DNil {
    const LENGTH: usize = Nil::LENGTH;
}

// node of dim list

pub struct DCons<D: Dim, S: Unsigned, T: DimList> {
    _phantom: PhantomData<(D, S, T)>,
}

impl<D, S, T> DimList for DCons<D, S, T>
where
    D: Dim,
    S: Unsigned,
    T: DimList,
{
    fn shape() -> Vec<usize> {
        // TODO: fix O(n^2) complexity here
        let mut shp = T::shape();
        shp.insert(0, S::USIZE);
        shp
    }
}

impl<D, S, T> TList for DCons<D, S, T>
where
    D: Dim,
    S: Unsigned,
    T: DimList,
{
    const LENGTH: usize = Cons::<(D, S), T>::LENGTH;
}

// extract dimension part

pub trait ExtractDim
where
    Self::Output: TList,
{
    type Output;
}

impl ExtractDim for DNil {
    type Output = Nil;
}

impl<D, S, T> ExtractDim for DCons<D, S, T>
where
    D: Dim,
    S: Unsigned,
    T: DimList + ExtractDim,
{
    type Output = Cons<D, <T as ExtractDim>::Output>;
}

// index of

pub trait IndexOf<I: Where, E: Dim> {
    const INDEX: usize;
}

impl<I, E, D, S, T> IndexOf<I, E> for DCons<D, S, T>
where
    I: Where,
    E: Dim,
    D: Dim,
    S: Unsigned,
    T: DimList + ExtractDim,
    Cons<D, <T as ExtractDim>::Output>: TIndexOf<I, E>,
{
    const INDEX: usize = <<Self as ExtractDim>::Output as TIndexOf<I, E>>::INDEX;
}

// index of many

pub trait IndexOfMany<Indexes: TList, Targets: TList> {
    fn indexes() -> Vec<usize>;
    fn inverse_indexes() -> Vec<usize>;
}

impl<L> IndexOfMany<Nil, Nil> for L
where
    L: DimList,
{
    fn indexes() -> Vec<usize> {
        vec![]
    }

    fn inverse_indexes() -> Vec<usize> {
        (0..(L::LENGTH)).collect()
    }
}

impl<Index, IRemain, Target, TRemain, D, Size, Tail>
    IndexOfMany<Cons<Index, IRemain>, Cons<Target, TRemain>> for DCons<D, Size, Tail>
where
    Index: Where,
    IRemain: TList,
    Target: Dim,
    TRemain: TList,
    D: Dim,
    Size: Unsigned,
    Tail: DimList,
    Self: IndexOfMany<IRemain, TRemain> + IndexOf<Index, Target>,
{
    fn indexes() -> Vec<usize> {
        let mut indexes = <Self as IndexOfMany<IRemain, TRemain>>::indexes();
        indexes.insert(0, <Self as IndexOf<Index, Target>>::INDEX);
        indexes
    }

    fn inverse_indexes() -> Vec<usize> {
        let mut indexes = <Self as IndexOfMany<IRemain, TRemain>>::inverse_indexes();
        indexes.remove_item(&<Self as IndexOf<Index, Target>>::INDEX);
        indexes
    }
}

// size at

pub trait SizeAt<I: Where, E: Dim>
where
    Self::Output: Unsigned,
{
    type Output;
}

impl<D, S, T> SizeAt<Here, D> for DCons<D, S, T>
where
    D: Dim,
    S: Unsigned,
    T: DimList,
{
    type Output = S;
}

impl<I, E, D, S, T> SizeAt<There<I>, E> for DCons<D, S, T>
where
    I: Where,
    E: Dim,
    D: Dim,
    S: Unsigned,
    T: DimList + SizeAt<I, E>,
{
    type Output = <T as SizeAt<I, E>>::Output;
}

// prepend

pub trait Prepend<ND: Dim, NS: Unsigned>
where
    Self::Output: DimList,
{
    type Output;
}

impl<ND, NS, L> Prepend<ND, NS> for L
where
    ND: Dim,
    NS: Unsigned,
    L: DimList,
{
    type Output = DCons<ND, NS, L>;
}

// append

pub trait Append<ND: Dim, NS: Unsigned>
where
    Self::Output: DimList,
{
    type Output;
}

impl<ND, NS> Append<ND, NS> for DNil
where
    ND: Dim,
    NS: Unsigned,
{
    type Output = DCons<ND, NS, DNil>;
}

impl<ND, NS, D, S, T> Append<ND, NS> for DCons<D, S, T>
where
    ND: Dim,
    NS: Unsigned,
    D: Dim,
    S: Unsigned,
    T: DimList + Append<ND, NS>,
{
    type Output = DCons<D, S, <T as Append<ND, NS>>::Output>;
}

// insert at

pub trait InsertAt<I: Where, E: Dim, ND: Dim, NS: Unsigned>
where
    Self::Output: DimList,
{
    type Output;
}

impl<ND, NS, D, S, T> InsertAt<Here, D, ND, NS> for DCons<D, S, T>
where
    ND: Dim,
    NS: Unsigned,
    D: Dim,
    S: Unsigned,
    T: DimList,
{
    type Output = DCons<ND, NS, DCons<D, S, T>>;
}

impl<I, ND, NS, E, D, S, T> InsertAt<There<I>, E, ND, NS> for DCons<D, S, T>
where
    I: Where,
    ND: Dim,
    NS: Unsigned,
    E: Dim,
    D: Dim,
    S: Unsigned,
    T: DimList + InsertAt<I, E, ND, NS>,
{
    type Output = DCons<D, S, <T as InsertAt<I, E, ND, NS>>::Output>;
}

// expand at

pub trait ExpandAt<I: Where, E: Dim, ND: Dim>
where
    Self::Output: DimList,
{
    type Output;
}

impl<I, E, ND, L> ExpandAt<I, E, ND> for L
where
    I: Where,
    E: Dim,
    ND: Dim,
    L: DimList + InsertAt<I, E, ND, typenum::consts::U1>,
{
    type Output = <L as InsertAt<I, E, ND, typenum::consts::U1>>::Output;
}

// remove at

pub trait RemoveAt<I: Where, E: Dim>
where
    Self::Output: DimList,
{
    type Output;

    fn index() -> usize;
}

impl<D, S, T> RemoveAt<Here, D> for DCons<D, S, T>
where
    D: Dim,
    S: Unsigned,
    T: DimList,
{
    type Output = T;

    fn index() -> usize {
        0
    }
}

impl<I, E, D, S, T> RemoveAt<There<I>, E> for DCons<D, S, T>
where
    I: Where,
    E: Dim,
    D: Dim,
    S: Unsigned,
    T: DimList + RemoveAt<I, E>,
{
    type Output = DCons<D, S, <T as RemoveAt<I, E>>::Output>;

    fn index() -> usize {
        1 + <T as RemoveAt<I, E>>::index()
    }
}

// remove many

pub trait RemoveMany<Indexes: TList, Targets: TList>
where
    Self::Output: DimList,
{
    type Output;

    fn indexes() -> Vec<usize>;
}

impl<L> RemoveMany<Nil, Nil> for L
where
    L: DimList,
{
    type Output = L;

    fn indexes() -> Vec<usize> {
        vec![]
    }
}

impl<Index, IRemain, Target, TRemain, Dimension, Size, Tail>
    RemoveMany<Cons<Index, IRemain>, Cons<Target, TRemain>> for DCons<Dimension, Size, Tail>
where
    Index: Where,
    IRemain: TList,
    Target: Dim,
    TRemain: TList,
    Dimension: Dim,
    Size: Unsigned,
    Tail: DimList,
    Self: RemoveAt<Index, Target>,
    <Self as RemoveAt<Index, Target>>::Output: RemoveMany<IRemain, TRemain>,
{
    type Output =
        <<Self as RemoveAt<Index, Target>>::Output as RemoveMany<IRemain, TRemain>>::Output;

    fn indexes() -> Vec<usize> {
        let curr_index = <Self as RemoveAt<Index, Target>>::index();
        let mut indexes =
            <<Self as RemoveAt<Index, Target>>::Output as RemoveMany<IRemain, TRemain>>::indexes()
                .into_iter()
                .map(|idx| if idx >= curr_index { idx + 1 } else { idx })
                .collect::<Vec<_>>();
        indexes.insert(0, curr_index);
        indexes
    }
}

// reduce size to one

pub trait ReduceToOne<Index: Where, Target: Dim>
where
    Self::Output: DimList,
{
    const INDEX: usize;
    type Output;
}

impl<Target, Size, Tail> ReduceToOne<Here, Target> for DCons<Target, Size, Tail>
where
    Target: Dim,
    Size: Unsigned,
    Tail: DimList,
{
    const INDEX: usize = 0;
    type Output = DCons<Target, typenum::consts::U1, Tail>;
}

impl<Index, Target, NonTarget, Size, Tail> ReduceToOne<There<Index>, Target>
    for DCons<NonTarget, Size, Tail>
where
    Index: Where,
    Target: Dim,
    NonTarget: Dim,
    Size: Unsigned,
    Tail: DimList,
    Tail: ReduceToOne<Index, Target>,
{
    const INDEX: usize = 1 + <Tail as ReduceToOne<Index, Target>>::INDEX;
    type Output = DCons<NonTarget, Size, <Tail as ReduceToOne<Index, Target>>::Output>;
}

// reduce many dimension sizes to one

pub trait ReduceManyToOne<Indexes: TList, Targets: TList>
where
    Self::Output: DimList,
{
    type Output;

    fn indexes() -> Vec<usize>;
}

impl<L> ReduceManyToOne<Nil, Nil> for L
where
    L: DimList,
{
    type Output = L;

    fn indexes() -> Vec<usize> {
        vec![]
    }
}

impl<Index, IRemain, Target, TRemain, SomeDim, Size, Tail>
    ReduceManyToOne<Cons<Index, IRemain>, Cons<Target, TRemain>> for DCons<SomeDim, Size, Tail>
where
    Index: Where,
    IRemain: TList,
    Target: Dim,
    TRemain: TList,
    SomeDim: Dim,
    Size: Unsigned,
    Tail: DimList,
    Self: ReduceToOne<Index, Target>,
    <Self as ReduceToOne<Index, Target>>::Output: ReduceManyToOne<IRemain, TRemain>,
{
    type Output =
        <<Self as ReduceToOne<Index, Target>>::Output as ReduceManyToOne<IRemain, TRemain>>::Output;

    fn indexes() -> Vec<usize> {
        let mut indexes = <<Self as ReduceToOne<Index, Target>>::Output as ReduceManyToOne<
            IRemain,
            TRemain,
        >>::indexes();
        indexes.insert(0, <Self as ReduceToOne<Index, Target>>::INDEX);
        indexes
    }
}

// reverse

pub trait Reverse
where
    Self::Output: DimList,
{
    type Output;
}

impl<L> Reverse for L
where
    L: DimList + ReverseWithRemain<DNil>,
{
    type Output = <L as ReverseWithRemain<DNil>>::Output;
}

pub trait ReverseWithRemain<L: DimList>
where
    Self::Output: DimList,
{
    type Output;
}

impl<L> ReverseWithRemain<L> for DNil
where
    L: DimList,
{
    type Output = L;
}

impl<L, D, S, T> ReverseWithRemain<L> for DCons<D, S, T>
where
    L: DimList,
    D: Dim,
    S: Unsigned,
    T: DimList + ReverseWithRemain<DCons<D, S, L>>,
{
    type Output = <T as ReverseWithRemain<DCons<D, S, L>>>::Output;
}

// set equal

pub trait SetEqual<IL: TList, L: DimList> {
    type Output;
}

impl<IL, LL, LR> SetEqual<IL, LL> for LR
where
    IL: TList,
    LL: DimList + ExtractDim,
    LR: DimList + ExtractDim,
    <LR as ExtractDim>::Output: TSetEqual<IL, <LL as ExtractDim>::Output>,
{
    type Output = <<LR as ExtractDim>::Output as TSetEqual<IL, <LL as ExtractDim>::Output>>::Output;
}

// permute

pub trait Permute<IL: TList, L: TList>
where
    Self::Output: DimList,
{
    type Output;

    fn permute_index() -> Vec<usize>;
    fn reverse_permute_index() -> Vec<usize>;
}

impl Permute<Nil, Nil> for DNil {
    type Output = DNil;

    fn permute_index() -> Vec<usize> {
        vec![]
    }

    fn reverse_permute_index() -> Vec<usize> {
        vec![]
    }
}

impl<I, IL, N, TN, D, S, T> Permute<Cons<I, IL>, Cons<N, TN>> for DCons<D, S, T>
where
    I: Where,
    IL: TList,
    N: Dim,
    TN: TList,
    D: Dim,
    S: Unsigned,
    T: DimList,
    DCons<D, S, T>: SizeAt<I, N> + RemoveAt<I, N>,
    <DCons<D, S, T> as RemoveAt<I, N>>::Output: Permute<IL, TN>,
{
    type Output = DCons<
        N,
        <DCons<D, S, T> as SizeAt<I, N>>::Output,
        <<DCons<D, S, T> as RemoveAt<I, N>>::Output as Permute<IL, TN>>::Output,
    >;

    fn permute_index() -> Vec<usize> {
        let mut indexes =
            <<DCons<D, S, T> as RemoveAt<I, N>>::Output as Permute<IL, TN>>::permute_index()
                .into_iter()
                .map(|idx| if idx >= I::COUNT { idx + 1 } else { idx })
                .collect::<Vec<_>>();
        indexes.insert(0, I::COUNT);
        indexes
    }

    fn reverse_permute_index() -> Vec<usize> {
        let rev_indexes = Self::permute_index();
        let mut indexes = vec![0; rev_indexes.len()];

        for (from, to) in rev_indexes.into_iter().enumerate() {
            indexes[to] = from;
        }

        indexes
    }
}

// equal assertion

pub trait DimListEqual<Rhs: DimList> {
    type Output;
}

impl DimListEqual<DNil> for DNil {
    type Output = ();
}

impl<CurrDim, CurrSize, RTail, LTail> DimListEqual<DCons<CurrDim, CurrSize, RTail>>
    for DCons<CurrDim, CurrSize, LTail>
where
    CurrDim: Dim,
    CurrSize: Unsigned,
    RTail: DimList,
    LTail: DimList + DimListEqual<RTail>,
{
    type Output = <LTail as DimListEqual<RTail>>::Output;
}

// extend dimension

pub trait Extend<Rhs>
where
    Rhs: DimList,
    Self::Output: DimList,
{
    type Output;
}

impl<Rhs> Extend<Rhs> for DNil
where
    Rhs: DimList,
{
    type Output = Rhs;
}

impl<Rhs, DimT, Size, Tail> Extend<Rhs> for DCons<DimT, Size, Tail>
where
    Rhs: DimList,
    DimT: Dim,
    Size: Unsigned,
    Tail: DimList + Extend<Rhs>,
{
    type Output = DCons<DimT, Size, <Tail as Extend<Rhs>>::Output>;
}

// assert equal

pub trait AssertEqual<Rhs>
where
    Rhs: DimList,
    Self::Output: DimList,
{
    type Output;
}

impl AssertEqual<DNil> for DNil {
    type Output = DNil;
}

impl<DimT, Size, RTail, LTail> AssertEqual<DCons<DimT, Size, RTail>> for DCons<DimT, Size, LTail>
where
    DimT: Dim,
    Size: Unsigned,
    RTail: DimList,
    LTail: DimList + AssertEqual<RTail>,
{
    type Output = DCons<DimT, Size, <LTail as AssertEqual<RTail>>::Output>;
}

// concat

pub trait ConcatAt<Rhs, Target, Index>
where
    Rhs: DimList,
    Target: Dim,
    Index: Where,
    Self::Output: DimList,
{
    const INDEX: usize;

    type Output;
}

impl<RSize, RTail, Target, LSize, LTail> ConcatAt<DCons<Target, RSize, RTail>, Target, Here>
    for DCons<Target, LSize, LTail>
where
    RSize: Unsigned,
    RTail: DimList,
    LSize: Unsigned + std::ops::Add<RSize>,
    LTail: DimList + AssertEqual<RTail>,
    Target: Dim,
    typenum::Sum<LSize, RSize>: Unsigned,
{
    const INDEX: usize = 0;

    type Output = DCons<Target, typenum::Sum<LSize, RSize>, <LTail as AssertEqual<RTail>>::Output>;
}

impl<Index, DimT, Size, RTail, Target, LTail>
    ConcatAt<DCons<DimT, Size, RTail>, Target, There<Index>> for DCons<DimT, Size, LTail>
where
    Index: Where,
    DimT: Dim,
    Size: Unsigned,
    RTail: DimList,
    LTail: DimList + ConcatAt<RTail, Target, Index>,
    Target: Dim,
{
    const INDEX: usize = 1 + <LTail as ConcatAt<RTail, Target, Index>>::INDEX;

    type Output = DCons<DimT, Size, <LTail as ConcatAt<RTail, Target, Index>>::Output>;
}

// macros

#[macro_export]
macro_rules! make_dims {
    ( $($name:ident),+ ) => {
        $(
            pub struct $name;

            impl $crate::dim::Dim for $name {}
        )*
    };
}

#[macro_export]
macro_rules! DimListType {
    () => { $crate::dim::DNil };
    (($name:ty, $size:ty)) => { $crate::dim::DCons<$name, $size, $crate::dim::DNil> };
    (($name:ty, $size:ty), $(($names:ty, $sizes:ty)),+) => { $crate::dim::DCons<$name, $size, $crate::DimListType!($(($names, $sizes)),*)> };
}

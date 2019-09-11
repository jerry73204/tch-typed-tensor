mod broadcast;
pub use broadcast::*;

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

pub trait ExtractDim {
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
    <T as ExtractDim>::Output: TList,
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
    <T as ExtractDim>::Output: TList,
    Cons<D, <T as ExtractDim>::Output>: TIndexOf<I, E>,
{
    const INDEX: usize = <<Self as ExtractDim>::Output as TIndexOf<I, E>>::INDEX;
}

// size at

pub trait SizeAt<I: Where, E: Dim> {
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

pub trait Prepend<ND: Dim, NS: Unsigned> {
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

pub trait Append<ND: Dim, NS: Unsigned> {
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
    <T as Append<ND, NS>>::Output: DimList,
{
    type Output = DCons<D, S, <T as Append<ND, NS>>::Output>;
}

// insert at

pub trait InsertAt<I: Where, E: Dim, ND: Dim, NS: Unsigned> {
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
    <T as InsertAt<I, E, ND, NS>>::Output: DimList,
{
    type Output = DCons<D, S, <T as InsertAt<I, E, ND, NS>>::Output>;
}

// expand at

pub trait ExpandAt<I: Where, E: Dim, ND: Dim> {
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

pub trait RemoveAt<I: Where, E: Dim> {
    type Output;
}

impl<D, S, T> RemoveAt<Here, D> for DCons<D, S, T>
where
    D: Dim,
    S: Unsigned,
    T: DimList,
{
    type Output = T;
}

impl<I, E, D, S, T> RemoveAt<There<I>, E> for DCons<D, S, T>
where
    I: Where,
    E: Dim,
    D: Dim,
    S: Unsigned,
    T: DimList + RemoveAt<I, E>,
    <T as RemoveAt<I, E>>::Output: DimList,
{
    type Output = DCons<D, S, <T as RemoveAt<I, E>>::Output>;
}

// reverse

pub trait Reverse {
    type Output;
}

impl<L> Reverse for L
where
    L: DimList + ReverseWithRemain<DNil>,
{
    type Output = <L as ReverseWithRemain<DNil>>::Output;
}

pub trait ReverseWithRemain<L: DimList> {
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
    <LL as ExtractDim>::Output: TList,
    <LR as ExtractDim>::Output: TSetEqual<IL, <LL as ExtractDim>::Output>,
{
    type Output = <<LR as ExtractDim>::Output as TSetEqual<IL, <LL as ExtractDim>::Output>>::Output;
}

// reorder

pub trait Reorder<IL: TList, L: TList> {
    type Output;

    fn permute_index() -> Vec<usize>;
    fn reverse_permute_index() -> Vec<usize>;
}

impl Reorder<Nil, Nil> for DNil {
    type Output = DNil;

    fn permute_index() -> Vec<usize> {
        vec![]
    }

    fn reverse_permute_index() -> Vec<usize> {
        vec![]
    }
}

impl<I, IL, N, TN, D, S, T> Reorder<Cons<I, IL>, Cons<N, TN>> for DCons<D, S, T>
where
    I: Where,
    IL: TList,
    N: Dim,
    TN: TList,
    D: Dim,
    S: Unsigned,
    T: DimList,
    DCons<D, S, T>: SizeAt<I, N> + RemoveAt<I, N>,
    <DCons<D, S, T> as SizeAt<I, N>>::Output: Unsigned,
    <DCons<D, S, T> as RemoveAt<I, N>>::Output: Reorder<IL, TN>,
    <<DCons<D, S, T> as RemoveAt<I, N>>::Output as Reorder<IL, TN>>::Output: DimList,
{
    type Output = DCons<
        N,
        <DCons<D, S, T> as SizeAt<I, N>>::Output,
        <<DCons<D, S, T> as RemoveAt<I, N>>::Output as Reorder<IL, TN>>::Output,
    >;

    fn permute_index() -> Vec<usize> {
        let mut indexes =
            <<DCons<D, S, T> as RemoveAt<I, N>>::Output as Reorder<IL, TN>>::permute_index()
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

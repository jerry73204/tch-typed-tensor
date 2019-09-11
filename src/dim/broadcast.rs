use super::{DCons, DNil, Dim, DimList, Reverse, ReverseWithRemain};
use std::marker::PhantomData;
use typenum::marker_traits::Unsigned;

// broadcast matcher for broadcast ops

pub trait BroadcastMatcher {}

pub struct BLeft<M: BroadcastMatcher> {
    _phantom: PhantomData<M>,
}

impl<M> BroadcastMatcher for BLeft<M> where M: BroadcastMatcher {}

pub struct BRight<M: BroadcastMatcher> {
    _phantom: PhantomData<M>,
}

impl<M> BroadcastMatcher for BRight<M> where M: BroadcastMatcher {}

pub struct BEqual<M: BroadcastMatcher> {
    _phantom: PhantomData<M>,
}

impl<M> BroadcastMatcher for BEqual<M> where M: BroadcastMatcher {}

pub struct BAbscent;

impl BroadcastMatcher for BAbscent {}

pub trait BroadcastState {}

pub struct BSInit;

impl BroadcastState for BSInit {}

pub struct BSStarted;

impl BroadcastState for BSStarted {}

// internal broadcast op

pub trait BroadcastInternal<S: BroadcastState, M: BroadcastMatcher, L: DimList> {
    type Output;
}

impl BroadcastInternal<BSStarted, BAbscent, DNil> for DNil {
    type Output = DNil;
}

impl<D, S, T> BroadcastInternal<BSStarted, BAbscent, DNil> for DCons<D, S, T>
where
    D: Dim,
    S: Unsigned,
    T: DimList + BroadcastInternal<BSStarted, BAbscent, DNil>,
    <T as BroadcastInternal<BSStarted, BAbscent, DNil>>::Output: DimList,
{
    type Output = DCons<D, S, <T as BroadcastInternal<BSStarted, BAbscent, DNil>>::Output>;
}

impl<D, S, T> BroadcastInternal<BSStarted, BAbscent, DCons<D, S, T>> for DNil
where
    D: Dim,
    S: Unsigned,
    T: DimList + BroadcastInternal<BSStarted, BAbscent, DNil>,
    <T as BroadcastInternal<BSStarted, BAbscent, DNil>>::Output: DimList,
{
    type Output = DCons<D, S, <T as BroadcastInternal<BSStarted, BAbscent, DNil>>::Output>;
}

impl<M, D, S, T1, T2> BroadcastInternal<BSStarted, BEqual<M>, DCons<D, S, T1>> for DCons<D, S, T2>
where
    M: BroadcastMatcher,
    D: Dim,
    S: Unsigned,
    T1: DimList + BroadcastInternal<BSStarted, M, T2>,
    T2: DimList,
    <T1 as BroadcastInternal<BSStarted, M, T2>>::Output: DimList,
{
    type Output = DCons<D, S, <T1 as BroadcastInternal<BSStarted, M, T2>>::Output>;
}

impl<M, D, S, T1, T2> BroadcastInternal<BSStarted, BLeft<M>, DCons<D, S, T1>>
    for DCons<D, typenum::consts::U1, T2>
where
    M: BroadcastMatcher,
    D: Dim,
    S: Unsigned,
    T1: DimList + BroadcastInternal<BSStarted, M, T2>,
    T2: DimList,
    <T1 as BroadcastInternal<BSStarted, M, T2>>::Output: DimList,
{
    type Output = DCons<D, S, <T1 as BroadcastInternal<BSStarted, M, T2>>::Output>;
}

impl<M, D, S, T1, T2> BroadcastInternal<BSStarted, BRight<M>, DCons<D, typenum::consts::U1, T1>>
    for DCons<D, S, T2>
where
    M: BroadcastMatcher,
    D: Dim,
    S: Unsigned,
    T1: DimList + BroadcastInternal<BSStarted, M, T2>,
    T2: DimList,
    <T1 as BroadcastInternal<BSStarted, M, T2>>::Output: DimList,
{
    type Output = DCons<D, S, <T1 as BroadcastInternal<BSStarted, M, T2>>::Output>;
}

impl<M, D, S, T1, T2> BroadcastInternal<BSInit, BEqual<M>, DCons<D, S, T1>> for DCons<D, S, T2>
where
    M: BroadcastMatcher,
    D: Dim,
    S: Unsigned,
    T1: DimList + BroadcastInternal<BSStarted, M, T2>,
    T2: DimList,
    <T1 as BroadcastInternal<BSStarted, M, T2>>::Output: DimList,
{
    type Output = <Self as BroadcastInternal<BSStarted, BEqual<M>, DCons<D, S, T1>>>::Output;
}

impl<M, D, S, T1, T2> BroadcastInternal<BSInit, BLeft<M>, DCons<D, S, T1>>
    for DCons<D, typenum::consts::U1, T2>
where
    M: BroadcastMatcher,
    D: Dim,
    S: Unsigned,
    T1: DimList + BroadcastInternal<BSStarted, M, T2>,
    T2: DimList,
    <T1 as BroadcastInternal<BSStarted, M, T2>>::Output: DimList,
{
    type Output = <Self as BroadcastInternal<BSStarted, BLeft<M>, DCons<D, S, T1>>>::Output;
}

impl<M, D, S, T1, T2> BroadcastInternal<BSInit, BRight<M>, DCons<D, typenum::consts::U1, T1>>
    for DCons<D, S, T2>
where
    M: BroadcastMatcher,
    D: Dim,
    S: Unsigned,
    T1: DimList + BroadcastInternal<BSStarted, M, T2>,
    T2: DimList,
    <T1 as BroadcastInternal<BSStarted, M, T2>>::Output: DimList,
{
    type Output = <Self as BroadcastInternal<
        BSStarted,
        BRight<M>,
        DCons<D, typenum::consts::U1, T1>,
    >>::Output;
}

// broadcast from head

pub trait BroadcastFromHead<M: BroadcastMatcher, L: DimList> {
    type Output;
}

impl<L, R, M> BroadcastFromHead<M, L> for R
where
    L: DimList,
    R: DimList + BroadcastInternal<BSInit, M, L>,
    M: BroadcastMatcher,
    <R as BroadcastInternal<BSInit, M, L>>::Output: DimList + ReverseWithRemain<DNil>,
{
    type Output = <<R as BroadcastInternal<BSInit, M, L>>::Output as Reverse>::Output;
}

// broadcast from tail

pub trait BroadcastFromTail<M: BroadcastMatcher, L: DimList> {
    type Output;
}

impl<L, R, M> BroadcastFromTail<M, L> for R
where
    L: DimList + ReverseWithRemain<DNil>,
    R: DimList + ReverseWithRemain<DNil>,
    M: BroadcastMatcher,
    <L as ReverseWithRemain<DNil>>::Output: DimList,
    <R as ReverseWithRemain<DNil>>::Output:
        BroadcastInternal<BSInit, M, <L as ReverseWithRemain<DNil>>::Output>,
    <<R as ReverseWithRemain<DNil>>::Output as BroadcastInternal<
        BSInit,
        M,
        <L as ReverseWithRemain<DNil>>::Output,
    >>::Output: DimList,
    <<R as ReverseWithRemain<DNil>>::Output as BroadcastInternal<
        BSInit,
        M,
        <L as ReverseWithRemain<DNil>>::Output,
    >>::Output: ReverseWithRemain<DNil>,
{
    type Output = <<<R as Reverse>::Output as BroadcastInternal<
        BSInit,
        M,
        <L as Reverse>::Output,
    >>::Output as Reverse>::Output;
}

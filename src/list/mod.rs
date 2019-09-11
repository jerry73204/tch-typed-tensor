use std::marker::PhantomData;

// typed indexer

pub trait Where {
    const COUNT: usize;
}

pub struct Here;

impl Where for Here {
    const COUNT: usize = 0;
}

pub struct There<I: Where> {
    _phantom: PhantomData<I>,
}

impl<I> Where for There<I>
where
    I: Where,
{
    const COUNT: usize = 1 + I::COUNT;
}

// list

pub trait TList {
    const LENGTH: usize;
}

pub struct Cons<H, T: TList> {
    _phantom: PhantomData<(H, T)>,
}

impl<H, T> TList for Cons<H, T>
where
    T: TList,
{
    const LENGTH: usize = 1 + T::LENGTH;
}

pub struct Nil;

impl TList for Nil {
    const LENGTH: usize = 0;
}

// prepend

pub trait Prepend<H> {
    type Output;
}

impl<H, L> Prepend<H> for L
where
    L: TList,
{
    type Output = Cons<H, L>;
}

// pop front

pub trait PopFront {
    type Output;
}

impl<H, T> PopFront for Cons<H, T>
where
    T: TList,
{
    type Output = (H, T);
}

// append

pub trait Append<E> {
    type Output;
}

impl<E> Append<E> for Nil {
    type Output = Cons<E, Nil>;
}

impl<E, H, T> Append<E> for Cons<H, T>
where
    T: TList + Append<E>,
    <T as Append<E>>::Output: TList,
{
    type Output = Cons<H, <T as Append<E>>::Output>;
}

// pop back

pub trait PopBack<I: Where> {
    type Output;
}

impl<H> PopBack<Here> for Cons<H, Nil> {
    type Output = (H, Nil);
}

impl<I, H, T> PopBack<There<I>> for Cons<H, T>
where
    I: Where,
    T: TList + PopBack<I>,
{
    type Output = (H, <T as PopBack<I>>::Output);
}

// insert at

pub trait InsertAt<I: Where, A, B> {
    type Output;
}

impl<A, B, L> InsertAt<Here, A, B> for Cons<A, L>
where
    L: TList,
{
    type Output = Cons<A, Cons<B, L>>;
}

impl<I, A, B, C, L> InsertAt<There<I>, A, B> for Cons<C, L>
where
    L: TList,
    I: Where,
    L: InsertAt<I, A, B>,
    <L as InsertAt<I, A, B>>::Output: TList,
{
    type Output = Cons<C, <L as InsertAt<I, A, B>>::Output>;
}

// remove

pub trait RemoveAt<I: Where, A> {
    type Output;
}

impl<E, L> RemoveAt<Here, E> for Cons<E, L>
where
    L: TList,
{
    type Output = L;
}

impl<I, E, A, L> RemoveAt<There<I>, E> for Cons<A, L>
where
    I: Where,
    L: TList + RemoveAt<I, E>,
    <L as RemoveAt<I, E>>::Output: TList,
{
    type Output = Cons<A, <L as RemoveAt<I, E>>::Output>;
}

// index of item

pub trait IndexOf<I: Where, E> {
    const INDEX: usize;
}

impl<H, T> IndexOf<Here, H> for Cons<H, T>
where
    T: TList,
{
    const INDEX: usize = 0;
}

impl<I, E, H, T> IndexOf<There<I>, E> for Cons<H, T>
where
    I: Where,
    T: TList + IndexOf<I, E>,
{
    const INDEX: usize = 1 + <T as IndexOf<I, E>>::INDEX;
}

// reverse

pub trait Reverse {
    type Output;
}

impl<L> Reverse for L
where
    L: TList + ReverseWithRemain<Nil>,
{
    type Output = <L as ReverseWithRemain<Nil>>::Output;
}

pub trait ReverseWithRemain<L: TList> {
    type Output;
}

impl<L> ReverseWithRemain<L> for Nil
where
    L: TList,
{
    type Output = L;
}

impl<L, H, T> ReverseWithRemain<L> for Cons<H, T>
where
    L: TList,
    T: TList + ReverseWithRemain<Cons<H, L>>,
{
    type Output = <T as ReverseWithRemain<Cons<H, L>>>::Output;
}

// set equal

pub trait SetEqual<IL: TList, L: TList> {
    type Output;
}

impl SetEqual<Nil, Nil> for Nil {
    type Output = ();
}

impl<I, IL, LH, LT, RH, RT> SetEqual<Cons<I, IL>, Cons<LH, LT>> for Cons<RH, RT>
where
    I: Where,
    IL: TList,
    LT: TList,
    RT: TList,
    Cons<RH, RT>: RemoveAt<I, LH>,
    <Cons<RH, RT> as RemoveAt<I, LH>>::Output: SetEqual<IL, LT>,
{
    type Output = <<Cons<RH, RT> as RemoveAt<I, LH>>::Output as SetEqual<IL, LT>>::Output;
}

// macro

#[macro_export]
macro_rules! TListType {
    () => { $crate::list::Nil };
    ($name:ty) => { $crate::list::Cons<$name, $crate::list::Nil> };
    ($name:ty, $($names:ty),+) => { $crate::list::Cons<$name, $crate::TListType!($($names),*)> };
}

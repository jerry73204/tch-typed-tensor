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

pub trait Prepend<H>
where
    Self::Output: TList,
{
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

pub trait Append<E>
where
    Self::Output: TList,
{
    type Output;
}

impl<E> Append<E> for Nil {
    type Output = Cons<E, Nil>;
}

impl<E, H, T> Append<E> for Cons<H, T>
where
    T: TList + Append<E>,
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

pub trait InsertAt<I: Where, A, B>
where
    Self::Output: TList,
{
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
{
    type Output = Cons<C, <L as InsertAt<I, A, B>>::Output>;
}

// remove

pub trait RemoveAt<I: Where, A>
where
    Self::Output: TList,
{
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
{
    type Output = Cons<A, <L as RemoveAt<I, E>>::Output>;
}

// remove multiple items

pub trait RemoveMany<Indexes: TList, Targets: TList>
where
    Self::Output: TList,
{
    type Output;
}

impl<L> RemoveMany<Nil, Nil> for L
where
    L: TList,
{
    type Output = L;
}

impl<Index, IRemain, Target, TRemain, Head, Tail>
    RemoveMany<Cons<Index, IRemain>, Cons<Target, TRemain>> for Cons<Head, Tail>
where
    Index: Where,
    IRemain: TList,
    TRemain: TList,
    Tail: TList,
    Cons<Head, Tail>: RemoveAt<Index, Target>,
    <Cons<Head, Tail> as RemoveAt<Index, Target>>::Output: RemoveMany<IRemain, TRemain>,
{
    type Output = <<Cons<Head, Tail> as RemoveAt<Index, Target>>::Output as RemoveMany<
        IRemain,
        TRemain,
    >>::Output;
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

// index of many

pub trait IndexOfMany<Indexes: TList, Targets: TList> {
    fn indexes() -> Vec<usize>;
    fn inverse_indexes() -> Vec<usize>;
}

impl<L> IndexOfMany<Nil, Nil> for L
where
    L: TList,
{
    fn indexes() -> Vec<usize> {
        vec![]
    }

    fn inverse_indexes() -> Vec<usize> {
        (0..L::LENGTH).collect()
    }
}

impl<Index, IRemain, Target, TRemain, Head, Tail>
    IndexOfMany<Cons<Index, IRemain>, Cons<Target, TRemain>> for Cons<Head, Tail>
where
    Index: Where,
    IRemain: TList,
    TRemain: TList,
    Tail: TList,
    Self: IndexOf<Index, Target> + IndexOfMany<IRemain, TRemain>,
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

// reverse

pub trait Reverse
where
    Self::Output: TList,
{
    type Output;
}

impl<L> Reverse for L
where
    L: TList + ReverseWithRemain<Nil>,
{
    type Output = <L as ReverseWithRemain<Nil>>::Output;
}

pub trait ReverseWithRemain<L: TList>
where
    Self::Output: TList,
{
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

// check empty

pub trait EmptyList: TList {}

impl EmptyList for Nil {}

pub trait NonEmptyList: TList {}

impl<Head, Tail> NonEmptyList for Cons<Head, Tail> where Tail: TList {}

// concatenate

pub trait Concat<Rhs>
where
    Self::Output: TList,
{
    type Output;
}

impl<Rhs> Concat<Rhs> for Nil
where
    Rhs: TList,
{
    type Output = Rhs;
}

impl<Rhs, Head, Tail> Concat<Rhs> for Cons<Head, Tail>
where
    Rhs: TList,
    Tail: TList + Concat<Rhs>,
{
    type Output = Cons<Head, <Tail as Concat<Rhs>>::Output>;
}

// macro

#[macro_export]
macro_rules! TListType {
    () => { $crate::list::Nil };
    ($name:ty) => { $crate::list::Cons<$name, $crate::list::Nil> };
    ($name:ty, $($names:ty),+) => { $crate::list::Cons<$name, $crate::TListType!($($names),*)> };
}

use crate::counter::{Here, There, Where};
use std::marker::PhantomData;

// list

pub trait TList {
    const LENGTH: usize;
}

pub struct LCons<Head, Tail: TList> {
    _phantom: PhantomData<(Head, Tail)>,
}

impl<Head, Tail> LCons<Head, Tail>
where
    Tail: TList,
{
    pub fn new() -> LCons<Head, Tail> {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Head, Tail> TList for LCons<Head, Tail>
where
    Tail: TList,
{
    const LENGTH: usize = 1 + Tail::LENGTH;
}

pub struct LNil;

impl TList for LNil {
    const LENGTH: usize = 0;
}

// {,non-}empty list trait

pub trait EmptyList: TList {}

impl EmptyList for LNil {}

pub trait NonEmptyList: TList {}

impl<Head, Tail> NonEmptyList for LCons<Head, Tail> where Tail: TList {}

// prepend

pub trait LPrepend<Head>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Item, List> LPrepend<Item> for List
where
    List: TList,
{
    type Output = LCons<Item, List>;
}

pub type LPrependOutput<List, Item> = <List as LPrepend<Item>>::Output;

// append

pub trait LAppend<Item>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Item> LAppend<Item> for LNil {
    type Output = LCons<Item, LNil>;
}

impl<Item, Head, Tail> LAppend<Item> for LCons<Head, Tail>
where
    Tail: TList + LAppend<Item>,
{
    type Output = LCons<Head, LAppendOutput<Tail, Item>>;
}

pub type LAppendOutput<List, Item> = <List as LAppend<Item>>::Output;

// insert at

pub trait LInsertAt<Item, Target, Index>
where
    Index: Where,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Target, Item, Tail> LInsertAt<Item, Target, Here> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Output = LCons<Target, LCons<Item, Tail>>;
}

impl<Item, Target, Index, NonTarget, Tail> LInsertAt<Item, Target, There<Index>>
    for LCons<NonTarget, Tail>
where
    Tail: TList + LInsertAt<Item, Target, Index>,
    Index: Where,
{
    type Output = LCons<NonTarget, LInsertAtOutput<Tail, Item, Target, Index>>;
}

pub type LInsertAtOutput<List, Item, Target, Index> =
    <List as LInsertAt<Item, Target, Index>>::Output;

// remove

pub trait LRemoveAt<Target, Index>
where
    Index: Where,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Target, Tail> LRemoveAt<Target, Here> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Output = Tail;
}

impl<Target, Index, NonTarget, Tail> LRemoveAt<Target, There<Index>> for LCons<NonTarget, Tail>
where
    Index: Where,
    Tail: TList + LRemoveAt<Target, Index>,
{
    type Output = LCons<NonTarget, LRemoveAtOutput<Tail, Target, Index>>;
}

pub type LRemoveAtOutput<List, Target, Index> = <List as LRemoveAt<Target, Index>>::Output;

// remove multiple items

pub trait LRemoveMany<Targets, Indexes>
where
    Targets: TList,
    Indexes: TList,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<List> LRemoveMany<LNil, LNil> for List
where
    List: TList,
{
    type Output = List;
}

impl<Index, IRemain, Target, TRemain, Head, Tail>
    LRemoveMany<LCons<Target, TRemain>, LCons<Index, IRemain>> for LCons<Head, Tail>
where
    Index: Where,
    IRemain: TList,
    TRemain: TList,
    Tail: TList,
    Self: LRemoveAt<Target, Index>,
    <Self as LRemoveAt<Target, Index>>::Output: LRemoveMany<TRemain, IRemain>,
{
    type Output = LRemoveManyOutput<LRemoveAtOutput<Self, Target, Index>, TRemain, IRemain>;
}

pub type LRemoveManyOutput<List, Targets, Indexes> =
    <List as LRemoveMany<Targets, Indexes>>::Output;

// index of item

pub trait LIndexOf<Item, Index>
where
    Self: TList,
    Index: Where,
{
    const INDEX: usize;
}

impl<Target, Tail> LIndexOf<Target, Here> for LCons<Target, Tail>
where
    Tail: TList,
{
    const INDEX: usize = 0;
}

impl<Target, Index, NonTarget, Tail> LIndexOf<Target, There<Index>> for LCons<NonTarget, Tail>
where
    Index: Where,
    Tail: TList + LIndexOf<Target, Index>,
{
    const INDEX: usize = 1 + <Tail as LIndexOf<Target, Index>>::INDEX;
}

// index of many

pub trait LIndexOfMany<Targets, Indexes>
where
    Self: TList,
    Targets: TList,
    Indexes: TList,
{
    fn indexes() -> Vec<usize>;
    fn inverse_indexes() -> Vec<usize>;
}

impl<List> LIndexOfMany<LNil, LNil> for List
where
    List: TList,
{
    fn indexes() -> Vec<usize> {
        vec![]
    }

    fn inverse_indexes() -> Vec<usize> {
        (0..List::LENGTH).collect()
    }
}

impl<Index, IRemain, Target, TRemain, Head, Tail>
    LIndexOfMany<LCons<Target, TRemain>, LCons<Index, IRemain>> for LCons<Head, Tail>
where
    Index: Where,
    IRemain: TList,
    TRemain: TList,
    Tail: TList,
    Self: LIndexOf<Target, Index> + LIndexOfMany<TRemain, IRemain>,
{
    fn indexes() -> Vec<usize> {
        let mut indexes = <Self as LIndexOfMany<TRemain, IRemain>>::indexes();
        indexes.insert(0, <Self as LIndexOf<Target, Index>>::INDEX);
        indexes
    }

    fn inverse_indexes() -> Vec<usize> {
        let mut indexes = <Self as LIndexOfMany<TRemain, IRemain>>::inverse_indexes();
        indexes.remove_item(&<Self as LIndexOf<Target, Index>>::INDEX);
        indexes
    }
}

// reverse

pub trait LReverseWithTail<Tail>
where
    Tail: TList,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Tail> LReverseWithTail<Tail> for LNil
where
    Tail: TList,
{
    type Output = Tail;
}

impl<ReversedTail, Head, Tail> LReverseWithTail<ReversedTail> for LCons<Head, Tail>
where
    ReversedTail: TList,
    Tail: TList + LReverseWithTail<LCons<Head, ReversedTail>>,
{
    type Output = LReverseWithTailOutput<Tail, LCons<Head, ReversedTail>>;
}

pub type LReverseWithTailOutput<List, ReversedTail> =
    <List as LReverseWithTail<ReversedTail>>::Output;
pub type LReverseOutput<List> = LReverseWithTailOutput<List, LNil>;

// set equal

pub trait LSetEqual<Rhs, Indexes>
where
    Rhs: TList,
    Indexes: TList,
    Self: TList,
{
    type Output;
}

impl LSetEqual<LNil, LNil> for LNil {
    type Output = ();
}

impl<LHead, LTail, RHead, RTail, Index, IRemain>
    LSetEqual<LCons<RHead, RTail>, LCons<Index, IRemain>> for LCons<LHead, LTail>
where
    Index: Where,
    IRemain: TList,
    LTail: TList,
    RTail: TList,
    Self: LRemoveAt<RHead, Index>,
    LRemoveAtOutput<Self, RHead, Index>: LSetEqual<RTail, IRemain>,
{
    type Output = LSetEqualOutput<LRemoveAtOutput<Self, RHead, Index>, RTail, IRemain>;
}

pub type LSetEqualOutput<Lhs, Rhs, Indexes> = <Lhs as LSetEqual<Rhs, Indexes>>::Output;

// assert equal

pub trait LAssertEqual<Rhs>
where
    Self: TList,
{
    type Output;
}

impl LAssertEqual<LNil> for LNil {
    type Output = ();
}

impl<Item, LTail, RTail> LAssertEqual<LCons<Item, RTail>> for LCons<Item, LTail>
where
    LTail: TList + LAssertEqual<RTail>,
    RTail: TList,
{
    type Output = LAssertEqualOutput<LTail, RTail>;
}

pub type LAssertEqualOutput<Lhs, Rhs> = <Lhs as LAssertEqual<Rhs>>::Output;

// concatenate

pub trait LConcat<Rhs>
where
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

impl<Rhs> LConcat<Rhs> for LNil
where
    Rhs: TList,
{
    type Output = Rhs;
}

impl<Rhs, Head, Tail> LConcat<Rhs> for LCons<Head, Tail>
where
    Rhs: TList,
    Tail: TList + LConcat<Rhs>,
{
    type Output = LCons<Head, LConcatOutput<Tail, Rhs>>;
}

pub type LConcatOutput<Lhs, Rhs> = <Lhs as LConcat<Rhs>>::Output;

// macro

#[macro_export]
macro_rules! TListType {
    () => { $crate::list::LNil };
    ($name:ty) => { $crate::list::LCons<$name, $crate::list::LNil> };
    ($name:ty, $($names:ty),+) => { $crate::list::LCons<$name, $crate::TListType!($($names),*)> };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TListType;

    struct A;
    struct B;
    struct C;
    struct D;
    struct E;

    type EmptyList = TListType! {};
    type SomeList = TListType! {A, B, C};
    type AnotherList = TListType! {D, E};

    type Assert1 = LAssertEqualOutput<LPrependOutput<EmptyList, A>, TListType! {A}>;
    type Assert2 = LAssertEqualOutput<LAppendOutput<EmptyList, D>, TListType! {D}>;

    type Assert3 = LAssertEqualOutput<LPrependOutput<SomeList, D>, TListType! {D, A, B, C}>;
    type Assert4 = LAssertEqualOutput<LAppendOutput<SomeList, D>, TListType! {A, B, C, D}>;

    type Assert5<Idx> =
        LAssertEqualOutput<LInsertAtOutput<SomeList, D, B, Idx>, TListType! {A, B, D, C}>;
    type Assert6<Idx> =
        LAssertEqualOutput<LInsertAtOutput<SomeList, D, C, Idx>, TListType! {A, B, C, D}>;

    type Assert7<Idx> = LAssertEqualOutput<LRemoveAtOutput<SomeList, B, Idx>, TListType! {A, C}>;

    type Assert8<Idx> =
        LAssertEqualOutput<LRemoveManyOutput<SomeList, TListType! {A, C}, Idx>, TListType! {B}>;

    type Assert9<Idx> =
        LAssertEqualOutput<LRemoveManyOutput<SomeList, TListType! {B, A, C}, Idx>, TListType! {}>;

    type Assert10 = LAssertEqualOutput<LReverseOutput<SomeList>, TListType! {C, B, A}>;

    type Assert11<Idx> = LSetEqualOutput<SomeList, TListType! {C, A, B}, Idx>;

    type Assert12 =
        LAssertEqualOutput<LConcatOutput<SomeList, AnotherList>, TListType! {A, B, C, D, E}>;

    #[test]
    fn tlist_test() {
        // prepend empty list
        let _: Assert1 = ();

        // append empty list
        let _: Assert2 = ();

        // prepend non-empty list
        let _: Assert3 = ();

        // append non-empty list
        let _: Assert4 = ();

        // insert in middle
        let _: Assert5<_> = ();

        // insert at end
        let _: Assert6<_> = ();

        // remove
        let _: Assert7<_> = ();

        // remove multiple items
        let _: Assert8<_> = ();

        // remove until empty
        let _: Assert9<_> = ();

        // reverse list
        let _: Assert10 = ();

        // assert identical set of items
        let _: Assert11<_> = ();

        // concat
        let _: Assert12 = ();

        // index of item
        assert_eq!(<SomeList as LIndexOf<A, _>>::INDEX, 0);
        assert_eq!(<SomeList as LIndexOf<B, _>>::INDEX, 1);
        assert_eq!(<SomeList as LIndexOf<C, _>>::INDEX, 2);

        // index of multiple items
        assert_eq!(
            <SomeList as LIndexOfMany<TListType! {C, A, B}, _>>::indexes(),
            &[2, 0, 1]
        );
    }
}

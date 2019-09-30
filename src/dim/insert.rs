use super::{DFromKVList, DFromKVListFunctor, DimList, DimName, DimSize};
use std::marker::PhantomData;
use type_freak::{
    counter::Counter,
    functional::{ApplyFunctor, Functor},
    kvlist::{
        KVAppend, KVAppendFunctor, KVInsertAt, KVInsertAtFunctor, KVPrepend, KVPrependFunctor,
    },
};

// insert at

/// A [Functor] that inserts a `NewName`-`NewSize` pair at after `Target` into input [DimList].
pub struct DInsertAtFunctor<NewName, NewSize, Target, Index>
where
    NewName: DimName,
    NewSize: DimSize,
    Target: DimName,
    Index: Counter,
{
    _phantom: PhantomData<(NewName, NewSize, Target, Index)>,
}

pub type DInsertAt<List, NewName, NewSize, Target, Index> =
    ApplyFunctor<DInsertAtFunctor<NewName, NewSize, Target, Index>, List>;

impl<List, NewName, NewSize, Target, Index> Functor<List>
    for DInsertAtFunctor<NewName, NewSize, Target, Index>
where
    NewName: DimName,
    NewSize: DimSize,
    List: DimList,
    Target: DimName,
    Index: Counter,
    KVInsertAtFunctor<NewName, NewSize, Target, Index>: Functor<List::List>,
    DFromKVListFunctor: Functor<KVInsertAt<List::List, NewName, NewSize, Target, Index>>,
{
    type Output = DFromKVList<KVInsertAt<List::List, NewName, NewSize, Target, Index>>;
}

// append

/// A [Functor] that appends a `NewName`-`NewSize` pair into input [DimList].
pub struct DAppendFunctor<NewName, NewSize>
where
    NewName: DimName,
    NewSize: DimSize,
{
    _phantom: PhantomData<(NewName, NewSize)>,
}

pub type DAppend<List, NewName, NewSize> = ApplyFunctor<DAppendFunctor<NewName, NewSize>, List>;

impl<List, NewName, NewSize> Functor<List> for DAppendFunctor<NewName, NewSize>
where
    NewName: DimName,
    NewSize: DimSize,
    List: DimList,
    KVAppendFunctor<NewName, NewSize>: Functor<List::List>,
    DFromKVListFunctor: Functor<KVAppend<List::List, NewName, NewSize>>,
{
    type Output = DFromKVList<KVAppend<List::List, NewName, NewSize>>;
}

// prepend

/// A [Functor] that prepends a `NewName`-`NewSize` pair into input [DimList].
pub struct DPrependFunctor<NewName, NewSize>
where
    NewName: DimName,
    NewSize: DimSize,
{
    _phantom: PhantomData<(NewName, NewSize)>,
}

pub type DPrepend<List, NewName, NewSize> = ApplyFunctor<DPrependFunctor<NewName, NewSize>, List>;

impl<List, NewName, NewSize> Functor<List> for DPrependFunctor<NewName, NewSize>
where
    NewName: DimName,
    NewSize: DimSize,
    List: DimList,
    KVPrependFunctor<NewName, NewSize>: Functor<List::List>,
    DFromKVListFunctor: Functor<KVPrepend<List::List, NewName, NewSize>>,
{
    type Output = DFromKVList<KVPrepend<List::List, NewName, NewSize>>;
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{define_dim_names, dim::Known, Dims};
    use type_freak::control::IfSameOutput;
    use typenum::consts::*;

    define_dim_names! {A, B, C, D, E}

    type EmptyDims = Dims![];
    type SomeDims = Dims![(A, U3), (B, U2), (C, U4)];

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    type Assert2 =
        AssertSame<DPrepend<SomeDims, D, Known<U5>>, Dims![(D, U5), (A, U3), (B, U2), (C, U4)]>;
    type Assert3 = AssertSame<DPrepend<EmptyDims, D, Known<U5>>, Dims![(D, U5)]>;

    type Assert4 =
        AssertSame<DAppend<SomeDims, D, Known<U5>>, Dims![(A, U3), (B, U2), (C, U4), (D, U5)]>;
    type Assert5 = AssertSame<DAppend<EmptyDims, D, Known<U5>>, Dims![(D, U5)]>;

    type Assert6<Idx> = AssertSame<
        DInsertAt<SomeDims, D, Known<U5>, B, Idx>,
        Dims![(A, U3), (D, U5), (B, U2), (C, U4)],
    >;

    #[test]
    fn dim_test() {
        // prepend to non-empty dims
        let _: Assert2 = ();

        // prepend to empty dims
        let _: Assert3 = ();

        // append to non-empty dims
        let _: Assert4 = ();

        // append to empty dims
        let _: Assert5 = ();

        // insert single dim
        let _: Assert6<_> = ();
    }
}

use super::{
    marker::MatrixDim, BcastAbscent, BroadcastIndicator, DBroadcastBothReversely,
    DBroadcastBothReverselyFunctor, DReverse, DReverseFunctor, DimList, DimName, DimSize, Known,
    Unknown,
};
use crate::{DimsVerbose, DimsWithTailVerbose};
use std::marker::PhantomData;
use type_freak::functional::{ApplyFunctor, Functor};
use typenum::Unsigned;

// two-dimensional matrix multiplication

pub struct DMatMulFunctor<Rhs>
where
    Rhs: MatrixDim,
{
    _phantom: PhantomData<Rhs>,
}

pub type DMatMul<Lhs, Rhs> = ApplyFunctor<DMatMulFunctor<Rhs>, Lhs>;

// (m x n) . (n x p) -> (m x p)
impl<MDim, MSize, NDimL, NDimR, NSize, PDim, PSize>
    Functor<DimsVerbose![(MDim, MSize), (NDimL, Known<NSize>)]>
    for DMatMulFunctor<DimsVerbose![(NDimR, Known<NSize>), (PDim, PSize)]>
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    PDim: DimName,
    PSize: DimSize,
{
    type Output = DimsVerbose![(MDim, MSize), (PDim, PSize)];
}

// (m x ?) . (n x p) -> (m x p)
impl<MDim, MSize, NDimL, NDimR, NSize, PDim, PSize>
    Functor<DimsVerbose![(MDim, MSize), (NDimL, Unknown)]>
    for DMatMulFunctor<DimsVerbose![(NDimR, Known<NSize>), (PDim, PSize)]>
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    PDim: DimName,
    PSize: DimSize,
{
    type Output = DimsVerbose![(MDim, MSize), (PDim, PSize)];
}

// (m x n) . (? x p) -> (m x p)
impl<MDim, MSize, NDimL, NDimR, NSize, PDim, PSize>
    Functor<DimsVerbose![(MDim, MSize), (NDimL, Known<NSize>)]>
    for DMatMulFunctor<DimsVerbose![(NDimR, Unknown), (PDim, PSize)]>
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    PDim: DimName,
    PSize: DimSize,
{
    type Output = DimsVerbose![(MDim, MSize), (PDim, PSize)];
}

// (m x ?) . (? x p) -> (m x p)
impl<MDim, MSize, NDimL, NDimR, PDim, PSize> Functor<DimsVerbose![(MDim, MSize), (NDimL, Unknown)]>
    for DMatMulFunctor<DimsVerbose![(NDimR, Unknown), (PDim, PSize)]>
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    PDim: DimName,
    PSize: DimSize,
{
    type Output = DimsVerbose![(MDim, MSize), (PDim, PSize)];
}

// broadcasted matmul

pub struct DMatMulBroadcastedFunctor<Rhs, Matcher>
where
    Rhs: DimList,
    Matcher: BroadcastIndicator,
{
    _phantom: PhantomData<(Rhs, Matcher)>,
}

pub type DMatMulBroadcasted<Lhs, Rhs, Matcher> =
    ApplyFunctor<DMatMulBroadcastedFunctor<Rhs, Matcher>, Lhs>;

impl<Lhs, Rhs, Matcher> Functor<Lhs> for DMatMulBroadcastedFunctor<Rhs, Matcher>
where
    Lhs: DimList,
    Rhs: DimList,
    Matcher: BroadcastIndicator,
    DReverseFunctor: Functor<Lhs>
        + Functor<Rhs>
        + Functor<DMatMulReversely<DReverse<Lhs>, DReverse<Rhs>, Matcher>>,
    DMatMulReverselyFunctor<DReverse<Rhs>, Matcher>: Functor<DReverse<Lhs>>,
    DReverse<Rhs>: DimList,
{
    type Output = DReverse<DMatMulReversely<DReverse<Lhs>, DReverse<Rhs>, Matcher>>;
}

// auxiliary trait for DMatMulBroadcastable broadcastable matrix multiplication

pub struct DMatMulReverselyFunctor<Rhs, Matcher>
where
    Rhs: DimList,
    Matcher: BroadcastIndicator,
{
    _phantom: PhantomData<(Rhs, Matcher)>,
}

pub type DMatMulReversely<Lhs, Rhs, Matcher> =
    ApplyFunctor<DMatMulReverselyFunctor<Rhs, Matcher>, Lhs>;

// [n] x [n] -> []
impl<LDim, RDim, Size> Functor<DimsVerbose![(LDim, Known<Size>)]>
    for DMatMulReverselyFunctor<DimsVerbose![(RDim, Known<Size>)], BcastAbscent>
where
    LDim: DimName,
    RDim: DimName,
    Size: Unsigned,
{
    type Output = DimsVerbose![];
}

// [?] x [n] -> []
impl<LDim, RDim, Size> Functor<DimsVerbose![(LDim, Known<Size>)]>
    for DMatMulReverselyFunctor<DimsVerbose![(RDim, Unknown)], BcastAbscent>
where
    LDim: DimName,
    RDim: DimName,
    Size: Unsigned,
{
    type Output = DimsVerbose![];
}

// [n] x [?] -> []
impl<LDim, RDim, Size> Functor<DimsVerbose![(LDim, Unknown)]>
    for DMatMulReverselyFunctor<DimsVerbose![(RDim, Known<Size>)], BcastAbscent>
where
    LDim: DimName,
    RDim: DimName,
    Size: Unsigned,
{
    type Output = DimsVerbose![];
}

// [?] x [?] -> []
impl<LDim, RDim> Functor<DimsVerbose![(LDim, Unknown)]>
    for DMatMulReverselyFunctor<DimsVerbose![(RDim, Unknown)], BcastAbscent>
where
    LDim: DimName,
    RDim: DimName,
{
    type Output = DimsVerbose![];
}

// [..., m, n] x [n, p] -> [..., m, p]
impl<MDim, MSize, NDimL, NDimR, NSize, PDim, PSize, LTail, RTail, Matcher>
    Functor<DimsWithTailVerbose![(NDimL, Known<NSize>), (MDim, MSize); LTail]>
    for DMatMulReverselyFunctor<
        DimsWithTailVerbose![(PDim, PSize), (NDimR, Known<NSize>); RTail],
        Matcher,
    >
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    PDim: DimName,
    PSize: DimSize,
    LTail: DimList,
    RTail: DimList,
    Matcher: BroadcastIndicator,
    DBroadcastBothReverselyFunctor<RTail, Matcher>: Functor<LTail>,
    DBroadcastBothReversely<LTail, RTail, Matcher>: DimList,
{
    type Output = DimsWithTailVerbose![(PDim, PSize), (MDim, MSize); DBroadcastBothReversely<LTail, RTail, Matcher>];
}

// [..., m, ?] x [n, p] -> [..., m, p]
impl<MDim, MSize, NDimL, NDimR, NSize, PDim, PSize, LTail, RTail, Matcher>
    Functor<DimsWithTailVerbose![(NDimL, Unknown), (MDim, MSize); LTail]>
    for DMatMulReverselyFunctor<
        DimsWithTailVerbose![(PDim, PSize), (NDimR, Known<NSize>); RTail],
        Matcher,
    >
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    PDim: DimName,
    PSize: DimSize,
    LTail: DimList,
    RTail: DimList,
    Matcher: BroadcastIndicator,
    DBroadcastBothReverselyFunctor<RTail, Matcher>: Functor<LTail>,
    DBroadcastBothReversely<LTail, RTail, Matcher>: DimList,
{
    type Output = DimsWithTailVerbose![(PDim, PSize), (MDim, MSize); DBroadcastBothReversely<LTail, RTail, Matcher>];
}

// [..., m, n] x [?, p] -> [..., m, p]
impl<MDim, MSize, NDimL, NDimR, NSize, PDim, PSize, LTail, RTail, Matcher>
    Functor<DimsWithTailVerbose![(NDimL, Known<NSize>), (MDim, MSize); LTail]>
    for DMatMulReverselyFunctor<
        DimsWithTailVerbose![(PDim, PSize), (NDimR, Unknown); RTail],
        Matcher,
    >
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    PDim: DimName,
    PSize: DimSize,
    LTail: DimList,
    RTail: DimList,
    Matcher: BroadcastIndicator,
    DBroadcastBothReverselyFunctor<RTail, Matcher>: Functor<LTail>,
    DBroadcastBothReversely<LTail, RTail, Matcher>: DimList,
{
    type Output = DimsWithTailVerbose![(PDim, PSize), (MDim, MSize); DBroadcastBothReversely<LTail, RTail, Matcher>];
}

// [..., m, ?] x [?, p] -> [..., m, p]
impl<MDim, MSize, NDimL, NDimR, PDim, PSize, LTail, RTail, Matcher>
    Functor<DimsWithTailVerbose![(NDimL, Unknown), (MDim, MSize); LTail]>
    for DMatMulReverselyFunctor<
        DimsWithTailVerbose![(PDim, PSize), (NDimR, Unknown); RTail],
        Matcher,
    >
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    PDim: DimName,
    PSize: DimSize,
    LTail: DimList,
    RTail: DimList,
    Matcher: BroadcastIndicator,
    DBroadcastBothReverselyFunctor<RTail, Matcher>: Functor<LTail>,
    DBroadcastBothReversely<LTail, RTail, Matcher>: DimList,
{
    type Output = DimsWithTailVerbose![(PDim, PSize), (MDim, MSize); DBroadcastBothReversely<LTail, RTail, Matcher>];
}

// [n] x [..., n, p] -> [..., p]
impl<NDimL, NDimR, NSize, PDim, PSize, Tail> Functor<DimsVerbose![(NDimL, Known<NSize>)]>
    for DMatMulReverselyFunctor<
        DimsWithTailVerbose![(PDim, PSize), (NDimR, Known<NSize>); Tail],
        BcastAbscent,
    >
where
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    PDim: DimName,
    PSize: DimSize,
    Tail: DimList,
{
    type Output = DimsWithTailVerbose![(PDim, PSize); Tail];
}

// [?] x [..., n, p] -> [..., p]
impl<NDimL, NDimR, NSize, PDim, PSize, Tail> Functor<DimsVerbose![(NDimL, Unknown)]>
    for DMatMulReverselyFunctor<
        DimsWithTailVerbose![(PDim, PSize), (NDimR, Known<NSize>); Tail],
        BcastAbscent,
    >
where
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    PDim: DimName,
    PSize: DimSize,
    Tail: DimList,
{
    type Output = DimsWithTailVerbose![(PDim, PSize); Tail];
}

// [n] x [..., ?, p] -> [..., p]
impl<NDimL, NDimR, NSize, PDim, PSize, Tail> Functor<DimsVerbose![(NDimL, Known<NSize>)]>
    for DMatMulReverselyFunctor<
        DimsWithTailVerbose![(PDim, PSize), (NDimR, Unknown); Tail],
        BcastAbscent,
    >
where
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    PDim: DimName,
    PSize: DimSize,
    Tail: DimList,
{
    type Output = DimsWithTailVerbose![(PDim, PSize); Tail];
}

// [?] x [..., ?, p] -> [..., p]
impl<NDimL, NDimR, PDim, PSize, Tail> Functor<DimsVerbose![(NDimL, Unknown)]>
    for DMatMulReverselyFunctor<
        DimsWithTailVerbose![(PDim, PSize), (NDimR, Unknown); Tail],
        BcastAbscent,
    >
where
    NDimL: DimName,
    NDimR: DimName,
    PDim: DimName,
    PSize: DimSize,
    Tail: DimList,
{
    type Output = DimsWithTailVerbose![(PDim, PSize); Tail];
}

// [..., m, n] x [n] -> [..., m]
impl<MDim, MSize, NDimL, NDimR, NSize, Tail>
    Functor<DimsWithTailVerbose![(NDimL, Known<NSize>), (MDim, MSize); Tail]>
    for DMatMulReverselyFunctor<DimsVerbose![(NDimR, Known<NSize>)], BcastAbscent>
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    Tail: DimList,
{
    type Output = DimsWithTailVerbose![(MDim, MSize); Tail];
}

// [..., m, ?] x [n] -> [..., m]
impl<MDim, MSize, NDimL, NDimR, NSize, Tail>
    Functor<DimsWithTailVerbose![(NDimL, Unknown), (MDim, MSize); Tail]>
    for DMatMulReverselyFunctor<DimsVerbose![(NDimR, Known<NSize>)], BcastAbscent>
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    Tail: DimList,
{
    type Output = DimsWithTailVerbose![(MDim, MSize); Tail];
}

// [..., m, n] x [?] -> [..., m]
impl<MDim, MSize, NDimL, NDimR, NSize, Tail>
    Functor<DimsWithTailVerbose![(NDimL, Known<NSize>), (MDim, MSize); Tail]>
    for DMatMulReverselyFunctor<DimsVerbose![(NDimR, Unknown)], BcastAbscent>
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    NSize: Unsigned,
    Tail: DimList,
{
    type Output = DimsWithTailVerbose![(MDim, MSize); Tail];
}

// [..., m, ?] x [?] -> [..., m]
impl<MDim, MSize, NDimL, NDimR, Tail>
    Functor<DimsWithTailVerbose![(NDimL, Unknown), (MDim, MSize); Tail]>
    for DMatMulReverselyFunctor<DimsVerbose![(NDimR, Unknown)], BcastAbscent>
where
    MDim: DimName,
    MSize: DimSize,
    NDimL: DimName,
    NDimR: DimName,
    Tail: DimList,
{
    type Output = DimsWithTailVerbose![(MDim, MSize); Tail];
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{define_dim_names, Dims};
    use type_freak::control::{IfOutput, IfSameOutput};
    use typenum::consts::*;

    define_dim_names! {M, N, P, X, Y, Z, W, A}

    type VecDims1 = Dims![(Z, U2)];
    type VecDims2 = Dims![(W, U2)];
    type VecDims3 = Dims![(Z,)];
    type VecDims4 = Dims![(W,)];

    type MatDims1 = Dims![(M, U3), (N, U2)];
    type MatDims2 = Dims![(A, U2), (P, U5)];
    type MatDims3 = Dims![(M, U3), (N,)];
    type MatDims4 = Dims![(A,), (P, U5)];

    type BatchMatDims1 = Dims![(X, U7), (Y, U1), (M, U3), (N, U2)];
    type BatchMatDims2 = Dims![(Y, U11), (A, U2), (P, U5)];
    type BatchMatDims3 = Dims![(X, U7), (Y, U1), (M, U3), (N,)];
    type BatchMatDims4 = Dims![(Y, U11), (A,), (P, U5)];

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    // non-broadcasted matmul
    type Assert1 = IfOutput<
        (),
        (
            AssertSame<DMatMul<MatDims1, MatDims2>, Dims![(M, U3), (P, U5)]>,
            AssertSame<DMatMul<MatDims1, MatDims4>, Dims![(M, U3), (P, U5)]>,
            AssertSame<DMatMul<MatDims3, MatDims2>, Dims![(M, U3), (P, U5)]>,
            AssertSame<DMatMul<MatDims3, MatDims4>, Dims![(M, U3), (P, U5)]>,
        ),
    >;

    // vector x vector
    type Assert2<Matcher> = IfOutput<
        (),
        (
            AssertSame<DMatMulBroadcasted<VecDims1, VecDims2, Matcher>, Dims![]>,
            AssertSame<DMatMulBroadcasted<VecDims1, VecDims4, Matcher>, Dims![]>,
            AssertSame<DMatMulBroadcasted<VecDims3, VecDims2, Matcher>, Dims![]>,
            AssertSame<DMatMulBroadcasted<VecDims3, VecDims4, Matcher>, Dims![]>,
        ),
    >;

    // matrix x vector
    type Assert3<Matcher> = IfOutput<
        (),
        (
            AssertSame<DMatMulBroadcasted<MatDims1, VecDims2, Matcher>, Dims![(M, U3)]>,
            AssertSame<DMatMulBroadcasted<MatDims1, VecDims4, Matcher>, Dims![(M, U3)]>,
            AssertSame<DMatMulBroadcasted<MatDims3, VecDims2, Matcher>, Dims![(M, U3)]>,
            AssertSame<DMatMulBroadcasted<MatDims3, VecDims4, Matcher>, Dims![(M, U3)]>,
        ),
    >;

    // vector x matrix
    type Assert4<Matcher> = IfOutput<
        (),
        (
            AssertSame<DMatMulBroadcasted<VecDims1, MatDims2, Matcher>, Dims![(P, U5)]>,
            AssertSame<DMatMulBroadcasted<VecDims1, MatDims4, Matcher>, Dims![(P, U5)]>,
            AssertSame<DMatMulBroadcasted<VecDims3, MatDims2, Matcher>, Dims![(P, U5)]>,
            AssertSame<DMatMulBroadcasted<VecDims3, MatDims4, Matcher>, Dims![(P, U5)]>,
        ),
    >;

    // matrix x matrix
    type Assert5<Matcher> = IfOutput<
        (),
        (
            AssertSame<DMatMulBroadcasted<MatDims1, MatDims2, Matcher>, Dims![(M, U3), (P, U5)]>,
            AssertSame<DMatMulBroadcasted<MatDims1, MatDims4, Matcher>, Dims![(M, U3), (P, U5)]>,
            AssertSame<DMatMulBroadcasted<MatDims3, MatDims2, Matcher>, Dims![(M, U3), (P, U5)]>,
            AssertSame<DMatMulBroadcasted<MatDims1, MatDims4, Matcher>, Dims![(M, U3), (P, U5)]>,
        ),
    >;

    // batched matrix x vector
    type Assert6<Matcher> = IfOutput<
        (),
        (
            AssertSame<
                DMatMulBroadcasted<BatchMatDims1, VecDims1, Matcher>,
                Dims![(X, U7), (Y, U1), (M, U3)],
            >,
            AssertSame<
                DMatMulBroadcasted<BatchMatDims1, VecDims3, Matcher>,
                Dims![(X, U7), (Y, U1), (M, U3)],
            >,
            AssertSame<
                DMatMulBroadcasted<BatchMatDims3, VecDims1, Matcher>,
                Dims![(X, U7), (Y, U1), (M, U3)],
            >,
            AssertSame<
                DMatMulBroadcasted<BatchMatDims3, VecDims3, Matcher>,
                Dims![(X, U7), (Y, U1), (M, U3)],
            >,
        ),
    >;

    // vector x batched matrix
    type Assert7<Matcher> = IfOutput<
        (),
        (
            AssertSame<
                DMatMulBroadcasted<VecDims1, BatchMatDims2, Matcher>,
                Dims![(Y, U11), (P, U5)],
            >,
            AssertSame<
                DMatMulBroadcasted<VecDims1, BatchMatDims4, Matcher>,
                Dims![(Y, U11), (P, U5)],
            >,
            AssertSame<
                DMatMulBroadcasted<VecDims3, BatchMatDims2, Matcher>,
                Dims![(Y, U11), (P, U5)],
            >,
            AssertSame<
                DMatMulBroadcasted<VecDims3, BatchMatDims4, Matcher>,
                Dims![(Y, U11), (P, U5)],
            >,
        ),
    >;

    // batched matrix x matrix
    type Assert8<Matcher> = IfOutput<
        (),
        (
            AssertSame<
                DMatMulBroadcasted<BatchMatDims1, MatDims2, Matcher>,
                Dims![(X, U7), (Y, U1), (M, U3), (P, U5)],
            >,
            AssertSame<
                DMatMulBroadcasted<BatchMatDims1, MatDims4, Matcher>,
                Dims![(X, U7), (Y, U1), (M, U3), (P, U5)],
            >,
            AssertSame<
                DMatMulBroadcasted<BatchMatDims3, MatDims2, Matcher>,
                Dims![(X, U7), (Y, U1), (M, U3), (P, U5)],
            >,
            AssertSame<
                DMatMulBroadcasted<BatchMatDims3, MatDims4, Matcher>,
                Dims![(X, U7), (Y, U1), (M, U3), (P, U5)],
            >,
        ),
    >;

    // batched matrix x batched matrix
    type Assert9<Matcher> = IfOutput<
        (),
        (
            AssertSame<
                DMatMulBroadcasted<BatchMatDims1, BatchMatDims2, Matcher>,
                Dims![(X, U7), (Y, U11), (M, U3), (P, U5)],
            >,
            AssertSame<
                DMatMulBroadcasted<BatchMatDims1, BatchMatDims4, Matcher>,
                Dims![(X, U7), (Y, U11), (M, U3), (P, U5)],
            >,
            AssertSame<
                DMatMulBroadcasted<BatchMatDims3, BatchMatDims2, Matcher>,
                Dims![(X, U7), (Y, U11), (M, U3), (P, U5)],
            >,
            AssertSame<
                DMatMulBroadcasted<BatchMatDims3, BatchMatDims4, Matcher>,
                Dims![(X, U7), (Y, U11), (M, U3), (P, U5)],
            >,
        ),
    >;

    #[test]
    fn dim_matmul_test() {
        let _: Assert1 = ();
        let _: Assert2<_> = ();
        let _: Assert3<_> = ();
        let _: Assert4<_> = ();
        let _: Assert5<_> = ();
        let _: Assert6<_> = ();
        let _: Assert7<_> = ();
        let _: Assert8<_> = ();
        let _: Assert9<_> = ();
    }
}

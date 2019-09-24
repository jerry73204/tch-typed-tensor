use super::{
    BcastAbscent, BroadcastMatcher, DBroadcastBothReversely, DBroadcastBothReverselyOutput,
    DReverse, DReverseOutput, Dim, DimList, MatrixDim,
};

use crate::{DimListType, DimListTypeWithTail};
use typenum::Unsigned;

// two-dimensional matrix multiplication

pub trait DMatMul<Rhs>
where
    Self: MatrixDim,
    Rhs: MatrixDim,
    Self::Output: MatrixDim,
{
    type Output;
}

pub type DMatMulOutput<Lhs, Rhs> = <Lhs as DMatMul<Rhs>>::Output;

// (m x n) . (n x p) -> (m x p)
impl<MDim, MSize, NDim, NSize, PDim, PSize> DMatMul<DimListType! {(NDim, NSize), (PDim, PSize)}> for DimListType! {(MDim, MSize), (NDim, NSize)}
where
    MDim: Dim,
    MSize: Unsigned,
    NDim: Dim,
    NSize: Unsigned,
    PDim: Dim,
    PSize: Unsigned,
{
    type Output = DimListType! {(MDim, MSize), (PDim, PSize)};
}

// broadcasted matmul

pub trait DMatMulBroadcasted<Rhs, Matcher>
where
    Rhs: DimList,
    Matcher: BroadcastMatcher,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

pub type DMatMulBroadcastedOutput<Lhs, Rhs, Matcher> =
    <Lhs as DMatMulBroadcasted<Rhs, Matcher>>::Output;

impl<Lhs, Rhs, Matcher> DMatMulBroadcasted<Rhs, Matcher> for Lhs
where
    Lhs: DimList + DReverse,
    Rhs: DimList + DReverse,
    Matcher: BroadcastMatcher,
    DReverseOutput<Lhs>: DMatMulAuxiliary<DReverseOutput<Rhs>, Matcher>,
    DMatMulAuxiliaryOutput<DReverseOutput<Lhs>, DReverseOutput<Rhs>, Matcher>: DReverse,
{
    type Output =
        DReverseOutput<DMatMulAuxiliaryOutput<DReverseOutput<Lhs>, DReverseOutput<Rhs>, Matcher>>;
}

// auxiliary trait for DMatMulBroadcastable broadcastable matrix multiplication

pub trait DMatMulAuxiliary<Rhs, Matcher>
where
    Rhs: DimList,
    Matcher: BroadcastMatcher,
    Self: DimList,
    Self::Output: DimList,
{
    type Output;
}

pub type DMatMulAuxiliaryOutput<Lhs, Rhs, Matcher> =
    <Lhs as DMatMulAuxiliary<Rhs, Matcher>>::Output;

// vector x vector
impl<LDim, RDim, Size> DMatMulAuxiliary<DimListType! {(RDim, Size)}, BcastAbscent> for DimListType! {(LDim, Size)}
where
    LDim: Dim,
    RDim: Dim,
    Size: Unsigned,
{
    type Output = DimListType! {};
}

// broadcasted matrix x matrix
impl<MDim, MSize, NDim, NSize, PDim, PSize, LTail, RTail, Matcher>
    DMatMulAuxiliary<DimListTypeWithTail! {(PDim, PSize), (NDim, NSize), RTail}, Matcher> for DimListTypeWithTail! {(NDim, NSize), (MDim, MSize), LTail}
where
    MDim: Dim,
    MSize: Unsigned,
    NDim: Dim,
    NSize: Unsigned,
    PDim: Dim,
    PSize: Unsigned,
    LTail: DimList + DBroadcastBothReversely<RTail, Matcher>,
    RTail: DimList,
    Matcher: BroadcastMatcher,
{
    type Output = DimListTypeWithTail! {(PDim, PSize), (MDim, MSize), DBroadcastBothReverselyOutput<LTail, RTail, Matcher>};
}

// broadcasted vector x matrix
impl<NDim, NSize, PDim, PSize, Tail>
    DMatMulAuxiliary<DimListTypeWithTail! {(PDim, PSize), (NDim, NSize), Tail}, BcastAbscent> for DimListType! {(NDim, NSize)}
where
    NDim: Dim,
    NSize: Unsigned,
    PDim: Dim,
    PSize: Unsigned,
    Tail: DimList,
{
    type Output = DimListTypeWithTail! {(PDim, PSize), Tail};
}

// broadcasted matrix x vector
impl<MDim, MSize, NDim, NSize, Tail> DMatMulAuxiliary<DimListType! {(NDim, NSize)}, BcastAbscent> for DimListTypeWithTail! {(NDim, NSize), (MDim, MSize), Tail}
where
    MDim: Dim,
    MSize: Unsigned,
    NDim: Dim,
    NSize: Unsigned,
    Tail: DimList,
{
    type Output = DimListTypeWithTail! {(MDim, MSize), Tail};
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{make_dims, DimListType};
    use type_freak::control::IfSameOutput;
    use typenum::consts::*;

    make_dims! {M, N, P, X, Y}

    type VecDims1 = DimListType! {(N, U2)};
    type VecDims2 = DimListType! {(M, U2)};
    type MatDims1 = DimListType! {(M, U3), (N, U2)};
    type MatDims2 = DimListType! {(N, U2), (P, U5)};
    type BatchMatDims1 = DimListType! {(X, U7), (Y, U1), (M, U3), (N, U2)};
    type BatchMatDims2 = DimListType! {(Y, U11), (N, U2), (P, U5)};

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    // non-broadcasted matmul
    type Assert1 = AssertSame<DMatMulOutput<MatDims1, MatDims2>, DimListType! {(M, U3), (P, U5)}>;

    // vector x vector
    type Assert2<Matcher> =
        AssertSame<DMatMulBroadcastedOutput<VecDims1, VecDims2, Matcher>, DimListType! {}>;

    // matrix x vector
    type Assert3<Matcher> =
        AssertSame<DMatMulBroadcastedOutput<MatDims1, VecDims1, Matcher>, DimListType! {(M, U3)}>;

    // vector x matrix
    type Assert4<Matcher> =
        AssertSame<DMatMulBroadcastedOutput<VecDims1, MatDims2, Matcher>, DimListType! {(P, U5)}>;

    // matrix x matrix
    type Assert5<Matcher> = AssertSame<
        DMatMulBroadcastedOutput<MatDims1, MatDims2, Matcher>,
        DimListType! {(M, U3), (P, U5)},
    >;

    // batched matrix x vector
    type Assert6<Matcher> = AssertSame<
        DMatMulBroadcastedOutput<BatchMatDims1, VecDims1, Matcher>,
        DimListType! {(X, U7), (Y, U1), (M, U3)},
    >;

    // vector x batched matrix
    type Assert7<Matcher> = AssertSame<
        DMatMulBroadcastedOutput<VecDims1, BatchMatDims2, Matcher>,
        DimListType! {(Y, U11), (P, U5)},
    >;

    // batched matrix x matrix
    type Assert8<Matcher> = AssertSame<
        DMatMulBroadcastedOutput<BatchMatDims1, MatDims2, Matcher>,
        DimListType! {(X, U7), (Y, U1), (M, U3), (P, U5)},
    >;

    // batched matrix x batched matrix
    type Assert9<Matcher> = AssertSame<
        DMatMulBroadcastedOutput<BatchMatDims1, BatchMatDims2, Matcher>,
        DimListType! {(X, U7), (Y, U11), (M, U3), (P, U5)},
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

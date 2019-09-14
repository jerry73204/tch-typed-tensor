extern crate typed_tensor_demo;

use typed_tensor_demo::{
    device::{Cpu, Cuda},
    dim::{BroadcastFromTail, DimListEqual, IndexOf, Permute, RemoveMany, SetEqual, SizeAt},
    kind::{Double, Float, Uint8},
    list::TList,
    make_dims,
    tensor::{DoKeepDim, NamedTensor, NoKeepDim, ReduceMean, ReduceSum},
    DimListType, TListType,
};
use typenum::{consts::*, Unsigned};

make_dims! {Batch, Channel, Height, Width}

type ScalarDim = DimListType!();
type VectorDim = DimListType!((Batch, U32));
type MatrixDim = DimListType!((Height, U300), (Width, U400));
type TensorDim = DimListType!((Batch, U32), (Channel, U3), (Height, U300), (Width, U400));
type TensorDim2 = DimListType!((Batch, U32), (Channel, U1), (Height, U300), (Width, U400));

type TensorBHWCDim<I> =
    <TensorDim as Permute<I, TListType!(Batch, Height, Width, Channel)>>::Output;

type XDim<I> = <TensorDim as RemoveMany<I, TListType!(Channel)>>::Output;
type YDim = DimListType!((Batch, U32), (Height, U300), (Width, U400));
type Assert<I> = <XDim<I> as DimListEqual<YDim>>::Output;

type BroadcastExample<M> = <MatrixDim as BroadcastFromTail<M, TensorDim>>::Output;
type SetEqualExample<I1, I2> = <TensorDim as SetEqual<I1, TensorBHWCDim<I2>>>::Output;

type ScalarTensor = NamedTensor<ScalarDim, Uint8, Cpu>;
type VectorTensor = NamedTensor<VectorDim, Float, Cuda<U0>>;
type MatrixTensor = NamedTensor<MatrixDim, Double, Cpu>;
type BCHWTensor = NamedTensor<TensorDim, Double, Cuda<U0>>;
type BCHWTensor2 = NamedTensor<TensorDim2, Double, Cuda<U0>>;
type BHWCTensor<I> = NamedTensor<TensorBHWCDim<I>, Double, Cuda<U0>>;

#[test]
fn test() {
    let _: Assert<_> = ();

    // Lengths
    assert_eq!(ScalarDim::LENGTH, 0);
    assert_eq!(VectorDim::LENGTH, 1);
    assert_eq!(MatrixDim::LENGTH, 2);
    assert_eq!(TensorDim::LENGTH, 4);

    // Dimension broadcast
    assert_eq!(<BroadcastExample<_> as IndexOf<_, Width>>::INDEX, 3);
    assert_eq!(
        <BroadcastExample<_> as SizeAt<_, Height>>::Output::USIZE,
        300
    );

    // Compare set of dimensions
    // Statically assert both sets are equal
    let _: SetEqualExample<_, _> = ();

    // Tensor declarations
    let scalar_tensor = ScalarTensor::zeros();
    let vector_tensor = VectorTensor::zeros();
    let matrix_tensor = MatrixTensor::zeros();
    let bchw_tensor = BCHWTensor::zeros();

    // Statically compute transpose index
    let _: BHWCTensor<_> = bchw_tensor.transpose::<TListType!(Batch, Height, Width, Channel), _>();

    // reduction
    let _: NamedTensor<
        DimListType! {(Batch, U32), (Channel, U3), (Height, U1), (Width, U1)},
        _,
        _,
    > = bchw_tensor.mean::<_, TListType! {Height, Width}, DoKeepDim, Double>();

    let _: NamedTensor<DimListType! {(Batch, U32), (Channel, U3)}, _, _> =
        bchw_tensor.mean::<_, TListType! {Width, Height}, NoKeepDim, Double>();

    // concat dim
    {
        let a = BCHWTensor::zeros();
        let b = BCHWTensor2::zeros();
        let _: NamedTensor<DimListType! {(_, _), (Channel, U4), (_, _), (_, _)}, _, _> =
            a.concat(&b);
    }
}

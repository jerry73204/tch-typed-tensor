extern crate typed_tensor_demo;

use typed_tensor_demo::{
    device::{Cpu, Cuda},
    dim::{BroadcastFromTail, IndexOf, Reorder, SetEqual, SizeAt},
    kind::{Double, Float, Uint8},
    list::TList,
    make_dims,
    tensor::NamedTensor,
    DimListType, TListType,
};
use typenum::{Unsigned, U0, U1, U3, U300, U32, U400};

make_dims! {Batch, Channel, Height, Width}

type ScalarDim = DimListType!();
type VectorDim = DimListType!((Batch, U32));
type MatrixDim = DimListType!((Height, U1), (Width, U1));
type TensorDim = DimListType!((Batch, U32), (Channel, U3), (Height, U300), (Width, U400));

type TensorBHWCDim<I> =
    <TensorDim as Reorder<I, TListType!(Batch, Height, Width, Channel)>>::Output;

type BroadcastExample<M> = <MatrixDim as BroadcastFromTail<M, TensorDim>>::Output;
type SetEqualExample<I1, I2> = <TensorDim as SetEqual<I1, TensorBHWCDim<I2>>>::Output;

type ScalarTensor = NamedTensor<ScalarDim, Uint8, Cpu>;
type VectorTensor = NamedTensor<VectorDim, Float, Cuda<U0>>;
type BCHWTensor = NamedTensor<TensorDim, Double, Cuda<U0>>;
type BHWCTensor<I> = NamedTensor<TensorBHWCDim<I>, Double, Cuda<U0>>;

#[test]
fn test() {
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

    // Example of tensor declarations
    let scalar_tensor = ScalarTensor::zeros();
    let vector_tensor = VectorTensor::zeros();
    let bchw_tensor = BCHWTensor::zeros();

    // Statically compute transpose index
    let bhwc_tensor: BHWCTensor<_> =
        bchw_tensor.transpose::<_, TListType!(Batch, Height, Width, Channel)>();
}

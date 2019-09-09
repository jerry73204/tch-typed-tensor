extern crate typed_tensor_demo;

use typenum::{U0, U3, U300, U32};
use typed_tensor_demo::{
    device::{Cpu, Cuda},
    kind::{Double, Float, Uint8},
    make_dim,
    dim_list,
    tensor::NamedTensor,
};

make_dim! {Batch, Channel, Height, Width}

type ScalarDimension = dim_list!();
type VectorDimension = dim_list!(Batch<U32>);
type MultiDimension = dim_list!(Batch<U32>, Channel<U3>, Height<U300>, Width<U300>);

type ScalarTensor = NamedTensor<ScalarDimension, Uint8, Cpu>;
type VectorTensor = NamedTensor<VectorDimension, Float, Cuda<U0>>;
type MultiDimTensor = NamedTensor<MultiDimension, Double, Cuda<U0>>;

#[test]
fn test() {
    let scalar_tensor = ScalarTensor::zeros();
    let vector_tensor = VectorTensor::zeros();
    let multi_dim_tensor = MultiDimTensor::zeros();
}

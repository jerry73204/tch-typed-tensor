# tch-typed-tensor

Inspired by [Tensor Considered Harmful](http://nlp.seas.harvard.edu/NamedTensor),
this project builds tensor type with named dimensions, featuring automatic dimension
inference and compile-time bound checking. The development is based on top of
[tch-rs](https://github.com/LaurentMazare/tch-rs), a Rust binding for PyTorch, and
[typenum](https://github.com/paholg/typenum) for compile-time numbers.

The project is still in alpha stage, and is not intended for production. Contributions
are welcome!

## Usage

There's no schedule to publish on crates.io. Put git link to your `Cargo.toml` instead.

```
tch-typed-tensor = { git = "https://github.com/jerry73204/tch-typed-tensor.git", branch = "master" }
```

The project depends on [tch-rs](https://github.com/LaurentMazare/tch-rs).
It requires extra environment setup to make `cargo build` work.
Please study [README](https://github.com/LaurentMazare/tch-rs) in tch-rs for details.


## Contribute

The project makes heavy use of trait constructions. It's suggested to take a look at
[Rust HLists (Heterogenous List)](https://beachape.com/blog/2016/10/23/rust-hlists-heterogenously-typed-list/)
and [frunk](https://github.com/lloydmeta/frunk) project before getting started.
Also, it's better to be familar with [PyTorch API](https://pytorch.org/docs/stable/index.html),
and sometimes you would visit [tch-rs reference](https://docs.rs/tch/).

## Examples

### Typed defined tensor and its properties

The tensor type design moves most properties into types, including
dimensions, data type and device. It ensures tensor operations are
type checked, and empowers by automatic type inference.

Dimensions are named types defined by `make_dims!` macro,
but not integer ordinals.

```rust
use tch_typed_tensor::{
    DimListType,
    tensor::NameTensor,
    kind::Double,
    device::Cpu,
};
use typenum::consts::*;

// make_dims! macro defines a list of dimension names
make_dims! {Batch, Channel, Height, Width}

fn main() {
    // Creates a double typed tensor with shape [32, 3, 480, 640]
    let tensor = NamedTensor<
        DimListType! {(Batch, U32), (Channel, U3), (Height, U480), (Width, U640)}, // dimensions
        Float,                                                                     // data type
        Cpu                                                                        // device
    >::zeros();

    let double_tensor: NamedTensor<_, Double, _> = cpu_tensor.to_kind::<Double>();
    let cuda_tensor: NamedTensor<_, _, Cuda<U0>> = cpu_tensor.to_device::<Cuda<U0>>();
}
```

### Compile-time bound check

The type design keeps bound checking in mind. For example, it verifies whether
`select()` index is bound by dimension in compile-time. Otherwise it triggers
compile error.

```rust
let tensor = NamedTensor<
    DimListType! {(Batch, U32), (Channel, U3), (Height, U480), (Width, U640)},
    Double,
    Cpu
>::zeros();

// The return type is automatically inferenced
let sub1: NamedTensor<
    DimListType! {(Batch, _), (Height, _), (Width, _)},
    _,
    _
> = tensor.select::<U1, Channel, _>();

// This is more compact syntax
let sub2 = tensor.select::<U1, Channel, _>();

// It triggers compile error because U3 exceeds Channel dimension
// let sub3 = tensor.select::<U3, Channel, _>();  // compile error!
```

### Safe dimension manipulation

Dimensions are automatically inferred in any tensor operation.
There's no need to explicitly specify returned dimensions.
It can be omitted, or partially specified like
`DimListType! {(Batch, _), (Height, _), (Width, _), (Channel, _)}`
to work as static assertion.

```rust
let bchw_tensor = NamedTensor<
    DimListType! {(Batch, U32), (Channel, U3), (Height, U480), (Width, U640)},
    Double,
    Cpu
>::zeros();

// Change order of dimensions
let bhwc_tensor1: NamedTensor<
    DimListType! {(Batch, _), (Height, _), (Width, _), (Channel, _)},
    _,
    _
> = bchw_tensor.transpose::<TListType! {Batch, Height, Width, Channel}, _>();

// Or use more compact syntax instead
let bhwc_tensor2 = bchw_tensor.transpose::<TListType! {Batch, Height, Width, Channel}, _>();

// Compile error if you miss a dimension here.
// let _ = bchw_tensor.transpose::<TListType! {Batch, Height, Width}, _>();  // compile error!

// Dimension inference also works for reduction operations
let sum_tensor: NamedTensor<
    DimListType! {(Batch, U32), (Channel, U3)},
    _,
    _
> = bhwc_tensor1.sum::<NoKeepDim, Double, TListType! {Width, Height}, _>();
```

## License

Apache 2.0

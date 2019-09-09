use frunk::hlist::{HCons, HList, HNil};
use std::marker::PhantomData;

#[macro_export]
macro_rules! make_dim {
    ( $($name:ident),+ ) => {
        $(
            struct $name<Size: crate::typed_tensor_demo::typenum::marker_traits::Unsigned> {
                phantom: std::marker::PhantomData<Size>,
            }

            impl<Size: crate::typed_tensor_demo::typenum::marker_traits::Unsigned> crate::typed_tensor_demo::dim::Dim
                for $name<Size>
            {
                const SIZE: usize = Size::USIZE;
            }
        )*
    };
}

#[macro_export]
macro_rules! dim_list {
    () => { crate::typed_tensor_demo::dim::DNil };
    ($name:ty) => { crate::typed_tensor_demo::dim::DCons<$name, crate::typed_tensor_demo::dim::DNil> };
    ($name:ty, $($names:ty),+) => { crate::typed_tensor_demo::dim::DCons<$name, crate::typed_tensor_demo::dim_list!($($names),*)> };
}

pub trait Dim {
    const SIZE: usize;
}

pub trait DimList: Sized + HList {
    fn shape() -> Vec<usize>;
}

pub struct DNil;

impl DimList for DNil {
    fn shape() -> Vec<usize> {
        vec![]
    }
}

impl HList for DNil {
    const LEN: usize = 0;
    fn static_len() -> usize {
        Self::LEN
    }
}

pub struct DCons<H: Dim, T: HList + DimList> {
    pub head: H,
    pub tail: T,
}

impl<H, T> DimList for DCons<H, T>
where
    H: Dim,
    T: HList + DimList,
{
    fn shape() -> Vec<usize> {
        let mut shape = vec![H::SIZE];
        shape.extend(T::shape());
        shape
    }
}

impl<H, T> HList for DCons<H, T>
where
    H: Dim,
    T: HList + DimList,
{
    const LEN: usize = 1 + <T as HList>::LEN;
    fn static_len() -> usize {
        Self::LEN
    }
}

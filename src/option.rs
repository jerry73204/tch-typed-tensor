use std::marker::PhantomData;

pub trait TOption {}

pub struct TSome<E> {
    _phantom: PhantomData<E>,
}

impl<E> TOption for TSome<E> {}

pub struct TNone;

impl TOption for TNone {}

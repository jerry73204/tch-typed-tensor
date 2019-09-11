use crate::list::{Cons, Nil, TList};
use std::marker::PhantomData;

// zipper

pub struct Zipper<L: TList, R: TList> {
    _phantom: PhantomData<(L, R)>,
}

// zipper move right

pub trait MoveRight {
    type Output;
}

impl<L> MoveRight for Zipper<L, Nil>
where
    L: TList,
{
    type Output = Zipper<L, Nil>;
}

impl<L, H, T> MoveRight for Zipper<L, Cons<H, T>>
where
    L: TList,
    T: TList,
{
    type Output = Zipper<Cons<H, L>, T>;
}

// zipper move left

pub trait MoveLeft {
    type Output;
}

impl<L> MoveLeft for Zipper<Nil, L>
where
    L: TList,
{
    type Output = Zipper<Nil, L>;
}

impl<L, H, T> MoveLeft for Zipper<Cons<H, T>, L>
where
    L: TList,
    T: TList,
{
    type Output = Zipper<T, Cons<H, L>>;
}

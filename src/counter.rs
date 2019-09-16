use std::marker::PhantomData;

pub trait Where {
    const COUNT_USIZE: usize;
    const COUNT_I64: i64;
}

pub struct Here;

impl Where for Here {
    const COUNT_USIZE: usize = 0;
    const COUNT_I64: i64 = 0;
}

pub struct There<Index: Where> {
    _phantom: PhantomData<Index>,
}

impl<Index> Where for There<Index>
where
    Index: Where,
{
    const COUNT_USIZE: usize = 1 + Index::COUNT_USIZE;
    const COUNT_I64: i64 = 1 + Index::COUNT_I64;
}

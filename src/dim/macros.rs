#[macro_export]
macro_rules! define_dim_names {
    ( $($name:ident),+ ) => {
        $(
            pub struct $name;

            impl $name {
                #[allow(dead_code)]
                pub fn new() -> $name {
                    $name
                }
            }

            impl $crate::dim::DimName for $name {}
        )*
    };
}

#[macro_export]
macro_rules! Dims {
    [] => { $crate::dim::DNil };
    [($name:ty,)] => { $crate::dim::DCons<$name, $crate::dim::Unknown, $crate::dim::DNil> };
    [($name:ty, $size:ty)] => { $crate::dim::DCons<$name, $crate::dim::Known<$size>, $crate::dim::DNil> };
    [($name:ty,), $(($names:ty, $($sizes:ty)?)),+] => { $crate::dim::DCons<$name, $crate::dim::Unknown, $crate::Dims![$(($names, $($sizes)?)),*]> };
    [($name:ty, $size:ty), $(($names:ty, $($sizes:ty)?)),+] => { $crate::dim::DCons<$name, $crate::dim::Known<$size>, $crate::Dims![$(($names, $($sizes)?)),*]> };
}

#[macro_export]
macro_rules! DimsVerbose {
    [] => { $crate::dim::DNil };
    [($name:ty, $size:ty)] => { $crate::dim::DCons<$name,$size, $crate::dim::DNil> };
    [($name:ty, $size:ty), $(($names:ty, $sizes:ty)),+] => { $crate::dim::DCons<$name, $size, $crate::DimsVerbose![$(($names, $sizes)),*]> };
}

#[macro_export]
macro_rules! DimsWithTail {
    [($name:ty, $size:ty); $tail:ty] => { $crate::dim::DCons<$name, $crate::dim::Known<$size>, $tail> };
    [($name:ty,), $(($names:ty, $($sizes:ty)?)),+; $tail:ty] => { $crate::dim::DCons<$name, $crate::dim::Unknown, $crate::DimsWithTail! [$(($names, $($sizes)?)),*; $tail]> };
    [($name:ty, $size:ty), $(($names:ty, $($sizes:ty)?)),+; $tail:ty] => { $crate::dim::DCons<$name, $crate::dim::Known<$size>, $crate::DimsWithTail! [$(($names, $($sizes)?)),*; $tail]> };
}

#[macro_export]
macro_rules! DimsWithTailVerbose {
    [($name:ty, $size:ty); $tail:ty] => { $crate::dim::DCons<$name, $size, $tail> };
    [($name:ty, $size:ty), $(($names:ty, $sizes:ty)),+; $tail:ty] => { $crate::dim::DCons<$name, $size, $crate::DimsWithTailVerbose! [$(($names, $sizes)),*; $tail]> };
}

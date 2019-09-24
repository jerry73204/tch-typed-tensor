#[macro_export]
macro_rules! make_dims {
    ( $($name:ident),+ ) => {
        $(
            pub struct $name;

            impl $name {
                pub fn new() -> $name {
                    $name
                }
            }

            impl $crate::dim::Dim for $name {}
        )*
    };
}

#[macro_export]
macro_rules! DimListType {
    () => { $crate::dim::DNil };
    (($name:ty, $size:ty)) => { $crate::dim::DCons<$name, $size, $crate::dim::DNil> };
    (($name:ty, $size:ty), $(($names:ty, $sizes:ty)),+) => { $crate::dim::DCons<$name, $size, $crate::DimListType! {$(($names, $sizes)),*}> };
}

#[macro_export]
macro_rules! DimListTypeWithTail {
    (($name:ty, $size:ty), $tail:ty) => { $crate::dim::DCons<$name, $size, $tail> };
    (($name:ty, $size:ty), $(($names:ty, $sizes:ty)),+, $tail:ty) => { $crate::dim::DCons<$name, $size, $crate::DimListTypeWithTail! {$(($names, $sizes)),*, $tail}> };
}

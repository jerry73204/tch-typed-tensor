use half::f16;
use num::Complex;
use tch::Kind;

pub trait TensorKind {
    const KIND: Kind;

    type Type;

    fn kind(&self) -> Kind {
        Self::KIND
    }
}

macro_rules! define_kind {
    ($name:ident, $kind_variant:ident, $type:ty) => {
        pub struct $name;

        impl TensorKind for $name {
            const KIND: Kind = Kind::$kind_variant;
            type Type = $type;
        }
    };
}

define_kind!(Uint8, Uint8, u8);
define_kind!(Int8, Int8, i8);
define_kind!(Int16, Int16, i16);
define_kind!(Int, Int, i32);
define_kind!(Int64, Int64, i64);
define_kind!(Half, Half, f16);
define_kind!(Float, Float, f32);
define_kind!(Double, Double, f64);
define_kind!(ComplexHalf, ComplexHalf, Complex<f16>);
define_kind!(ComplexFloat, ComplexFloat, Complex<f32>);
define_kind!(ComplexDouble, ComplexDouble, Complex<f64>);

// integer kind

pub trait IntegerKind {}

impl IntegerKind for Uint8 {}
impl IntegerKind for Int8 {}
impl IntegerKind for Int16 {}
impl IntegerKind for Int {}
impl IntegerKind for Int64 {}

// floating kind

pub trait FloatKind {}

impl FloatKind for Half {}
impl FloatKind for Float {}
impl FloatKind for Double {}

// comple kind

pub trait ComplexKind {}

impl ComplexKind for ComplexHalf {}
impl ComplexKind for ComplexFloat {}
impl ComplexKind for ComplexDouble {}

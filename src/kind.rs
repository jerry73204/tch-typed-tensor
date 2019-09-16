use tch::Kind;

pub trait TensorKind {
    const KIND: Kind;

    fn kind(&self) -> Kind {
        Self::KIND
    }
}

macro_rules! define_kind {
    ($name:ident, $kind_variant:ident) => {
        pub struct $name;

        impl TensorKind for $name {
            const KIND: Kind = Kind::$kind_variant;
        }
    };
}

define_kind!(Uint8, Uint8);
define_kind!(Int8, Int8);
define_kind!(Int16, Int16);
define_kind!(Int, Int);
define_kind!(Int64, Int64);
define_kind!(Half, Half);
define_kind!(Float, Float);
define_kind!(Double, Double);
define_kind!(ComplexHalf, ComplexHalf);
define_kind!(ComplexFloat, ComplexFloat);
define_kind!(ComplexDouble, ComplexDouble);

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

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

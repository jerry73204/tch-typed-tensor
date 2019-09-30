use typenum::{Bit, Integer, NInt, NonZero, PInt, UInt, UTerm, Unsigned, Z0};

pub trait ToUnsigned<Output> {
    fn to_unsigned() -> Output;
}

pub trait ToSigned<Output> {
    fn to_signed() -> Output;
}

pub trait ToInt<Output> {
    fn to_int() -> Output;
}

// UTerm

impl ToUnsigned<u8> for UTerm {
    fn to_unsigned() -> u8 {
        Self::U8
    }
}

impl ToUnsigned<u16> for UTerm {
    fn to_unsigned() -> u16 {
        Self::U16
    }
}

impl ToUnsigned<u32> for UTerm {
    fn to_unsigned() -> u32 {
        Self::U32
    }
}

impl ToUnsigned<u64> for UTerm {
    fn to_unsigned() -> u64 {
        Self::U64
    }
}

impl ToSigned<i8> for UTerm {
    fn to_signed() -> i8 {
        Self::I8
    }
}

impl ToSigned<i16> for UTerm {
    fn to_signed() -> i16 {
        Self::I16
    }
}

impl ToSigned<i32> for UTerm {
    fn to_signed() -> i32 {
        Self::I32
    }
}

impl ToSigned<i64> for UTerm {
    fn to_signed() -> i64 {
        Self::I64
    }
}

impl ToInt<u8> for UTerm {
    fn to_int() -> u8 {
        Self::U8
    }
}

impl ToInt<u16> for UTerm {
    fn to_int() -> u16 {
        Self::U16
    }
}

impl ToInt<u32> for UTerm {
    fn to_int() -> u32 {
        Self::U32
    }
}

impl ToInt<u64> for UTerm {
    fn to_int() -> u64 {
        Self::U64
    }
}

impl ToInt<i8> for UTerm {
    fn to_int() -> i8 {
        Self::I8
    }
}

impl ToInt<i16> for UTerm {
    fn to_int() -> i16 {
        Self::I16
    }
}

impl ToInt<i32> for UTerm {
    fn to_int() -> i32 {
        Self::I32
    }
}

impl ToInt<i64> for UTerm {
    fn to_int() -> i64 {
        Self::I64
    }
}

// UInt

impl<U: Unsigned, B: Bit> ToUnsigned<u8> for UInt<U, B> {
    fn to_unsigned() -> u8 {
        Self::U8
    }
}

impl<U: Unsigned, B: Bit> ToUnsigned<u16> for UInt<U, B> {
    fn to_unsigned() -> u16 {
        Self::U16
    }
}

impl<U: Unsigned, B: Bit> ToUnsigned<u32> for UInt<U, B> {
    fn to_unsigned() -> u32 {
        Self::U32
    }
}

impl<U: Unsigned, B: Bit> ToUnsigned<u64> for UInt<U, B> {
    fn to_unsigned() -> u64 {
        Self::U64
    }
}

impl<U: Unsigned, B: Bit> ToSigned<i8> for UInt<U, B> {
    fn to_signed() -> i8 {
        Self::I8
    }
}

impl<U: Unsigned, B: Bit> ToSigned<i16> for UInt<U, B> {
    fn to_signed() -> i16 {
        Self::I16
    }
}

impl<U: Unsigned, B: Bit> ToSigned<i32> for UInt<U, B> {
    fn to_signed() -> i32 {
        Self::I32
    }
}

impl<U: Unsigned, B: Bit> ToSigned<i64> for UInt<U, B> {
    fn to_signed() -> i64 {
        Self::I64
    }
}

impl<U: Unsigned, B: Bit> ToInt<u8> for UInt<U, B> {
    fn to_int() -> u8 {
        Self::U8
    }
}

impl<U: Unsigned, B: Bit> ToInt<u16> for UInt<U, B> {
    fn to_int() -> u16 {
        Self::U16
    }
}

impl<U: Unsigned, B: Bit> ToInt<u32> for UInt<U, B> {
    fn to_int() -> u32 {
        Self::U32
    }
}

impl<U: Unsigned, B: Bit> ToInt<u64> for UInt<U, B> {
    fn to_int() -> u64 {
        Self::U64
    }
}

impl<U: Unsigned, B: Bit> ToInt<i8> for UInt<U, B> {
    fn to_int() -> i8 {
        Self::I8
    }
}

impl<U: Unsigned, B: Bit> ToInt<i16> for UInt<U, B> {
    fn to_int() -> i16 {
        Self::I16
    }
}

impl<U: Unsigned, B: Bit> ToInt<i32> for UInt<U, B> {
    fn to_int() -> i32 {
        Self::I32
    }
}

impl<U: Unsigned, B: Bit> ToInt<i64> for UInt<U, B> {
    fn to_int() -> i64 {
        Self::I64
    }
}

// Z0

impl ToSigned<i8> for Z0 {
    fn to_signed() -> i8 {
        Self::I8
    }
}

impl ToSigned<i16> for Z0 {
    fn to_signed() -> i16 {
        Self::I16
    }
}

impl ToSigned<i32> for Z0 {
    fn to_signed() -> i32 {
        Self::I32
    }
}

impl ToSigned<i64> for Z0 {
    fn to_signed() -> i64 {
        Self::I64
    }
}

// PInt

impl<U: Unsigned + NonZero> ToSigned<i8> for PInt<U> {
    fn to_signed() -> i8 {
        Self::I8
    }
}

impl<U: Unsigned + NonZero> ToSigned<i16> for PInt<U> {
    fn to_signed() -> i16 {
        Self::I16
    }
}

impl<U: Unsigned + NonZero> ToSigned<i32> for PInt<U> {
    fn to_signed() -> i32 {
        Self::I32
    }
}

impl<U: Unsigned + NonZero> ToSigned<i64> for PInt<U> {
    fn to_signed() -> i64 {
        Self::I64
    }
}

// NInt

impl<U: Unsigned + NonZero> ToSigned<i8> for NInt<U> {
    fn to_signed() -> i8 {
        Self::I8
    }
}

impl<U: Unsigned + NonZero> ToSigned<i16> for NInt<U> {
    fn to_signed() -> i16 {
        Self::I16
    }
}

impl<U: Unsigned + NonZero> ToSigned<i32> for NInt<U> {
    fn to_signed() -> i32 {
        Self::I32
    }
}

impl<U: Unsigned + NonZero> ToSigned<i64> for NInt<U> {
    fn to_signed() -> i64 {
        Self::I64
    }
}

#[cfg(test)]
mod tests {
    use super::{ToSigned, ToUnsigned};
    use typenum::consts::*;

    #[test]
    fn to_concrete_test() {
        let x: u8 = U1::to_unsigned();
        assert_eq!(x, 1);

        let x: i16 = U3::to_signed();
        assert_eq!(x, 3);

        let x: i32 = P5::to_signed();
        assert_eq!(x, 5);

        let x: i32 = N7::to_signed();
        assert_eq!(x, -7);

        let x: i64 = Z0::to_signed();
        assert_eq!(x, 0);
    }
}

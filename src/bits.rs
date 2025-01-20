//! Helpers for extracting ranges of bits from arrays of bytes.

pub(crate) trait GetBits<T>: Sized {
    /// Extract the bit at index `I` from `self`.
    fn bit<const I: usize>(self) -> bool;
    /// Extract `N` bits beginning at index `I` from `self`.
    fn bits<const I: usize, const N: usize>(self) -> T;
}

impl GetBits<u8> for u8 {
    fn bit<const I: usize>(self) -> bool {
        self.bits::<1, I>() != 0
    }

    fn bits<const I: usize, const N: usize>(self) -> u8 {
        (self >> I) & !(u8::MAX << N)
    }
}

impl GetBits<u16> for u16 {
    fn bit<const I: usize>(self) -> bool {
        self.bits::<1, I>() != 0
    }

    fn bits<const I: usize, const N: usize>(self) -> u16 {
        (self >> I) & !(u16::MAX << N)
    }
}

impl GetBits<u8> for [u8; 1] {
    fn bit<const I: usize>(self) -> bool {
        self.bits::<1, I>() != 0
    }

    fn bits<const I: usize, const N: usize>(self) -> u8 {
        (self[0] >> I) & !(u8::MAX << N)
    }
}

impl GetBits<u16> for [u8; 2] {
    fn bit<const I: usize>(self) -> bool {
        self.bits::<1, I>() != 0
    }

    fn bits<const I: usize, const N: usize>(self) -> u16 {
        let raw = u16::from_le_bytes(self);
        (raw >> I) & !(u16::MAX << N)
    }
}

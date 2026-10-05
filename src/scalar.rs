use core::ops::{Add, Div, Mul, Neg, Sub};

/// Skalartyp (Koeffizienten): Ring-Operationen ohne Negation.
pub trait Scalar:
    Copy + PartialEq + core::fmt::Debug + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;

    /// Multiplikatives Inverses, falls es im Typ existiert.
    ///
    /// Floats: `1/x` fuer `x != 0`. Ganzzahlen: nur `1` (die Einheit).
    fn checked_recip(self) -> Option<Self>;
}

/// Skalar mit Negation (Voraussetzung ab `Complex`).
pub trait Signed: Scalar + Neg<Output = Self> {}
impl<T: Scalar + Neg<Output = T>> Signed for T {}

/// Koerper-artiger Skalar mit `/` (Voraussetzung fuer `Div` ab `Complex`).
pub trait Field: Signed + Div<Output = Self> {}

macro_rules! impl_int {
    ($($t:ty),*) => {$(
        impl Scalar for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            #[inline]
            fn checked_recip(self) -> Option<Self> {
                if self == 1 { Some(1) } else { None }
            }
        }
    )*};
}
impl_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_float {
    ($($t:ty),*) => {$(
        impl Scalar for $t {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
            #[inline]
            fn checked_recip(self) -> Option<Self> {
                if self == 0.0 { None } else { Some(1.0 / self) }
            }
        }
        impl Field for $t {}
    )*};
}
impl_float!(f32, f64);

#[cfg(feature = "f16")]
impl_float!(f16);
#[cfg(feature = "f128")]
impl_float!(f128);

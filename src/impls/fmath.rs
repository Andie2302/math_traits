//! Gleitkomma-Elementarfunktionen: aus `std`, sonst aus `libm`.
//!
//! Ohne beide Features gibt es für `f32`/`f64` keine Elementarfunktionen und
//! damit auch kein [`RealField`](crate::structures::RealField). Alle Strukturen
//! darunter (z. B. [`OrderedField`](crate::structures::OrderedField)) bleiben
//! verfügbar.

pub(crate) trait FMath: Copy {
    fn m_sqrt(self) -> Self;
    fn m_exp(self) -> Self;
    fn m_ln(self) -> Self;
    fn m_sin(self) -> Self;
    fn m_cos(self) -> Self;
    fn m_atan2(self, x: Self) -> Self;
    fn m_hypot(self, y: Self) -> Self;
}

#[cfg(feature = "std")]
macro_rules! fmath {
    ($($t:ty: $($_libm:ident),*);*) => {$(
        impl FMath for $t {
            fn m_sqrt(self) -> Self { <$t>::sqrt(self) }
            fn m_exp(self) -> Self { <$t>::exp(self) }
            fn m_ln(self) -> Self { <$t>::ln(self) }
            fn m_sin(self) -> Self { <$t>::sin(self) }
            fn m_cos(self) -> Self { <$t>::cos(self) }
            fn m_atan2(self, x: Self) -> Self { <$t>::atan2(self, x) }
            fn m_hypot(self, y: Self) -> Self { <$t>::hypot(self, y) }
        }
    )*};
}

#[cfg(all(feature = "libm", not(feature = "std")))]
macro_rules! fmath {
    ($($t:ty: $sqrt:ident, $exp:ident, $ln:ident, $sin:ident, $cos:ident, $atan2:ident, $hypot:ident);*) => {$(
        impl FMath for $t {
            fn m_sqrt(self) -> Self { libm::$sqrt(self) }
            fn m_exp(self) -> Self { libm::$exp(self) }
            fn m_ln(self) -> Self { libm::$ln(self) }
            fn m_sin(self) -> Self { libm::$sin(self) }
            fn m_cos(self) -> Self { libm::$cos(self) }
            fn m_atan2(self, x: Self) -> Self { libm::$atan2(self, x) }
            fn m_hypot(self, y: Self) -> Self { libm::$hypot(self, y) }
        }
    )*};
}

fmath!(
    f64: sqrt, exp, log, sin, cos, atan2, hypot;
    f32: sqrtf, expf, logf, sinf, cosf, atan2f, hypotf
);

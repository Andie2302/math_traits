use crate::scalar::Scalar;
use crate::traits::{CayleyDickson, TrivialZero};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Level 0: die reellen Zahlen ueber dem Skalartyp `T`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Real<T>(pub T);

impl<T> From<T> for Real<T> {
    fn from(t: T) -> Self {
        Real(t)
    }
}

impl<T: Scalar> Add for Real<T> {
    type Output = Self;
    #[inline]
    fn add(self, o: Self) -> Self {
        Real(self.0 + o.0)
    }
}
impl<T: Scalar> Sub for Real<T> {
    type Output = Self;
    #[inline]
    fn sub(self, o: Self) -> Self {
        Real(self.0 - o.0)
    }
}
impl<T: Scalar> Mul for Real<T> {
    type Output = Self;
    #[inline]
    fn mul(self, o: Self) -> Self {
        Real(self.0 * o.0)
    }
}
/// Bei Ganzzahlen ganzzahlige Division (panikt bei Division durch 0).
impl<T: Scalar + Div<Output = T>> Div for Real<T> {
    type Output = Self;
    #[inline]
    fn div(self, o: Self) -> Self {
        Real(self.0 / o.0)
    }
}
impl<T: Scalar + Neg<Output = T>> Neg for Real<T> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Real(-self.0)
    }
}

impl<T: Scalar> AddAssign for Real<T> {
    fn add_assign(&mut self, o: Self) {
        *self = *self + o;
    }
}
impl<T: Scalar> SubAssign for Real<T> {
    fn sub_assign(&mut self, o: Self) {
        *self = *self - o;
    }
}
impl<T: Scalar> MulAssign for Real<T> {
    fn mul_assign(&mut self, o: Self) {
        *self = *self * o;
    }
}
impl<T: Scalar + Div<Output = T>> DivAssign for Real<T> {
    fn div_assign(&mut self, o: Self) {
        *self = *self / o;
    }
}

impl<T: Scalar> TrivialZero for Real<T> {
    #[inline]
    fn zero() -> Self {
        Real(T::ZERO)
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.0 == T::ZERO
    }
}

impl<T: Scalar> CayleyDickson for Real<T> {
    type Scalar = T;
    const LEVEL: usize = 0;
    const DIM: usize = 1;

    #[inline]
    fn one() -> Self {
        Real(T::ONE)
    }
    #[inline]
    fn from_scalar(s: T) -> Self {
        Real(s)
    }
    #[inline]
    fn conjugate(self) -> Self {
        self
    }
    #[inline]
    fn real(self) -> T {
        self.0
    }
    #[inline]
    fn norm_sqr(self) -> T {
        self.0 * self.0
    }
    #[inline]
    fn coeff(&self, i: usize) -> T {
        assert!(i < 1, "Koeffizientenindex ausserhalb von DIM");
        self.0
    }
    #[inline]
    fn from_fn(mut f: impl FnMut(usize) -> T) -> Self {
        Real(f(0))
    }
    #[inline]
    fn scale(self, s: T) -> Self {
        Real(self.0 * s)
    }
}

use crate::scalar::Scalar;
use crate::traits::{Algebra, CayleyDickson, TrivialZero};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Cayley-Dickson-Verdopplung: das Paar `(a, b)` aus zwei Elementen der Vorstufe.
///
/// Meist benutzt man die Aliase `Complex<T>` .. `Trigintaduonion<T>`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cd<A> {
    /// Untere Haelfte (Koeffizienten `0..DIM/2`).
    pub a: A,
    /// Obere Haelfte (Koeffizienten `DIM/2..DIM`).
    pub b: A,
}

impl<A> Cd<A> {
    #[inline]
    pub const fn new(a: A, b: A) -> Self {
        Cd { a, b }
    }
}

impl<A: Add<Output = A>> Add for Cd<A> {
    type Output = Self;
    #[inline]
    fn add(self, o: Self) -> Self {
        Cd::new(self.a + o.a, self.b + o.b)
    }
}
impl<A: Sub<Output = A>> Sub for Cd<A> {
    type Output = Self;
    #[inline]
    fn sub(self, o: Self) -> Self {
        Cd::new(self.a - o.a, self.b - o.b)
    }
}
impl<A: Neg<Output = A>> Neg for Cd<A> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Cd::new(-self.a, -self.b)
    }
}

/// `(a, b) * (c, d) = (a*c - conj(d)*b, d*a + b*conj(c))`
impl<A: CayleyDickson + Neg<Output = A>> Mul for Cd<A> {
    type Output = Self;
    #[inline]
    fn mul(self, o: Self) -> Self {
        let (a, b, c, d) = (self.a, self.b, o.a, o.b);
        Cd::new(a * c - d.conjugate() * b, d * a + b * c.conjugate())
    }
}

impl<A: Add<Output = A> + Copy> AddAssign for Cd<A> {
    fn add_assign(&mut self, o: Self) {
        *self = *self + o;
    }
}
impl<A: Sub<Output = A> + Copy> SubAssign for Cd<A> {
    fn sub_assign(&mut self, o: Self) {
        *self = *self - o;
    }
}
impl<A: CayleyDickson + Neg<Output = A>> MulAssign for Cd<A> {
    fn mul_assign(&mut self, o: Self) {
        *self = *self * o;
    }
}

impl<A: TrivialZero> TrivialZero for Cd<A> {
    #[inline]
    fn zero() -> Self {
        Cd::new(A::zero(), A::zero())
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.a.is_zero() && self.b.is_zero()
    }
}

impl<A: CayleyDickson + Neg<Output = A>> Algebra for Cd<A> {
    type Scalar = A::Scalar;
    const DIM: usize = 2 * A::DIM;
    const COMMUTATIVE: bool = <Self as CayleyDickson>::LEVEL <= 1;
    const ASSOCIATIVE: bool = <Self as CayleyDickson>::LEVEL <= 2;
    const ALTERNATIVE: bool = <Self as CayleyDickson>::LEVEL <= 3;
    const FLEXIBLE: bool = true;
    const POWER_ASSOCIATIVE: bool = true;
    const MULTIPLICATIVE_NORM: bool = <Self as CayleyDickson>::LEVEL <= 3;
    const HAS_ZERO_DIVISORS: bool = <Self as CayleyDickson>::LEVEL >= 4;

    #[inline]
    fn one() -> Self {
        Cd::new(A::one(), A::zero())
    }
    #[inline]
    fn from_scalar(s: Self::Scalar) -> Self {
        Cd::new(A::from_scalar(s), A::zero())
    }
    #[inline]
    fn coeff(&self, i: usize) -> Self::Scalar {
        if i < A::DIM {
            self.a.coeff(i)
        } else {
            self.b.coeff(i - A::DIM)
        }
    }
    #[inline]
    fn from_fn(mut f: impl FnMut(usize) -> Self::Scalar) -> Self {
        let a = A::from_fn(&mut f);
        let b = A::from_fn(|i| f(i + A::DIM));
        Cd::new(a, b)
    }
    #[inline]
    fn scale(self, s: Self::Scalar) -> Self {
        Cd::new(self.a.scale(s), self.b.scale(s))
    }
    #[inline]
    fn inverse(self) -> Option<Self> {
        self.norm_sqr()
            .checked_recip()
            .map(|r| self.conjugate().scale(r))
    }
}

impl<A: CayleyDickson + Neg<Output = A>> CayleyDickson for Cd<A> {
    const LEVEL: usize = A::LEVEL + 1;

    #[inline]
    fn conjugate(self) -> Self {
        Cd::new(self.a.conjugate(), -self.b)
    }
    #[inline]
    fn real(self) -> Self::Scalar {
        self.a.real()
    }
    #[inline]
    fn norm_sqr(self) -> Self::Scalar {
        self.a.norm_sqr() + self.b.norm_sqr()
    }
}

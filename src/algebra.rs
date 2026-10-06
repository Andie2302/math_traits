use crate::doubled::Cd;
use crate::real::Real;
use crate::scalar::{Field, Scalar, Signed};
use crate::traits::{
    Algebra, Alternative, Associative, CayleyDickson, Commutative, DivisionAlgebra, Flexible,
    MultiplicativeNorm, NonTrivialZero, PowerAssociative,
};
use core::ops::{Div, DivAssign};

/// Level 1: komplexe Zahlen `(re, im)`.
pub type Complex<T> = Cd<Real<T>>;
/// Level 2: Quaternionen (Dimension 4).
pub type Quaternion<T> = Cd<Complex<T>>;
/// Level 3: Oktonionen (Dimension 8).
pub type Octonion<T> = Cd<Quaternion<T>>;
/// Level 4: Sedenionen (Dimension 16).
pub type Sedenion<T> = Cd<Octonion<T>>;
/// Level 5: Trigintaduonionen (Dimension 32).
pub type Trigintaduonion<T> = Cd<Sedenion<T>>;

// ---- Marker-Traits pro Stufe -------------------------------------------

macro_rules! mark {
    ($tr:ident: $($ty:ident),*) => {$(
        impl<T: Scalar> $tr for $ty<T> where $ty<T>: Algebra {}
    )*};
}
mark!(Commutative: Real, Complex);
mark!(Associative: Real, Complex, Quaternion);
mark!(Alternative: Real, Complex, Quaternion, Octonion);
mark!(MultiplicativeNorm: Real, Complex, Quaternion, Octonion);
mark!(Flexible: Real, Complex, Quaternion, Octonion, Sedenion, Trigintaduonion);
mark!(PowerAssociative: Real, Complex, Quaternion, Octonion, Sedenion, Trigintaduonion);

// ---- Division (nur Float-Skalare, nur Level <= 3) -----------------------

macro_rules! division {
    ($($ty:ident),*) => {$(
        /// `a / b = a * b^-1` (Rechtsdivision, loest `x * b = a`).
        impl<T: Field> Div for $ty<T> {
            type Output = Self;
            #[inline]
            fn div(self, b: Self) -> Self {
                self * b.conjugate().scale(T::ONE / b.norm_sqr())
            }
        }
        impl<T: Field> DivAssign for $ty<T> {
            fn div_assign(&mut self, o: Self) {
                *self = *self / o;
            }
        }
        impl<T: Field> DivisionAlgebra for $ty<T> {}
    )*};
}
division!(Complex, Quaternion, Octonion);

// Real: `Div` kommt aus real.rs (auch fuer Ganzzahlen); als Divisionsalgebra nur ueber Feldern.
impl<T: Field> DivisionAlgebra for Real<T> {}

// ---- Nichttriviale Nullteiler (ab Sedenion) -----------------------------

/// Konkretes Paar per Brute-Force-Suche gefunden, getestet in `tests/axioms.rs`.
impl<T: Signed> NonTrivialZero for Sedenion<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        (
            Self::basis(ZD_A.0) + Self::basis(ZD_A.1),
            Self::basis(ZD_B.0) - Self::basis(ZD_B.1),
        )
    }
}

/// Ein Sedenion-Nullteiler eingebettet in die obere Stufe: `(a,0)*(b,0) = (a*b, 0) = 0`.
impl<T: Signed> NonTrivialZero for Trigintaduonion<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        let (x, y) = Sedenion::<T>::zero_divisor_pair();
        (
            Cd::new(x, crate::TrivialZero::zero()),
            Cd::new(y, crate::TrivialZero::zero()),
        )
    }
}

/// `(e1 + e10) * (e4 - e15) = 0` in dieser Konvention der Verdopplung.
const ZD_A: (usize, usize) = (1, 10);
const ZD_B: (usize, usize) = (4, 15);

// ---- Komfort-Konstruktoren und Array-Konvertierung ----------------------

macro_rules! arrays {
    ($ty:ident, $n:literal) => {
        impl<T: Signed> From<[T; $n]> for $ty<T> {
            fn from(arr: [T; $n]) -> Self {
                <Self as Algebra>::from_fn(|i| arr[i])
            }
        }
        impl<T: Signed> From<$ty<T>> for [T; $n] {
            fn from(x: $ty<T>) -> Self {
                core::array::from_fn(|i| x.coeff(i))
            }
        }
    };
}
arrays!(Complex, 2);
arrays!(Quaternion, 4);
arrays!(Octonion, 8);
arrays!(Sedenion, 16);
arrays!(Trigintaduonion, 32);

impl<T: Signed> Complex<T> {
    /// `re + im*i`
    pub fn from_re_im(re: T, im: T) -> Self {
        [re, im].into()
    }
}
impl<T: Signed> Quaternion<T> {
    /// `w + x*i + y*j + z*k`
    pub fn from_wxyz(w: T, x: T, y: T, z: T) -> Self {
        [w, x, y, z].into()
    }
}

//! `Num<T>`: Formeln mit `+ − * /` schreiben, generisch über die Basis.
//!
//! ```
//! use math_traits::num::{Num, Real};
//! use math_traits::autodiff::Dual;
//!
//! // Eine Formel, einmal geschrieben ...
//! fn f<R: Real>(x: R) -> R {
//!     x * x * 3.0 + x.exp() - 2.0
//! }
//!
//! // ... für Werte und für Ableitungen.
//! assert_eq!(f(Num(0.0)).0, -1.0);
//! let d = f(Num(Dual::variable(0.0))).0;
//! assert_eq!(d.eps, 1.0); // f'(0) = 6·0 + e⁰
//! ```
//!
//! Die Operatoren verlangen nur die passende **Signatur**, nicht die Gesetze.
//! Gesetze verlangt man wie immer über Bounds aus [`crate::structures`].
//!
//! Konstanten wie `3.0` gelangen über die Einbettung `s ↦ s·1` in `T`: Jede
//! Algebra über ℝ hat sie. Dafür braucht `T: ScalarMul<f64>`. Gemischt wird
//! nur mit `f64` (sonst wäre `Num(3.0) + 1.0` mehrdeutig); für andere
//! Skalare gibt es [`Num::lift`]. In generischem Code über [`Real`] stehen
//! Konstanten **rechts** (`x * 2.5`), weil Rust `f64: Mul<R>` nicht als
//! Bound von `R` ausdrücken kann.
//!
//! Partielle Funktionen (`/`, `ln`, `sqrt`) **panicken** außerhalb ihres
//! Definitionsbereichs, wie Ganzzahl-Division durch 0. Wer das abfangen will,
//! nimmt `try_div`, `try_ln`, `try_sqrt`.

use core::cmp::Ordering;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::signature::{
    Additive, BinaryOp, BinaryRelation, HasExp, HasIdentity, HasInverse, HasLn, HasPartialInverse,
    HasSinCos, HasSqrt, LessEq, Multiplicative, ScalarMul, op,
};

/// Ein Element von `T` mit Rechenoperatoren.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
#[repr(transparent)]
pub struct Num<T>(pub T);

impl<T> Num<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: HasIdentity<Additive>> Num<T> {
    pub fn zero() -> Self {
        Num(T::identity())
    }
}

impl<T: HasIdentity<Multiplicative>> Num<T> {
    pub fn one() -> Self {
        Num(T::identity())
    }

    /// Einbettung eines Skalars: `s ↦ s·1`.
    pub fn lift<S>(s: S) -> Self
    where
        T: ScalarMul<S>,
    {
        Num(T::identity().scale(&s))
    }
}

// --- Operatoren zwischen Num<T> ------------------------------------------------------

macro_rules! binop {
    ($tr:ident, $method:ident, $assign_tr:ident, $assign:ident, [$($bound:tt)*], |$a:ident, $b:ident| $body:expr) => {
        impl<T: $($bound)*> $tr for Num<T> {
            type Output = Num<T>;
            fn $method(self, rhs: Num<T>) -> Num<T> {
                let ($a, $b) = (&self.0, &rhs.0);
                Num($body)
            }
        }
        impl<'a, T: $($bound)*> $tr<&'a Num<T>> for &'a Num<T> {
            type Output = Num<T>;
            fn $method(self, rhs: &Num<T>) -> Num<T> {
                let ($a, $b) = (&self.0, &rhs.0);
                Num($body)
            }
        }
        impl<T: $($bound)*> $assign_tr for Num<T> {
            fn $assign(&mut self, rhs: Num<T>) {
                let ($a, $b) = (&self.0, &rhs.0);
                self.0 = $body;
            }
        }
    };
}

binop!(
    Add,
    add,
    AddAssign,
    add_assign,
    [BinaryOp<Additive>],
    |a, b| op::<Additive, _>(a, b)
);
binop!(
    Sub,
    sub,
    SubAssign,
    sub_assign,
    [BinaryOp<Additive> + HasInverse<Additive>],
    |a, b| op::<Additive, _>(a, &b.inverse())
);
binop!(
    Mul,
    mul,
    MulAssign,
    mul_assign,
    [BinaryOp<Multiplicative>],
    |a, b| op::<Multiplicative, _>(a, b)
);

impl<T: HasPartialInverse<Multiplicative>> Div for Num<T> {
    type Output = Num<T>;
    /// Panickt, wenn `rhs` kein Inverses hat.
    fn div(self, rhs: Num<T>) -> Num<T> {
        self.try_div(&rhs)
            .expect("Division: Divisor hat kein Inverses")
    }
}

impl<T: HasInverse<Additive>> Neg for Num<T> {
    type Output = Num<T>;
    fn neg(self) -> Num<T> {
        Num(self.0.inverse())
    }
}

impl<T: BinaryOp<Additive> + HasIdentity<Additive>> Sum for Num<T> {
    fn sum<I: Iterator<Item = Num<T>>>(iter: I) -> Self {
        iter.fold(Num::zero(), |a, b| a + b)
    }
}

impl<T: BinaryOp<Multiplicative> + HasIdentity<Multiplicative>> Product for Num<T> {
    fn product<I: Iterator<Item = Num<T>>>(iter: I) -> Self {
        iter.fold(Num::one(), |a, b| a * b)
    }
}

/// Vergleich über die Ordnungsrelation `≤` von `T`.
impl<T: BinaryRelation<LessEq> + PartialEq> PartialOrd for Num<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.0.relates(&other.0), other.0.relates(&self.0)) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => None,
        }
    }
}

// --- Gemischt mit Gleitkomma-Konstanten ----------------------------------------------

macro_rules! with_float {
    ($($f:ty),*) => {$(
        impl<T: ScalarMul<$f>> Mul<$f> for Num<T> {
            type Output = Num<T>;
            fn mul(self, s: $f) -> Num<T> { Num(self.0.scale(&s)) }
        }
        impl<T: ScalarMul<$f>> Mul<Num<T>> for $f {
            type Output = Num<T>;
            fn mul(self, x: Num<T>) -> Num<T> { Num(x.0.scale(&self)) }
        }
        impl<T: ScalarMul<$f>> Div<$f> for Num<T> {
            type Output = Num<T>;
            fn div(self, s: $f) -> Num<T> { Num(self.0.scale(&(1.0 / s))) }
        }
        impl<T> Add<$f> for Num<T>
        where
            T: BinaryOp<Additive> + HasIdentity<Multiplicative> + ScalarMul<$f>,
        {
            type Output = Num<T>;
            fn add(self, s: $f) -> Num<T> { self + Num::lift(s) }
        }
        impl<T> Add<Num<T>> for $f
        where
            T: BinaryOp<Additive> + HasIdentity<Multiplicative> + ScalarMul<$f>,
        {
            type Output = Num<T>;
            fn add(self, x: Num<T>) -> Num<T> { Num::lift(self) + x }
        }
        impl<T> Sub<$f> for Num<T>
        where
            T: BinaryOp<Additive> + HasInverse<Additive> + HasIdentity<Multiplicative> + ScalarMul<$f>,
        {
            type Output = Num<T>;
            fn sub(self, s: $f) -> Num<T> { self - Num::lift(s) }
        }
        impl<T> Sub<Num<T>> for $f
        where
            T: BinaryOp<Additive> + HasInverse<Additive> + HasIdentity<Multiplicative> + ScalarMul<$f>,
        {
            type Output = Num<T>;
            fn sub(self, x: Num<T>) -> Num<T> { Num::lift(self) - x }
        }
        impl<T> Div<Num<T>> for $f
        where
            T: HasPartialInverse<Multiplicative> + ScalarMul<$f>,
        {
            type Output = Num<T>;
            fn div(self, x: Num<T>) -> Num<T> { Num(x.recip().0.scale(&self)) }
        }
    )*};
}

with_float!(f64);

// --- Funktionen ------------------------------------------------------------------------

impl<T: HasPartialInverse<Multiplicative>> Num<T> {
    pub fn try_div(&self, rhs: &Self) -> Option<Self> {
        rhs.0
            .try_inverse()
            .map(|i| Num(op::<Multiplicative, _>(&self.0, &i)))
    }
    pub fn try_recip(&self) -> Option<Self> {
        self.0.try_inverse().map(Num)
    }
    /// `1/x`, panickt ohne Inverses.
    pub fn recip(&self) -> Self {
        self.try_recip().expect("recip: kein Inverses")
    }
    /// `xⁿ` für ganze `n` (Square-and-Multiply), panickt für `n < 0` ohne Inverses.
    pub fn powi(&self, n: i32) -> Self {
        let mut base = if n < 0 {
            self.recip()
        } else {
            Num(op::<Multiplicative, _>(&self.0, &T::identity()))
        };
        let mut e = n.unsigned_abs();
        let mut acc = Num::one();
        while e > 0 {
            if e & 1 == 1 {
                acc *= Num(op::<Multiplicative, _>(&base.0, &T::identity()));
            }
            e >>= 1;
            if e > 0 {
                base = Num(op::<Multiplicative, _>(&base.0, &base.0));
            }
        }
        acc
    }
}

impl<T: HasExp> Num<T> {
    pub fn exp(&self) -> Self {
        Num(self.0.exp())
    }
}

impl<T: HasLn> Num<T> {
    pub fn try_ln(&self) -> Option<Self> {
        self.0.ln().map(Num)
    }
    /// Panickt außerhalb des Definitionsbereichs.
    pub fn ln(&self) -> Self {
        self.try_ln()
            .expect("ln: außerhalb des Definitionsbereichs")
    }
}

impl<T: HasSqrt> Num<T> {
    pub fn try_sqrt(&self) -> Option<Self> {
        self.0.sqrt().map(Num)
    }
    /// Panickt außerhalb des Definitionsbereichs.
    pub fn sqrt(&self) -> Self {
        self.try_sqrt()
            .expect("sqrt: außerhalb des Definitionsbereichs")
    }
}

impl<T: HasSinCos> Num<T> {
    pub fn sin(&self) -> Self {
        Num(self.0.sin())
    }
    pub fn cos(&self) -> Self {
        Num(self.0.cos())
    }
}

impl<T: HasExp + HasLn + BinaryOp<Multiplicative>> Num<T> {
    /// `xʸ = exp(y·ln x)`, panickt wo `ln x` nicht definiert ist.
    pub fn pow(&self, y: &Self) -> Self {
        Num(op::<Multiplicative, _>(&y.0, &self.ln().0).exp())
    }
}

impl<T: HasExp + HasLn + ScalarMul<f64>> Num<T> {
    /// `xᵉ` mit reellem Exponenten, panickt wo `ln x` nicht definiert ist.
    pub fn powf(&self, e: f64) -> Self {
        Num(self.ln().0.scale(&e).exp())
    }
}

/// Alles, was man für reelle Formeln mit Konstanten braucht: `+ − * /`,
/// `exp`, `ln`, `sqrt`, `sin`, `cos`, `powi`, `powf` und `f64`-Konstanten.
/// `f64`, `Dual<f64>`, `Dual<Dual<f64>>` und `Complex<f64>` erfüllen es.
pub trait Real:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Add<f64, Output = Self>
    + Sub<f64, Output = Self>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
{
    /// Die Konstante `c` als Element (`c·1`).
    fn constant(c: f64) -> Self;
    fn exp(&self) -> Self;
    fn ln(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn powi(&self, n: i32) -> Self;
    fn powf(&self, e: f64) -> Self;
}

impl<T> Real for Num<T>
where
    T: Copy
        + BinaryOp<Additive>
        + HasInverse<Additive>
        + HasPartialInverse<Multiplicative>
        + ScalarMul<f64>
        + HasExp
        + HasLn
        + HasSqrt
        + HasSinCos
        + HasIdentity<Multiplicative>,
{
    fn constant(c: f64) -> Self {
        Num::lift(c)
    }
    fn exp(&self) -> Self {
        Num::exp(self)
    }
    fn ln(&self) -> Self {
        Num::ln(self)
    }
    fn sqrt(&self) -> Self {
        Num::sqrt(self)
    }
    fn sin(&self) -> Self {
        Num::sin(self)
    }
    fn cos(&self) -> Self {
        Num::cos(self)
    }
    fn powi(&self, n: i32) -> Self {
        Num::powi(self, n)
    }
    fn powf(&self, e: f64) -> Self {
        Num::powf(self, e)
    }
}

//! Clifford-Algebren `Cl(p, q, r)` ueber einem vorzeichenbehafteten Skalar `T`.
//!
//! `Clifford<T, P, Q, R, N>` hat `P + Q + R` Generatoren `e_0, e_1, ..`; die ersten `P`
//! quadrieren zu `+1`, die naechsten `Q` zu `-1`, die letzten `R` zu `0` (degeneriert). Die
//! Basis sind alle Produkte verschiedener Generatoren (Teilmengen, als Bitmaske
//! indiziert), also `N = 2^(P+Q+R)` Koeffizienten. Alles liegt auf dem Stack
//! (`no_std`, kein `alloc`).
//!
//! `N` muss `2^(P+Q+R)` sein; das wird zur Compile-Zeit geprueft:
//!
//! ```compile_fail
//! use math_traits::{Algebra, Clifford};
//! // Cl(2,0,0) hat Dimension 4, nicht 5.
//! let _ = Clifford::<f64, 2, 0, 0, 5>::one();
//! ```
//!
//! Haeufige Algebren gibt es als generierte Aliase (siehe `tools/gen_clifford.py`),
//! z.B. [`Cl3`], [`Sta`], [`Pga3`], [`Cga3`]; alle anderen Signaturen schreibt man
//! mit `Clifford<T, P, Q, R, N>` direkt.
//!
//! # Axiome
//!
//! Clifford-Algebren sind immer assoziativ (damit auch alternativ, flexibel und
//! potenzassoziativ), aber nicht Teil der Cayley-Dickson-Reihe: Oktonionen und
//! hoeher sind keine Clifford-Algebren. `Cl(0,1)` entspricht den komplexen Zahlen und
//! `Cl(0,2)` den Quaternionen (siehe die `From`-Konvertierungen). Kommutativ sind nur
//! die Algebren mit hoechstens einem Generator, Divisionsalgebren nur `Cl(0,0)`,
//! `Cl(0,1)` und `Cl(0,2)`; alle anderen Signaturen haben Nullteiler
//! ([`Clifford::try_zero_divisor_pair`]).

use crate::scalar::Signed;
use crate::traits::{
    Algebra, Alternative, Associative, Commutative, Flexible, MultiplicativeNorm, PowerAssociative,
    TrivialZero,
};
use crate::{Complex, Quaternion};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

mod named;
pub use named::*;

/// Multivektor der Clifford-Algebra `Cl(P, Q, R)` mit `N = 2^(P+Q+R)` Koeffizienten.
///
/// Koeffizient `i` gehoert zum Basis-Blade, dessen Generatoren die gesetzten Bits von `i`
/// sind (`0` = Skalar, `1` = `e_0`, `2` = `e_1`, `3` = `e_0 e_1`, ...).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Clifford<T, const P: usize, const Q: usize, const R: usize, const N: usize>([T; N]);

/// Vorzeichen des Produkts der Basis-Blades `a` und `b` (Bitmasken): `1`, `-1` oder `0`.
///
/// Das Vorzeichen setzt sich aus dem Umsortieren der Generatoren in aufsteigende
/// Reihenfolge und aus den Quadraten der gemeinsamen Generatoren zusammen.
pub const fn blade_sign(a: usize, b: usize, p: usize, q: usize, r: usize) -> i8 {
    let mut swaps = 0u32;
    let mut x = a >> 1;
    while x != 0 {
        swaps += (x & b).count_ones();
        x >>= 1;
    }
    let mut negative = swaps & 1 == 1;
    let common = a & b;
    let q_mask = ((1usize << q) - 1) << p;
    let r_mask = ((1usize << r) - 1) << (p + q);
    if common & r_mask != 0 {
        return 0;
    }
    if (common & q_mask).count_ones() & 1 == 1 {
        negative = !negative;
    }
    if negative { -1 } else { 1 }
}

impl<T: Signed, const P: usize, const Q: usize, const R: usize, const N: usize>
    Clifford<T, P, Q, R, N>
{
    const CHECK: () = assert!(
        P + Q + R < usize::BITS as usize && N == 1usize << (P + Q + R),
        "N muss gleich 2^(P+Q+R) sein"
    );

    /// Anzahl der Generatoren `P + Q + R`.
    pub const GENERATORS: usize = P + Q + R;

    /// Baut einen Multivektor aus allen `N` Koeffizienten.
    pub fn from_coeffs(coeffs: [T; N]) -> Self {
        let () = Self::CHECK;
        Clifford(coeffs)
    }

    /// Alle Koeffizienten (Index = Bitmaske des Basis-Blades).
    pub fn coeffs(&self) -> &[T; N] {
        &self.0
    }

    /// Wie [`Clifford::coeffs`], aber als Wert.
    pub fn into_coeffs(self) -> [T; N] {
        self.0
    }

    /// Basis-Blade zur Bitmaske `mask` mit Koeffizient `coef`.
    pub fn blade(mask: usize, coef: T) -> Self {
        assert!(mask < N, "Blade-Maske ausserhalb von N");
        let mut c = [T::ZERO; N];
        c[mask] = coef;
        Self::from_coeffs(c)
    }

    /// Generator `e_i` fuer `i < P + Q + R`.
    pub fn generator(i: usize) -> Self {
        assert!(i < P + Q + R, "Generator-Index ausserhalb von P + Q + R");
        Self::blade(1 << i, T::ONE)
    }

    /// Pseudoskalar `e_0 e_1 .. e_(n-1)` (hoechstes Blade).
    pub fn pseudoscalar() -> Self {
        Self::blade(N - 1, T::ONE)
    }

    /// Skalarteil (Grad 0).
    pub fn scalar_part(self) -> T {
        self.0[0]
    }

    /// Nur die Blades vom Grad `k`.
    pub fn grade_part(self, k: usize) -> Self {
        let mut c = [T::ZERO; N];
        for (i, slot) in c.iter_mut().enumerate() {
            if i.count_ones() as usize == k {
                *slot = self.0[i];
            }
        }
        Clifford(c)
    }

    /// Vektorteil (Grad 1) als Koeffizienten der Generatoren `e_0 .. e_(K-1)`; `K = P + Q + R`.
    pub fn vector_part<const K: usize>(self) -> [T; K] {
        assert!(K == P + Q + R, "K muss P + Q + R sein");
        core::array::from_fn(|i| self.0[1 << i])
    }

    /// Vektor aus Koeffizienten der Generatoren `e_0 .. e_(K-1)`; `K = P + Q + R`.
    pub fn from_vector<const K: usize>(v: [T; K]) -> Self {
        assert!(K == P + Q + R, "K muss P + Q + R sein");
        let mut c = [T::ZERO; N];
        for (i, x) in v.into_iter().enumerate() {
            c[1 << i] = x;
        }
        Self::from_coeffs(c)
    }

    fn negate_where(self, negate: impl Fn(u32) -> bool) -> Self {
        let mut c = self.0;
        for (i, x) in c.iter_mut().enumerate() {
            if negate(i.count_ones()) {
                *x = -*x;
            }
        }
        Clifford(c)
    }

    /// Reverse `a~`: kehrt die Reihenfolge der Generatoren um (Vorzeichen `(-1)^(k(k-1)/2)`
    /// fuer Grad `k`). Es gilt `(a*b)~ = b~ * a~`.
    pub fn reverse(self) -> Self {
        self.negate_where(|k| (k * k.saturating_sub(1) / 2) & 1 == 1)
    }

    /// Gradinvolution: negiert alle Blades von ungeradem Grad. Es gilt `(a*b)^ = a^ * b^`.
    pub fn grade_involution(self) -> Self {
        self.negate_where(|k| k & 1 == 1)
    }

    /// Clifford-Konjugation (Reverse nach Gradinvolution), Vorzeichen `(-1)^(k(k+1)/2)`.
    pub fn clifford_conjugate(self) -> Self {
        self.negate_where(|k| (k * (k + 1) / 2) & 1 == 1)
    }

    fn product_where(self, o: Self, keep: impl Fn(usize, usize) -> bool) -> Self {
        let mut res = [T::ZERO; N];
        for i in 0..N {
            for j in 0..N {
                if !keep(i, j) {
                    continue;
                }
                let s = blade_sign(i, j, P, Q, R);
                if s == 0 {
                    continue;
                }
                let k = i ^ j;
                let t = self.0[i] * o.0[j];
                res[k] = if s > 0 { res[k] + t } else { res[k] - t };
            }
        }
        Clifford(res)
    }

    /// Aeusseres (Keil-)Produkt `a ^ b`; antisymmetrisch und assoziativ.
    pub fn wedge(self, o: Self) -> Self {
        self.product_where(o, |i, j| i & j == 0)
    }

    /// Linke Kontraktion `a _| b`: Grad `|b| - |a|`.
    pub fn left_contract(self, o: Self) -> Self {
        self.product_where(o, |i, j| i & j == i)
    }

    /// Rechte Kontraktion `a |_ b`: Grad `|a| - |b|`.
    pub fn right_contract(self, o: Self) -> Self {
        self.product_where(o, |i, j| i & j == j)
    }

    /// Skalarprodukt `<a * b>_0`.
    pub fn scalar_product(self, o: Self) -> T {
        let mut acc = T::ZERO;
        for i in 0..N {
            let t = self.0[i] * o.0[i];
            match blade_sign(i, i, P, Q, R) {
                1 => acc = acc + t,
                -1 => acc = acc - t,
                _ => {}
            }
        }
        acc
    }

    /// `<a * a~>_0`: Summe der Blade-Quadrate mit der Metrik der Signatur. Bei positiv
    /// definiten Signaturen (`Q = R = 0`) die Summe der Koeffizientenquadrate; bei
    /// indefiniten Signaturen kann der Wert negativ oder null sein.
    pub fn norm_sqr(self) -> T {
        self.scalar_product(self.reverse())
    }

    /// Sandwichprodukt `self * x * self~` (z.B. Rotation mit einem Rotor).
    pub fn sandwich(self, x: Self) -> Self {
        self * x * self.reverse()
    }

    /// Duale `a * I^-1` mit dem Pseudoskalar `I`; `None`, wenn `I` nicht invertierbar ist
    /// (degenerierte Signaturen, `R > 0`).
    pub fn dual(self) -> Option<Self> {
        let i = Self::pseudoscalar();
        let s = (i * i).0[0];
        if s == T::ZERO {
            return None;
        }
        // I^2 = s mit s = +-1, also I^-1 = s * I.
        Some(self * i.scale(s))
    }

    /// Ein Paar `(a, b)` mit `a != 0`, `b != 0`, `a * b == 0`; `None` fuer die
    /// Divisionsalgebren `Cl(0,0)`, `Cl(0,1)`, `Cl(0,2)`, die keine Nullteiler haben.
    ///
    /// * `R > 0`: ein degenerierter Generator `e` mit `e * e = 0`, Paar `(e, e)`.
    /// * `P > 0`: `(1 + e)(1 - e) = 1 - e^2 = 0` fuer einen positiven Generator.
    /// * `Q >= 3`: `w = e_0 e_1 e_2` hat `w^2 = +1`, Paar `(1 + w, 1 - w)`.
    pub fn try_zero_divisor_pair() -> Option<(Self, Self)> {
        if R > 0 {
            let e = Self::generator(P + Q);
            return Some((e, e));
        }
        let w = if P > 0 {
            Self::generator(0)
        } else if Q >= 3 {
            Self::blade(0b111, T::ONE)
        } else {
            return None;
        };
        let one = Self::from_scalar(T::ONE);
        Some((one + w, one - w))
    }
}

impl<T: Signed, const P: usize, const Q: usize, const R: usize, const N: usize> Add
    for Clifford<T, P, Q, R, N>
{
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Clifford(core::array::from_fn(|i| self.0[i] + o.0[i]))
    }
}
impl<T: Signed, const P: usize, const Q: usize, const R: usize, const N: usize> Sub
    for Clifford<T, P, Q, R, N>
{
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Clifford(core::array::from_fn(|i| self.0[i] - o.0[i]))
    }
}
impl<T: Signed, const P: usize, const Q: usize, const R: usize, const N: usize> Neg
    for Clifford<T, P, Q, R, N>
{
    type Output = Self;
    fn neg(self) -> Self {
        Clifford(self.0.map(|x| -x))
    }
}
/// Geometrisches Produkt.
impl<T: Signed, const P: usize, const Q: usize, const R: usize, const N: usize> Mul
    for Clifford<T, P, Q, R, N>
{
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        self.product_where(o, |_, _| true)
    }
}
impl<T: Signed, const P: usize, const Q: usize, const R: usize, const N: usize> AddAssign
    for Clifford<T, P, Q, R, N>
{
    fn add_assign(&mut self, o: Self) {
        *self = *self + o;
    }
}
impl<T: Signed, const P: usize, const Q: usize, const R: usize, const N: usize> SubAssign
    for Clifford<T, P, Q, R, N>
{
    fn sub_assign(&mut self, o: Self) {
        *self = *self - o;
    }
}
impl<T: Signed, const P: usize, const Q: usize, const R: usize, const N: usize> MulAssign
    for Clifford<T, P, Q, R, N>
{
    fn mul_assign(&mut self, o: Self) {
        *self = *self * o;
    }
}

impl<T: Signed, const P: usize, const Q: usize, const R: usize, const N: usize> TrivialZero
    for Clifford<T, P, Q, R, N>
{
    fn zero() -> Self {
        Self::from_coeffs([T::ZERO; N])
    }
    fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x == T::ZERO)
    }
}

impl<T: Signed, const P: usize, const Q: usize, const R: usize, const N: usize> Algebra
    for Clifford<T, P, Q, R, N>
{
    type Scalar = T;
    const DIM: usize = N;
    const COMMUTATIVE: bool = P + Q + R <= 1;
    const ASSOCIATIVE: bool = true;
    const ALTERNATIVE: bool = true;
    const FLEXIBLE: bool = true;
    const POWER_ASSOCIATIVE: bool = true;
    const MULTIPLICATIVE_NORM: bool = P == 0 && R == 0 && Q <= 2;
    const HAS_ZERO_DIVISORS: bool = !(P == 0 && R == 0 && Q <= 2);

    fn one() -> Self {
        Self::from_scalar(T::ONE)
    }
    fn from_scalar(s: T) -> Self {
        Self::blade(0, s)
    }
    fn coeff(&self, i: usize) -> T {
        self.0[i]
    }
    fn from_fn(f: impl FnMut(usize) -> T) -> Self {
        Self::from_coeffs(core::array::from_fn(f))
    }
    fn scale(self, s: T) -> Self {
        Clifford(self.0.map(|x| x * s))
    }
    /// `a~ / (a * a~)`, falls `a * a~` ein invertierbarer Skalar ist (Vektoren, Blades,
    /// Versoren); sonst `None`. Ein allgemeines Inverses gibt es in `Cl(p, q, r)` nicht
    /// in geschlossener Form.
    fn inverse(self) -> Option<Self> {
        let rev = self.reverse();
        let p = self * rev;
        if p.0[1..].iter().any(|&x| x != T::ZERO) {
            return None;
        }
        p.0[0].checked_recip().map(|r| rev.scale(r))
    }
}

// ---- Marker-Traits ---------------------------------------------------------------

macro_rules! always {
    ($($tr:ident),*) => {$(
        impl<T: Signed, const P: usize, const Q: usize, const R: usize, const N: usize> $tr
            for Clifford<T, P, Q, R, N> {}
    )*};
}
always!(Associative, Alternative, Flexible, PowerAssociative);

impl<T: Signed> Commutative for Clifford<T, 0, 0, 0, 1> {}
impl<T: Signed> Commutative for Clifford<T, 1, 0, 0, 2> {}
impl<T: Signed> Commutative for Clifford<T, 0, 1, 0, 2> {}
impl<T: Signed> Commutative for Clifford<T, 0, 0, 1, 2> {}

impl<T: Signed> MultiplicativeNorm for Clifford<T, 0, 0, 0, 1> {}
impl<T: Signed> MultiplicativeNorm for Clifford<T, 0, 1, 0, 2> {}
impl<T: Signed> MultiplicativeNorm for Clifford<T, 0, 2, 0, 4> {}

// ---- Verbindung zu Complex / Quaternion --------------------------------------------

impl<T: Signed> From<Complex<T>> for ClComplex<T> {
    /// `i = e_0`.
    fn from(c: Complex<T>) -> Self {
        Self::from_coeffs([c.coeff(0), c.coeff(1)])
    }
}
impl<T: Signed> From<ClComplex<T>> for Complex<T> {
    fn from(c: ClComplex<T>) -> Self {
        Complex::from_re_im(c.0[0], c.0[1])
    }
}
impl<T: Signed> From<Quaternion<T>> for ClQuaternion<T> {
    /// `i = e_0`, `j = e_1`, `k = e_0 e_1`.
    fn from(q: Quaternion<T>) -> Self {
        Self::from_coeffs([q.coeff(0), q.coeff(1), q.coeff(2), q.coeff(3)])
    }
}
impl<T: Signed> From<ClQuaternion<T>> for Quaternion<T> {
    fn from(c: ClQuaternion<T>) -> Self {
        Quaternion::from_wxyz(c.0[0], c.0[1], c.0[2], c.0[3])
    }
}

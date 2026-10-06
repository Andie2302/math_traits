// GENERIERT von tools/gen_clifford.py -- nicht von Hand aendern.
use super::Clifford;
use crate::scalar::Signed;
use crate::traits::NonTrivialZero;

/// Cl(0,1) ~ komplexe Zahlen (Dimension 2).
pub type ClComplex<T> = Clifford<T, 0, 1, 0, 2>;

/// Cl(0,2) ~ Quaternionen (Dimension 4).
pub type ClQuaternion<T> = Clifford<T, 0, 2, 0, 4>;

/// Cl(1,0): hyperbolische (split-komplexe) Zahlen, `e^2 = +1` (Dimension 2).
pub type SplitComplex<T> = Clifford<T, 1, 0, 0, 2>;

impl<T: Signed> NonTrivialZero for SplitComplex<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(0,0,1): duale Zahlen, `e^2 = 0` (Dimension 2).
pub type DualNumber<T> = Clifford<T, 0, 0, 1, 2>;

impl<T: Signed> NonTrivialZero for DualNumber<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(2,0): Geometrische Algebra der Ebene (Dimension 4).
pub type Cl2<T> = Clifford<T, 2, 0, 0, 4>;

impl<T: Signed> NonTrivialZero for Cl2<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(3,0): Geometrische Algebra des 3D-Raums (Pauli-Algebra) (Dimension 8).
pub type Cl3<T> = Clifford<T, 3, 0, 0, 8>;

impl<T: Signed> NonTrivialZero for Cl3<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(4,0): Euklidische Geometrische Algebra in 4D (Dimension 16).
pub type Cl4<T> = Clifford<T, 4, 0, 0, 16>;

impl<T: Signed> NonTrivialZero for Cl4<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(1,3): Raumzeit-Algebra (STA), Signatur (+,-,-,-) (Dimension 16).
pub type Sta<T> = Clifford<T, 1, 3, 0, 16>;

impl<T: Signed> NonTrivialZero for Sta<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(3,1): Minkowski-Algebra, Signatur (+,+,+,-) (Dimension 16).
pub type Cl31<T> = Clifford<T, 3, 1, 0, 16>;

impl<T: Signed> NonTrivialZero for Cl31<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(2,0,1): projektive Geometrische Algebra der Ebene (Dimension 8).
pub type Pga2<T> = Clifford<T, 2, 0, 1, 8>;

impl<T: Signed> NonTrivialZero for Pga2<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(3,0,1): projektive Geometrische Algebra des 3D-Raums (Dimension 16).
pub type Pga3<T> = Clifford<T, 3, 0, 1, 16>;

impl<T: Signed> NonTrivialZero for Pga3<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(4,1): konforme Geometrische Algebra des 3D-Raums (Dimension 32).
pub type Cga3<T> = Clifford<T, 4, 1, 0, 32>;

impl<T: Signed> NonTrivialZero for Cga3<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(0,0,2): aeussere (Grassmann-)Algebra in 2D (Dimension 4).
pub type Grassmann2<T> = Clifford<T, 0, 0, 2, 4>;

impl<T: Signed> NonTrivialZero for Grassmann2<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(0,0,3): aeussere (Grassmann-)Algebra in 3D (Dimension 8).
pub type Grassmann3<T> = Clifford<T, 0, 0, 3, 8>;

impl<T: Signed> NonTrivialZero for Grassmann3<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

/// Cl(0,0,4): aeussere (Grassmann-)Algebra in 4D (Dimension 16).
pub type Grassmann4<T> = Clifford<T, 0, 0, 4, 16>;

impl<T: Signed> NonTrivialZero for Grassmann4<T> {
    fn zero_divisor_pair() -> (Self, Self) {
        Self::try_zero_divisor_pair().expect("Algebra mit Nullteilern")
    }
}

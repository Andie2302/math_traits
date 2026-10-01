//! Lie-Klammern auf Tensoren und die Drehgruppe.
//!
//! * `Tensor1<T, 3>` mit dem Kreuzprodukt ist die Lie-Algebra so(3).
//! * Quadratische Matrizen mit dem Kommutator `[A, B] = AB − BA` sind gl(n).
//! * Drehvektor → Einheitsquaternion ist die Exponentialabbildung so(3) → SU(2).
//!   Einheitsquaternionen drehen Vektoren per `q·v·q*`.

use super::{Tensor1, Tensor2};
use crate::laws;
use crate::signature::{Additive, BinaryOp, Bracket, HasIdentity, HasInverse, Multiplicative, op};
use crate::structures::CommutativeRing;

/// Kreuzprodukt `[a, b] = a × b`.
impl<T> BinaryOp<Bracket> for Tensor1<T, 3>
where
    T: BinaryOp<Multiplicative> + BinaryOp<Additive> + HasInverse<Additive>,
{
    fn op(&self, b: &Self) -> Self {
        let a = &self.0;
        let c = |i: usize, j: usize| {
            op::<Additive, _>(
                &op::<Multiplicative, _>(&a[i], &b.0[j]),
                &op::<Multiplicative, _>(&a[j], &b.0[i]).inverse(),
            )
        };
        Tensor1([c(1, 2), c(2, 0), c(0, 1)])
    }
}

/// Kommutator `[A, B] = AB − BA`.
impl<T, const N: usize> BinaryOp<Bracket> for Tensor2<T, N, N>
where
    T: BinaryOp<Multiplicative> + BinaryOp<Additive> + HasIdentity<Additive> + HasInverse<Additive>,
{
    fn op(&self, b: &Self) -> Self {
        let ab = op::<Multiplicative, _>(self, b);
        let ba = op::<Multiplicative, _>(b, self);
        op::<Additive, _>(&ab, &ba.inverse())
    }
}

laws! {
    for[T: CommutativeRing] Tensor1<T, 3> {
        [Bracket, Additive]: lie_bracket;
        [Bracket, T]: op_homogeneous;
    }
    for[T: CommutativeRing, const N: usize] Tensor2<T, N, N> {
        [Bracket, Additive]: lie_bracket;
        [Bracket, T]: op_homogeneous;
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
mod rotation {
    use super::Tensor1;
    use crate::impls::cayley_dickson::{Complex, Quaternion};
    use crate::impls::fmath::FMath;
    use crate::laws;
    use crate::signature::{ExpMap, HasConjugate, Multiplicative, ScalarMul, op};

    macro_rules! rotation {
        ($($b:ty),*) => {$(
            /// `exp(v) = (cos(θ/2), sin(θ/2)·v/θ)` mit `θ = ‖v‖`: Drehung um die
            /// Achse `v` mit dem Winkel `‖v‖`.
            impl ExpMap<Quaternion<$b>> for Tensor1<$b, 3> {
                fn exp_map(&self) -> Quaternion<$b> {
                    let [x, y, z] = self.0;
                    let theta = (x * x + y * y + z * z).m_sqrt();
                    let half = theta / 2.0;
                    // sin(θ/2)/θ, stetig fortgesetzt bei θ = 0
                    let k = if theta > 1e-12 { half.m_sin() / theta } else { 0.5 };
                    Quaternion::new(Complex::new(half.m_cos(), k * x), Complex::new(k * y, k * z))
                }
            }

            /// Drehung `q·v·q*` (für Einheitsquaternionen eine echte Drehung).
            impl ScalarMul<Quaternion<$b>> for Tensor1<$b, 3> {
                fn scale(&self, q: &Quaternion<$b>) -> Self {
                    let [x, y, z] = self.0;
                    let v = Quaternion::new(Complex::new(0.0, x), Complex::new(y, z));
                    let r = op::<Multiplicative, _>(&op::<Multiplicative, _>(q, &v), &q.conj());
                    Tensor1([r.re.im, r.im.re, r.im.im])
                }
            }

            laws! {
                Tensor1<$b, 3> {
                    Quaternion<$b>: exp_map_zero, exp_map_negation, scalar_identity, scalar_compatible;
                }
            }
        )*};
    }

    rotation!(f32, f64);
}

//! Dynamischer Tensor: Form und Stufe erst zur Laufzeit bekannt (`alloc`).
//!
//! Weil die Formen nicht im Typ stehen, können Operationen scheitern. Statt
//! `BinaryOp` gibt es deshalb [`TryBinaryOp`] und [`TryContract`] mit
//! `Option` als Ergebnis. Die Gesetze gelten dort, wo beide Seiten definiert
//! sind ([`PartialAssociative`](crate::laws::PartialAssociative),
//! [`PartialCommutative`](crate::laws::PartialCommutative)).

use alloc::vec::Vec;

use super::TensorShape;
use crate::laws;
use crate::signature::{
    Additive, BinaryOp, HasIdentity, Multiplicative, Outer, ScalarMul, TryBinaryOp, TryContract, op,
};
use crate::structures::AbelianGroup;

/// Tensor beliebiger Stufe, zeilenweise gespeichert.
#[derive(Clone, PartialEq, Debug)]
pub struct DynTensor<T> {
    shape: Vec<usize>,
    data: Vec<T>,
}

impl<T> DynTensor<T> {
    /// `None`, wenn `data.len()` nicht zum Produkt der Form passt.
    pub fn new(shape: Vec<usize>, data: Vec<T>) -> Option<Self> {
        (shape.iter().product::<usize>() == data.len()).then_some(Self { shape, data })
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Nullen der gegebenen Form.
    pub fn zeros(shape: Vec<usize>) -> Self
    where
        T: HasIdentity<Additive>,
    {
        let n = shape.iter().product();
        Self {
            shape,
            data: (0..n).map(|_| T::identity()).collect(),
        }
    }

    /// Aus einem Tensor fester Größe.
    pub fn from_fixed<F: TensorShape<Elem = T>>(t: F) -> Self
    where
        T: Clone,
    {
        Self {
            shape: F::SHAPE.to_vec(),
            data: t.as_slice().to_vec(),
        }
    }

    /// In einen Tensor fester Größe, wenn die Form passt.
    pub fn to_fixed<F: TensorShape<Elem = T>>(&self) -> Option<F>
    where
        T: Clone,
    {
        (self.shape == F::SHAPE).then(|| F::from_flat_fn(|i| self.data[i].clone()))
    }

    /// Neue Form mit gleicher Elementzahl.
    pub fn reshape(self, shape: Vec<usize>) -> Option<Self> {
        Self::new(shape, self.data)
    }
}

impl<T: BinaryOp<Additive>> TryBinaryOp<Additive> for DynTensor<T> {
    fn try_op(&self, rhs: &Self) -> Option<Self> {
        (self.shape == rhs.shape).then(|| Self {
            shape: self.shape.clone(),
            data: self
                .data
                .iter()
                .zip(&rhs.data)
                .map(|(a, b)| op::<Additive, _>(a, b))
                .collect(),
        })
    }
}

impl<T: BinaryOp<Multiplicative>> ScalarMul<T> for DynTensor<T> {
    fn scale(&self, s: &T) -> Self {
        Self {
            shape: self.shape.clone(),
            data: self
                .data
                .iter()
                .map(|a| op::<Multiplicative, _>(s, a))
                .collect(),
        }
    }
}

/// Letzter Index links mit erstem Index rechts.
impl<T> TryContract<DynTensor<T>> for DynTensor<T>
where
    T: BinaryOp<Multiplicative> + BinaryOp<Additive> + HasIdentity<Additive>,
{
    type Output = DynTensor<T>;
    fn try_contract(&self, rhs: &DynTensor<T>) -> Option<DynTensor<T>> {
        let (&k, left) = self.shape.split_last()?;
        let (&k2, right) = rhs.shape.split_first()?;
        if k != k2 {
            return None;
        }
        let p: usize = left.iter().product();
        let q: usize = right.iter().product();
        let data = (0..p * q)
            .map(|idx| {
                let (i, j) = (idx / q, idx % q);
                (0..k).fold(T::identity(), |acc, l| {
                    op::<Additive, _>(
                        &acc,
                        &op::<Multiplicative, _>(&self.data[i * k + l], &rhs.data[l * q + j]),
                    )
                })
            })
            .collect();
        let shape = left.iter().chain(right).copied().collect();
        Some(Self { shape, data })
    }
}

impl<T: BinaryOp<Multiplicative>> Outer<DynTensor<T>> for DynTensor<T> {
    type Output = DynTensor<T>;
    fn outer(&self, rhs: &DynTensor<T>) -> DynTensor<T> {
        let q = rhs.data.len();
        let data = (0..self.data.len() * q)
            .map(|idx| op::<Multiplicative, _>(&self.data[idx / q], &rhs.data[idx % q]))
            .collect();
        let shape = self.shape.iter().chain(&rhs.shape).copied().collect();
        Self { shape, data }
    }
}

laws! {
    for[T: AbelianGroup<Additive>] DynTensor<T> {
        Additive: partial_associative, partial_commutative;
    }
}

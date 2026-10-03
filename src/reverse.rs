//! Rückwärts-AutoDiff („Backpropagation“): alle Steigungen in einem Durchlauf.
//!
//! Beim Vorwärtsrechnen wird jeder Schritt auf einem [`Tape`] notiert, zusammen
//! mit seinen **lokalen Wechselkursen**: Wie stark ändert sich das Ergebnis,
//! wenn sich ein Eingang ein wenig ändert? Rückwärts werden diese Kurse vom
//! Ergebnis aus zu allen Eingängen durchmultipliziert (Kettenregel) und dort
//! aufsummiert, wo sich Wege treffen.
//!
//! Vorwärts-AutoDiff ([`Dual`](crate::autodiff::Dual)) braucht einen Durchlauf
//! **pro Eingang**, rückwärts reicht **einer** für alle, ideal, wenn wie bei
//! neuronalen Netzen eine einzige Zahl (der Fehler) von sehr vielen Gewichten
//! abhängt.
//!
//! ```
//! use math_traits::reverse::Tape;
//!
//! // Mini-Netz aus Lektion 2: y = w₂ · ReLU(w₁ · x), Fehler = (y − t)²
//! let tape = Tape::new();
//! let (w1, w2) = (tape.var(2.0), tape.var(3.0));
//! let x = tape.constant(1.0);
//! let y = w2 * (w1 * x).relu();
//! let err = (y - tape.constant(5.0)).square();
//! let grad = tape.gradient(err);
//! assert_eq!(err.value(), 1.0);
//! assert_eq!(grad[w1.index()], 6.0);
//! assert_eq!(grad[w2.index()], 4.0);
//! ```

use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::ops::{Add, Mul, Neg, Sub};

use crate::signature::{
    Additive, BinaryOp, BinaryRelation, HasExp, HasIdentity, HasInverse, HasLn, HasPartialInverse,
    LessEq, Multiplicative, op,
};

/// Ein Rechenschritt: bis zu zwei Eingänge mit ihren lokalen Wechselkursen.
#[derive(Clone)]
struct Node<T> {
    parents: [(usize, T); 2],
    arity: usize,
}

/// Das Band, auf dem alle Rechenschritte notiert werden.
pub struct Tape<T> {
    nodes: RefCell<Vec<Node<T>>>,
}

/// Ein Wert auf dem Band. Klein und `Copy`, rechnet mit `+ − *`.
#[derive(Clone, Copy)]
pub struct Var<'t, T> {
    tape: &'t Tape<T>,
    index: usize,
    value: T,
}

impl<T> Default for Tape<T> {
    fn default() -> Self {
        Self {
            nodes: RefCell::new(Vec::new()),
        }
    }
}

impl<T> Tape<T>
where
    T: Copy
        + BinaryOp<Additive>
        + BinaryOp<Multiplicative>
        + HasIdentity<Additive>
        + HasIdentity<Multiplicative>,
{
    pub fn new() -> Self {
        Self::default()
    }

    fn push(&self, value: T, parents: &[(usize, T)]) -> Var<'_, T> {
        let zero = <T as HasIdentity<Additive>>::identity();
        let mut nodes = self.nodes.borrow_mut();
        let mut p = [(0, zero), (0, zero)];
        p[..parents.len()].copy_from_slice(parents);
        nodes.push(Node {
            parents: p,
            arity: parents.len(),
        });
        Var {
            tape: self,
            index: nodes.len() - 1,
            value,
        }
    }

    /// Eine Größe, nach der abgeleitet wird (z. B. ein Gewicht).
    pub fn var(&self, value: T) -> Var<'_, T> {
        self.push(value, &[])
    }

    /// Eine Konstante. Technisch dasselbe wie [`Tape::var`]; ihre Steigung
    /// wird einfach nicht abgefragt.
    pub fn constant(&self, value: T) -> Var<'_, T> {
        self.push(value, &[])
    }

    /// Anzahl der notierten Schritte.
    pub fn len(&self) -> usize {
        self.nodes.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Alle Steigungen `∂out/∂v` für jeden Wert `v` auf dem Band, abrufbar über
    /// [`Var::index`]. Ein einziger Rückwärtsdurchlauf.
    pub fn gradient(&self, out: Var<'_, T>) -> Vec<T> {
        let nodes = self.nodes.borrow();
        let zero = <T as HasIdentity<Additive>>::identity();
        let mut grad = vec![zero; nodes.len()];
        grad[out.index] = <T as HasIdentity<Multiplicative>>::identity();
        for i in (0..=out.index).rev() {
            let g = grad[i];
            let node = &nodes[i];
            for &(parent, rate) in &node.parents[..node.arity] {
                // bisherige Zahl × Wechselkurs, aufsummiert, wo Wege zusammenlaufen
                grad[parent] =
                    op::<Additive, _>(&grad[parent], &op::<Multiplicative, _>(&g, &rate));
            }
        }
        grad
    }
}

impl<'t, T> Var<'t, T>
where
    T: Copy
        + BinaryOp<Additive>
        + BinaryOp<Multiplicative>
        + HasIdentity<Additive>
        + HasIdentity<Multiplicative>,
{
    pub fn value(&self) -> T {
        self.value
    }

    /// Position auf dem Band, Index in das Ergebnis von [`Tape::gradient`].
    pub fn index(&self) -> usize {
        self.index
    }

    fn unary(self, value: T, rate: T) -> Self {
        self.tape.push(value, &[(self.index, rate)])
    }

    /// `x²`, Wechselkurs `2x`.
    pub fn square(self) -> Self {
        let two_x = op::<Additive, _>(&self.value, &self.value);
        self.unary(op::<Multiplicative, _>(&self.value, &self.value), two_x)
    }

    /// `ReLU(x) = max(x, 0)`, Wechselkurs 1 für `x > 0`, sonst 0.
    pub fn relu(self) -> Self
    where
        T: BinaryRelation<LessEq>,
    {
        let zero = <T as HasIdentity<Additive>>::identity();
        if self.value.relates(&zero) {
            self.unary(zero, zero)
        } else {
            self.unary(self.value, <T as HasIdentity<Multiplicative>>::identity())
        }
    }

    /// `eˣ`, Wechselkurs `eˣ`.
    pub fn exp(self) -> Self
    where
        T: HasExp,
    {
        let e = self.value.exp();
        self.unary(e, e)
    }

    /// `ln x`, Wechselkurs `1/x`. Panickt außerhalb des Definitionsbereichs.
    pub fn ln(self) -> Self
    where
        T: HasLn + HasPartialInverse<Multiplicative>,
    {
        let l = self
            .value
            .ln()
            .expect("ln: außerhalb des Definitionsbereichs");
        self.unary(l, self.value.try_inverse().expect("ln: x ≠ 0"))
    }

    /// Logistische Funktion `σ(x) = 1/(1 + e⁻ˣ)`, Wechselkurs `σ(x)·(1 − σ(x))`.
    pub fn sigmoid(self) -> Self
    where
        T: HasExp + HasInverse<Additive> + HasPartialInverse<Multiplicative>,
    {
        let one = <T as HasIdentity<Multiplicative>>::identity();
        let s = op::<Additive, _>(&one, &self.value.inverse().exp())
            .try_inverse()
            .expect("1 + e⁻ˣ > 0");
        let rate = op::<Multiplicative, _>(&s, &op::<Additive, _>(&one, &s.inverse()));
        self.unary(s, rate)
    }
}

impl<'t, T> Add for Var<'t, T>
where
    T: Copy
        + BinaryOp<Additive>
        + BinaryOp<Multiplicative>
        + HasIdentity<Additive>
        + HasIdentity<Multiplicative>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let one = <T as HasIdentity<Multiplicative>>::identity();
        self.tape.push(
            op::<Additive, _>(&self.value, &rhs.value),
            &[(self.index, one), (rhs.index, one)],
        )
    }
}

impl<'t, T> Mul for Var<'t, T>
where
    T: Copy
        + BinaryOp<Additive>
        + BinaryOp<Multiplicative>
        + HasIdentity<Additive>
        + HasIdentity<Multiplicative>,
{
    type Output = Self;
    /// `a·b`, Wechselkurse `b` (für `a`) und `a` (für `b`).
    fn mul(self, rhs: Self) -> Self {
        self.tape.push(
            op::<Multiplicative, _>(&self.value, &rhs.value),
            &[(self.index, rhs.value), (rhs.index, self.value)],
        )
    }
}

impl<'t, T> Neg for Var<'t, T>
where
    T: Copy
        + BinaryOp<Additive>
        + BinaryOp<Multiplicative>
        + HasIdentity<Additive>
        + HasIdentity<Multiplicative>
        + HasInverse<Additive>,
{
    type Output = Self;
    fn neg(self) -> Self {
        let minus_one = <T as HasIdentity<Multiplicative>>::identity().inverse();
        self.unary(self.value.inverse(), minus_one)
    }
}

impl<'t, T> Sub for Var<'t, T>
where
    T: Copy
        + BinaryOp<Additive>
        + BinaryOp<Multiplicative>
        + HasIdentity<Additive>
        + HasIdentity<Multiplicative>
        + HasInverse<Additive>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

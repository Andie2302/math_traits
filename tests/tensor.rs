//! Tensoren, Matrizen, lineare Algebra, Lie-Algebren, dynamische Tensoren.

use math_traits::autodiff::{Dual, jacobian_fixed};
use math_traits::fft::{fft_nd, ifft_nd};
use math_traits::impls::cayley_dickson::{Complex, Quaternion};
use math_traits::laws::*;
use math_traits::signature::*;
use math_traits::structures::*;
use math_traits::tensor::dynamic::DynTensor;
use math_traits::tensor::matrix::*;
use math_traits::tensor::*;

type M3 = SquareMatrix<i64, 3>;

fn m(rows: [[i64; 3]; 3]) -> M3 {
    Tensor2(rows)
}

fn mul<T: BinaryOp<Multiplicative>>(a: &T, b: &T) -> T {
    op::<Multiplicative, _>(a, b)
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-10
}

#[test]
fn structures() {
    fn ring<T: Ring>() {}
    fn algebra<A: AssociativeAlgebra<S>, S>() {}
    fn euclidean<V: EuclideanSpace<S>, S: RealField>() {}
    fn module<V: Module<S>, S: Ring>() {}
    fn lie<L: LieAlgebra<S>, S: CommutativeRing>() {}
    fn bilinear<A: BilinearContract<B, S>, B, S>() {}

    ring::<SquareMatrix<f64, 4>>();
    ring::<SquareMatrix<Complex<f64>, 2>>();
    algebra::<SquareMatrix<i64, 3>, i64>();
    euclidean::<Matrix<f64, 2, 5>, f64>();
    euclidean::<Tensor4<f32, 2, 2, 2, 2>, f32>();
    module::<Tensor3<Quaternion<f64>, 2, 3, 4>, Quaternion<f64>>();
    lie::<Tensor1<f64, 3>, f64>();
    lie::<SquareMatrix<i64, 4>, i64>();
    bilinear::<Matrix<f64, 2, 3>, Matrix<f64, 3, 4>, f64>();
    bilinear::<Matrix<f64, 2, 3>, Vector<f64, 3>, f64>();
}

#[test]
fn matrix_ring_and_contraction_laws_exact() {
    let ms = [
        m([[1, 2, 0], [-1, 3, 4], [2, 0, 1]]),
        m([[0, 1, 1], [5, -2, 0], [1, 1, -3]]),
        m([[2, 0, 0], [0, 2, 0], [7, 0, 2]]),
    ];
    for a in &ms {
        assert!(<M3 as LeftIdentity<Multiplicative>>::holds(a));
        for b in &ms {
            for c in &ms {
                assert!(<M3 as Associative<Multiplicative>>::holds(a, b, c));
                assert!(<M3 as LeftDistributive<Multiplicative, Additive>>::holds(
                    a, b, c
                ));
            }
            assert!(<M3 as OpHomogeneous<Multiplicative, i64>>::holds(&-3, a, b));
        }
    }
    // Rechteckig: (2×3)·(3×2)·(2×4)
    let a = Tensor2([[1i64, 2, 3], [4, 5, 6]]);
    let b = Tensor2([[1i64, 0], [0, 1], [1, 1]]);
    let c = Tensor2([[1i64, 2, 3, 4], [0, -1, 0, 1]]);
    assert!(<Matrix<i64, 2, 3> as ContractAssociative<
        Matrix<i64, 3, 2>,
        Matrix<i64, 2, 4>,
    >>::holds(&a, &b, &c));
    let v = Tensor1([1i64, -1]);
    assert!(<Matrix<i64, 2, 3> as ContractAssociative<
        Matrix<i64, 3, 2>,
        Vector<i64, 2>,
    >>::holds(&a, &b, &v));
    assert_eq!(a.contract(&b), Tensor2([[4, 5], [10, 11]]));
    assert_eq!(a.contract(&Tensor1([1, 1, 1])), Tensor1([6, 15]));
    assert_eq!(Tensor1([1i64, 2]).contract(&a), Tensor1([9, 12, 15]));
    assert_eq!(Tensor1([1i64, 2, 3]).contract(&Tensor1([4, 5, 6])), 32);
}

#[test]
fn higher_rank_contraction_and_outer() {
    let x = Tensor1([1i64, 2]);
    let y = Tensor1([3i64, 4, 5]);
    let xy = x.outer(&y);
    assert_eq!(xy, Tensor2([[3, 4, 5], [6, 8, 10]]));
    let z = Tensor1([1i64, -1]);
    let xyz: Tensor3<i64, 2, 3, 2> = xy.outer(&z);
    assert_eq!(xyz.0[1][2], [10, -10]);
    assert!(<Vector<i64, 2> as OuterAssociative<
        Vector<i64, 3>,
        Vector<i64, 2>,
    >>::holds(&x, &y, &z));
    // Kontraktion senkt die Stufe wieder: (x⊗y⊗z)·w = (z·w)·(x⊗y)
    let w = Tensor1([2i64, 1]);
    let back: Tensor2<i64, 2, 3> = xyz.contract(&w);
    assert_eq!(back, xy.scale(&z.contract(&w)));
    // Stufe 4: (x⊗y) ⊗ (y⊗x)
    let t4: Tensor4<i64, 2, 3, 3, 2> = xy.outer(&y.outer(&x));
    assert_eq!(t4.0[1][2][0][1], 10 * 3 * 2);
    assert_eq!(
        <Tensor4<i64, 2, 3, 3, 2> as TensorShape>::SHAPE,
        &[2, 3, 3, 2]
    );
}

#[test]
fn lie_algebras_exact() {
    let vs = [
        Tensor1([1i64, 2, 3]),
        Tensor1([-2, 0, 5]),
        Tensor1([4, -1, 1]),
    ];
    for a in &vs {
        assert!(<Tensor1<i64, 3> as Alternating<Bracket, Additive>>::holds(
            a
        ));
        for b in &vs {
            for c in &vs {
                assert!(<Tensor1<i64, 3> as Jacobi<Bracket, Additive>>::holds(
                    a, b, c
                ));
            }
        }
    }
    assert_eq!(
        op::<Bracket, _>(&Tensor1([1i64, 0, 0]), &Tensor1([0, 1, 0])),
        Tensor1([0, 0, 1])
    );

    let ms = [
        m([[1, 2, 0], [0, 1, 4], [2, 0, 1]]),
        m([[0, 1, 1], [1, 0, 0], [1, 1, 0]]),
        m([[3, 0, 0], [1, 2, 0], [0, 1, 1]]),
    ];
    for a in &ms {
        assert!(<M3 as Alternating<Bracket, Additive>>::holds(a));
        for b in &ms {
            for c in &ms {
                assert!(<M3 as Jacobi<Bracket, Additive>>::holds(a, b, c));
            }
        }
    }
}

#[test]
fn rotations_via_exp_map() {
    use std::f64::consts::FRAC_PI_2;
    // Drehung um z um 90°: x ↦ y
    let q: Quaternion<f64> = Tensor1([0.0, 0.0, FRAC_PI_2]).exp_map();
    let r = Tensor1([1.0, 0.0, 0.0]).scale(&q);
    assert!(close(r.0[0], 0.0) && close(r.0[1], 1.0) && close(r.0[2], 0.0));
    assert!(<Tensor1<f64, 3> as ExpMapZero<Quaternion<f64>>>::holds());
    // Zwei Vierteldrehungen = halbe Drehung
    let r2 = Tensor1([1.0, 0.0, 0.0]).scale(&mul(&q, &q));
    assert!(close(r2.0[0], -1.0) && close(r2.0[1], 0.0));
}

#[test]
fn linear_algebra_exact_and_float() {
    let a = Tensor2([[2.0, 1.0, 1.0], [4.0, -6.0, 0.0], [-2.0, 7.0, 2.0]]);
    assert!(close(det(&a), -16.0));
    let inv = inverse(&a).unwrap();
    let id = mul(&a, &inv);
    for i in 0..3 {
        for j in 0..3 {
            assert!(close(id.0[i][j], if i == j { 1.0 } else { 0.0 }));
        }
    }
    // LU mit Pivot bei a₀₀ = 0
    let b = Tensor2([[0.0, 2.0, 1.0], [1.0, -2.0, -3.0], [-1.0, 1.0, 2.0]]);
    let d = lu_pivoting(&b).unwrap();
    let x = d.solve(&Tensor1([-8.0, 0.0, 3.0])).unwrap();
    assert!(close(x.0[0], -4.0) && close(x.0[1], -5.0) && close(x.0[2], 2.0));
    assert!(is_lower_triangular(&d.l()) && is_upper_triangular(&d.u()));
    assert_eq!(inverse(&Tensor2([[1.0, 2.0], [2.0, 4.0]])), None);
    assert_eq!(det(&Tensor2([[1.0, 2.0], [2.0, 4.0]])), 0.0);

    assert_eq!(trace(&m([[1, 2, 3], [4, 5, 6], [7, 8, 9]])), 15);
    assert_eq!(transpose(&Tensor2([[1, 2, 3]])), Tensor2([[1], [2], [3]]));
    assert!(is_diagonal(&from_diagonal(&Tensor1([1, 2, 3]))));
}

#[test]
fn qr_and_symmetric_eigen() {
    let a: Matrix<f64, 3, 3> =
        Tensor2([[12.0, -51.0, 4.0], [6.0, 167.0, -68.0], [-4.0, 24.0, -41.0]]);
    let (q, r) = qr(&a).unwrap();
    assert!(is_upper_triangular(&r));
    let qr_prod = q.contract(&r);
    let qtq = transpose(&q).contract(&q);
    for i in 0..3 {
        for j in 0..3 {
            assert!((qr_prod.0[i][j] - a.0[i][j]).abs() < 1e-9);
            assert!(close(qtq.0[i][j], if i == j { 1.0 } else { 0.0 }));
        }
    }

    let s: SquareMatrix<f64, 3> = Tensor2([[4.0, 1.0, 2.0], [1.0, 3.0, 0.0], [2.0, 0.0, 5.0]]);
    let (lambda, v) = eigen_symmetric(&s, &1e-12, 50).unwrap();
    for k in 0..3 {
        let vk = Tensor1([v.0[0][k], v.0[1][k], v.0[2][k]]);
        let sv = s.contract(&vk);
        for i in 0..3 {
            assert!((sv.0[i] - lambda.0[k] * vk.0[i]).abs() < 1e-9);
        }
    }
    let sum: f64 = lambda.0.iter().sum();
    assert!(close(sum, 12.0)); // Spur
}

#[test]
fn fft_on_fixed_tensor() {
    type C = Complex<f64>;
    let t: Tensor2<C, 4, 8> = Tensor2::from_flat_fn(|i| C::new(i as f64, (i * i) as f64 * 0.1));
    let mut u = t;
    fft_nd(u.as_mut_slice(), Tensor2::<C, 4, 8>::SHAPE).unwrap();
    ifft_nd(u.as_mut_slice(), Tensor2::<C, 4, 8>::SHAPE).unwrap();
    for (a, b) in t.as_slice().iter().zip(u.as_slice()) {
        assert!((a.re - b.re).abs() < 1e-10 && (a.im - b.im).abs() < 1e-10);
    }
}

#[test]
fn dynamic_tensor() {
    let a = DynTensor::from_fixed(Tensor2([[1i64, 2, 3], [4, 5, 6]]));
    let b = DynTensor::new(vec![3, 2], vec![1i64, 0, 0, 1, 1, 1]).unwrap();
    let c = a.try_contract(&b).unwrap();
    assert_eq!(
        c.to_fixed::<Tensor2<i64, 2, 2>>(),
        Some(Tensor2([[4, 5], [10, 11]]))
    );
    assert_eq!(a.try_contract(&a), None); // 3 ≠ 2
    assert_eq!(a.try_op(&b), None); // Formen verschieden
    assert!(<DynTensor<i64> as PartialAssociative<Additive>>::holds(
        &a, &b, &a
    ));
    assert!(<DynTensor<i64> as PartialCommutative<Additive>>::holds(
        &a, &a
    ));
    assert_eq!(a.outer(&b).shape(), &[2, 3, 3, 2]);
    assert_eq!(DynTensor::<i64>::new(vec![2, 2], vec![1, 2, 3]), None);
}

#[test]
fn fixed_jacobian() {
    // F(x, y) = (x²·y, x + y)
    let f = |v: &Tensor1<Dual<f64>, 2>| {
        let [x, y] = &v.0;
        Tensor1([mul(&mul(x, x), y), op::<Additive, _>(x, y)])
    };
    let (val, jac) = jacobian_fixed(f, &Tensor1([2.0, 3.0]));
    assert_eq!(val, Tensor1([12.0, 5.0]));
    assert_eq!(jac, Tensor2([[12.0, 4.0], [1.0, 1.0]]));
}

use math_traits::*;

#[allow(clippy::eq_op)]
fn generic_basics<X: Tensor + core::fmt::Debug>()
where
    X::Scalar: From<u8> + core::fmt::Debug,
{
    let z = X::zero();
    assert!(z.is_zero());
    let a = X::from_fn(|idx| X::Scalar::from((idx.iter().sum::<usize>() % 5) as u8 + 1));
    assert_eq!(a + z, a);
    assert_eq!(a - a, z);
    assert_eq!(a.scale(X::Scalar::ONE), a);
    assert!(!a.is_zero() || X::LEN == 0);
    // Anzahl der Eintraege = LEN, Summe wird ueber fold gezaehlt
    assert_eq!(a.fold(0usize, |n, _| n + 1), X::LEN);
    // Rang/Shape konsistent
    let mut shape = [0usize; MAX_RANK];
    X::shape_into(&mut shape);
    assert_eq!(shape[..X::RANK].iter().product::<usize>(), X::LEN);
    // get/get_mut ueber Mehrfachindex, falsche Laenge -> None
    let mut b = a;
    let first = [0usize; MAX_RANK + 1];
    *b.get_mut(&first[..X::RANK]).unwrap() = X::Scalar::from(9);
    assert_eq!(*b.get(&first[..X::RANK]).unwrap(), X::Scalar::from(9));
    assert!(b.get(&first[..X::RANK + 1]).is_none());
}

#[test]
fn ranks_0_to_6_and_32() {
    assert_eq!(Tensor0::<i32>::RANK, 0);
    assert_eq!(Tensor1::<i32, 3>::RANK, 1);
    assert_eq!(Tensor2::<i32, 2, 3>::LEN, 6);
    generic_basics::<Tensor0<i32>>();
    generic_basics::<Tensor1<u8, 4>>();
    generic_basics::<Tensor2<i64, 2, 3>>();
    generic_basics::<Tensor3<f32, 2, 3, 4>>();
    generic_basics::<Tensor4<usize, 1, 2, 1, 3>>();
    generic_basics::<Tensor5<f64, 2, 1, 2, 1, 2>>();
    generic_basics::<Tensor6<i16, 2, 2, 2, 2, 2, 2>>();
}

#[test]
fn rank_32_with_different_axis_lengths() {
    type T32 = Tensor32<
        i32,
        2,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        3,
        2,
    >;
    assert_eq!(T32::RANK, 32);
    assert_eq!(T32::LEN, 12);
    assert_eq!(T32::dim(0), 2);
    assert_eq!(T32::dim(30), 3);
    assert_eq!(T32::dim(31), 2);
    generic_basics::<T32>();
    let t = T32::from_fn(|i| (i[0] * 100 + i[30] * 10 + i[31]) as i32);
    let mut idx = [0usize; 32];
    idx[0] = 1;
    idx[30] = 2;
    idx[31] = 1;
    assert_eq!(*t.get(&idx).unwrap(), 121);
    assert_eq!(
        t.0[1][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0]
            [0][2][1],
        121
    );
}

#[test]
fn ops_and_neg() {
    let a = Tensor2::<i32, 2, 3>::from_fn(|i| (i[0] * 3 + i[1]) as i32);
    let b = Tensor2::<i32, 2, 3>::from_fn(|_| 1);
    assert_eq!((a + b).0, [[1, 2, 3], [4, 5, 6]]);
    assert_eq!((a - b).0, [[-1, 0, 1], [2, 3, 4]]);
    assert_eq!((-a).0, [[0, -1, -2], [-3, -4, -5]]);
    assert_eq!((a * 2).0, [[0, 2, 4], [6, 8, 10]]);
    assert_eq!(a.sum(), 15);
    assert_eq!(a.dot(b), 15);
    assert_eq!(a[1][2], 5);
    let mut c = a;
    c[0][1] = 7;
    assert_eq!(c.0[0], [0, 7, 2]);
    let mut d = a;
    d += b;
    d -= b;
    assert_eq!(d, a);
}

#[test]
fn transpose_trace_matmul() {
    let a = Nd([[1, 2, 3], [4, 5, 6]]); // 2x3
    let t = a.transpose(); // 3x2
    assert_eq!(t.0, [[1, 4], [2, 5], [3, 6]]);
    assert_eq!(t.transpose(), a);

    let b = Nd([[7, 8], [9, 10], [11, 12]]); // 3x2
    let p: Tensor2<i32, 2, 2> = a * b;
    assert_eq!(p.0, [[58, 64], [139, 154]]);
    assert_eq!(p.trace(), 212);
    // (AB)^T = B^T A^T
    assert_eq!(p.transpose(), b.transpose() * a.transpose());
    // Einheitsmatrix
    assert_eq!(a * Tensor2::<i32, 3, 3>::identity(), a);
    assert_eq!(Tensor2::<i32, 2, 2>::identity() * a, a);
    // Matrix * Vektor
    let v = Nd([1, 0, -1]);
    assert_eq!((a * v).0, [-2, -2]);
    // Floats
    let m = Nd([[0.5f64, 0.0], [0.0, 2.0]]);
    assert_eq!((m * m).trace(), 4.25);
}

#[test]
fn swap_and_contract_deeper_axes() {
    let t = Tensor3::<i32, 2, 3, 4>::from_fn(|i| (i[0] * 100 + i[1] * 10 + i[2]) as i32);
    let s: Tensor3<i32, 3, 2, 4> = t.swap_adjacent::<U0>();
    let u: Tensor3<i32, 2, 4, 3> = t.swap_adjacent::<U1>();
    for a in 0..2 {
        for b in 0..3 {
            for c in 0..4 {
                assert_eq!(s.0[b][a][c], t.0[a][b][c]);
                assert_eq!(u.0[a][c][b], t.0[a][b][c]);
            }
        }
    }
    // Permutation (0 1 2) -> (2 1 0) durch drei Nachbarvertauschungen
    let r: Tensor3<i32, 4, 3, 2> = t
        .swap_adjacent::<U0>() // 3,2,4
        .swap_adjacent::<U1>() // 3,4,2
        .swap_adjacent::<U0>(); // 4,3,2
    for a in 0..2 {
        for b in 0..3 {
            for c in 0..4 {
                assert_eq!(r.0[c][b][a], t.0[a][b][c]);
            }
        }
    }
    // Kontraktion: Tensor3<3,3,2> ueber Achsen 0,1 -> Vektor der Laenge 2
    let m = Tensor3::<i32, 3, 3, 2>::from_fn(|i| (i[0] * 10 + i[1] + i[2] * 1000) as i32);
    let c: Tensor1<i32, 2> = m.contract_adjacent::<U0>();
    assert_eq!(c.0[0], 11 + 22);
    assert_eq!(c.0[1], 1000 + 1011 + 1022);
    // Kontraktion tiefer: Achsen 1,2 von Tensor3<2,3,3>
    let n = Tensor3::<i32, 2, 3, 3>::from_fn(|i| (i[0] * 1000 + i[1] * 10 + i[2]) as i32);
    let c: Tensor1<i32, 2> = n.contract_adjacent::<U1>();
    assert_eq!(c.0, [11 + 22, 1000 + 1011 + 1022]);
}

#[test]
fn tensor0_real_conversions() {
    let r = Real(2.5f64);
    let t: Tensor0<f64> = r.into();
    assert_eq!(t.0, 2.5);
    let back: Real<f64> = t.into();
    assert_eq!(back, r);
    assert_eq!(Tensor0::from(3u8), Nd(3u8));
    assert_eq!(
        core::mem::size_of::<Tensor0<u64>>(),
        core::mem::size_of::<Real<u64>>()
    );
    assert_eq!(core::mem::size_of::<Tensor0<u64>>(), 8);
    // Spur einer 1x1-Matrix ist ein Tensor0
    let tr: Tensor0<i32> = Nd([[5]]).contract_adjacent::<U0>();
    assert_eq!(tr, Nd(5));
}

#[test]
fn tensor1_cayley_dickson_conversions() {
    let q = Quaternion::<i32>::from_wxyz(1, 2, 3, 4);
    let v: Tensor1<i32, 4> = q.into();
    assert_eq!(v.0, [1, 2, 3, 4]);
    assert_eq!(Quaternion::<i32>::from(v), q);
    let c: Tensor1<f64, 2> = Complex::from_re_im(1.0, -1.0).into();
    assert_eq!(c.0, [1.0, -1.0]);
    let s = Sedenion::<i8>::basis(9);
    let sv: Tensor1<i8, 16> = s.into();
    assert_eq!(Sedenion::from(sv), s);
    let o: Tensor1<i32, 8> = Octonion::<i32>::one().into();
    assert_eq!(o.0[0], 1);
    let t: Tensor1<i32, 32> = Trigintaduonion::<i32>::basis(31).into();
    assert_eq!(t.0[31], 1);
}

#[test]
fn outer_product_ranks_and_values() {
    // Vektor (x) Vektor = Matrix
    let a = Nd([1, 2, 3]);
    let b = Nd([10, 20]);
    let m: Tensor2<i32, 3, 2> = a.outer(b);
    assert_eq!(m.0, [[10, 20], [20, 40], [30, 60]]);

    // Skalar (x) Tensor = skalierter Tensor, Tensor (x) Skalar ebenso
    let s = Tensor0::from(3);
    assert_eq!(s.outer(b), Nd([30, 60]));
    assert_eq!(b.outer(s), Nd([30, 60]));

    // Matrix (x) Vektor = Rang 3 mit verschiedenen Achsenlaengen
    let t: Tensor3<i32, 3, 2, 3> = m.outer(a);
    for i in 0..3 {
        for j in 0..2 {
            for k in 0..3 {
                assert_eq!(t.0[i][j][k], m.0[i][j] * a.0[k]);
            }
        }
    }
    // Matrix (x) Matrix = Rang 4
    let q: Tensor4<i32, 3, 2, 2, 3> = m.outer(m.transpose());
    assert_eq!(Tensor4::<i32, 3, 2, 2, 3>::RANK, 4);
    assert_eq!(q.0[2][1][1][2], m.0[2][1] * m.0[2][1]);
    // Frobenius-Norm multiplikativ: |a (x) b|^2 = |a|^2 |b|^2
    assert_eq!(m.outer(m).norm_sqr(), m.norm_sqr() * m.norm_sqr());
    // Assoziativ: (a (x) b) (x) c == a (x) (b (x) c)
    assert_eq!(a.outer(b).outer(a), a.outer(b.outer(a)));
    // Bilinear
    let c = Nd([1, 1]);
    assert_eq!(a.outer(b + c), a.outer(b) + a.outer(c));
    assert_eq!((a * 2).outer(b), a.outer(b) * 2);
}

#[test]
fn outer_product_up_to_rank_32() {
    let a = Tensor16::<i32, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2>::from_fn(|i| {
        (i[14] * 2 + i[15] + 1) as i32
    });
    let b = Tensor16::<i32, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3>::from_fn(|i| {
        (i[0] * 3 + i[15] + 1) as i32
    });
    let p = a.outer(b);
    type P = Tensor32<
        i32,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        2,
        2,
        2,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        3,
    >;
    let p: P = p;
    assert_eq!(P::RANK, 32);
    assert_eq!(P::LEN, 24);
    let mut idx = [0usize; 32];
    idx[14] = 1;
    idx[15] = 0;
    idx[16] = 1;
    idx[31] = 2;
    // a[1][0] = 3, b[1][2] = 6
    assert_eq!(*p.get(&idx).unwrap(), 3 * 6);
}

#[test]
fn tensordot_matches_matmul_and_dot() {
    let a = Nd([[1, 2, 3], [4, 5, 6]]);
    let b = Nd([[7, 8], [9, 10], [11, 12]]);
    let p: Tensor2<i32, 2, 2> = a.tensordot(b);
    assert_eq!(p, a * b);
    // Vektor . Vektor = Skalar
    let u = Nd([1, 2, 3]);
    let v = Nd([4, 5, 6]);
    let d: Tensor0<i32> = u.tensordot(v);
    assert_eq!(d.0, u.dot(v));
    // Vektor . Matrix = Zeilenvektor, Matrix . Vektor = Spaltenvektor
    let r: Tensor1<i32, 2> = u.tensordot(b);
    assert_eq!(r.0, [58, 64]);
    let c: Tensor1<i32, 2> = a.tensordot(u);
    assert_eq!(c, a * u);
    // Rang 3 . Rang 2 = Rang 3: letzte Achse von x (3) mit erster von b (3)
    let x = Tensor3::<i32, 2, 2, 3>::from_fn(|i| (i[0] * 6 + i[1] * 3 + i[2]) as i32);
    let y: Tensor3<i32, 2, 2, 2> = x.tensordot(b);
    for i in 0..2 {
        for j in 0..2 {
            for l in 0..2 {
                let want: i32 = (0..3).map(|k| x.0[i][j][k] * b.0[k][l]).sum();
                assert_eq!(y.0[i][j][l], want);
            }
        }
    }
    // Rang 2 . Rang 3 = Rang 3
    let z: Tensor3<i32, 2, 2, 3> = a.tensordot(Tensor3::<i32, 3, 2, 3>::from_fn(|i| {
        (i[0] + i[1] + i[2]) as i32
    }));
    assert_eq!(Tensor3::<i32, 2, 2, 3>::LEN, 12);
    assert_eq!(z.0[1][0][2], 4 * 2 + 5 * 3 + 6 * 4);
    // tensordot == contract_adjacent(outer): Achsen (letzte von a, erste von b)
    let via_outer: Tensor2<i32, 2, 2> = a.outer(b).contract_adjacent::<U1>();
    assert_eq!(via_outer, a.tensordot(b));
}

#[test]
fn sum_axis_hadamard_norm_cross() {
    let t = Tensor3::<i32, 2, 3, 4>::from_fn(|i| (i[0] * 100 + i[1] * 10 + i[2]) as i32);
    let s0: Tensor2<i32, 3, 4> = t.sum_axis::<U0>();
    assert_eq!(s0.0[1][2], 12 + 112);
    let s1: Tensor2<i32, 2, 4> = t.sum_axis::<U1>();
    assert_eq!(s1.0[1][3], 103 + 113 + 123);
    let s2: Tensor2<i32, 2, 3> = t.sum_axis::<U2>();
    assert_eq!(s2.0[1], [406, 446, 486]);
    // Summe ueber alle Achsen == sum()
    let all: Tensor0<i32> = t.sum_axis::<U0>().sum_axis::<U0>().sum_axis::<U0>();
    assert_eq!(all.0, t.sum());

    let a = Nd([1, -2, 3]);
    let b = Nd([4, 5, 6]);
    assert_eq!(a.hadamard(b).0, [4, -10, 18]);
    assert_eq!(a.norm_sqr(), 14);
    let c = a.cross(b);
    assert_eq!(c.0, [-27, 6, 13]);
    assert_eq!(a.dot(c), 0);
    assert_eq!(b.dot(c), 0);
    assert_eq!(a.cross(a), Nd([0, 0, 0]));
    assert_eq!(a.cross(b), -b.cross(a));
}

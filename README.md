# math_traits

Cayley-Dickson-Algebren als `no_std`-Traits ohne `alloc`: `Real<T>`, `Complex<T>`,
`Quaternion<T>`, `Octonion<T>`, `Sedenion<T>`, `Trigintaduonion<T>`.

* Alle Stufen implementieren `CayleyDickson` (+ `TrivialZero`); ab `Sedenion` zusaetzlich `NonTrivialZero`.
* Axiome pro Stufe: Marker-Traits `Commutative`, `Associative`, `Alternative`, `Flexible`,
  `PowerAssociative`, `MultiplicativeNorm`, `DivisionAlgebra` sowie Konstanten in `CayleyDickson`.
* Skalare: `u8..u128`, `usize`, `i8..i128`, `isize`, `f32`, `f64`; `f16`/`f128` mit den
  Nightly-Features `f16` / `f128`. Unsigned-Typen nur als `Real<uN>` (ab `Complex` ist `Neg` noetig).
* `/` fuer `Real<T>` und fuer `Complex`/`Quaternion`/`Octonion` ueber Floats; ab `Sedenion` nur `inverse()`.
* `f8` ist noch nicht in Rust vorhanden; ein Typ kann spaeter einfach `Scalar` (+ `Field`) implementieren.

## Tensoren (`Tensor0` .. `Tensor32`)

* `TensorN<T, D1, .., DN>`: Rang N, jede Achse mit eigener Laenge auf Typ-Ebene, gespeichert als
  verschachtelte Arrays (Stack, kein `alloc`). Alle Aliase zeigen auf `Nd<..>`, das gemeinsame
  Supertrait ist `Tensor` (`+`, `-`, `scale`, `Neg` bei Signed, `get`, `from_fn`, `sum`, `dot`, ...).
* `Tensor0<T>` ist ein eigener `repr(transparent)`-Typ mit `From`/`Into` zu `Real<T>`; `Tensor0` ist
  kein `CayleyDickson` und `Real` kein `Tensor`. `Tensor1<T, 2|4|8|16|32>` ist per `From`/`Into` mit
  `Complex`..`Trigintaduonion` verbunden.
* Achsen vertauschen / kontrahieren: `swap_adjacent::<Uk>()`, `contract_adjacent::<Uk>()`;
  `Tensor2`: `transpose`, `trace`, `identity`, Matrixprodukt via `*` (auch Matrix * Vektor).
* Tensorprodukt: `a.outer(b)` ergibt `Tensor<N+M>` (`(a (x) b)[i.., j..] = a[i..] * b[j..]`), bis Rang 32.
* `a.tensordot(b)` kontrahiert die letzte Achse von `a` mit der ersten von `b` (Rang 1 x 1 = Skalarprodukt,
  Rang 2 x 2 = Matrixprodukt); `t.sum_axis::<Uk>()` summiert eine Achse weg.
* Zusaetzlich: `hadamard`, `norm_sqr`, `dot`, `sum` (alle Raenge) und `cross` (`Tensor1<T, 3>`).
* `tools/gen_ranks.py` erzeugt `src/tensor/ranks.rs`.

## Clifford-Algebren (`Cl(p, q, r)`)

* `Clifford<T, P, Q, R, N>`: volle Signatur im Typ (`P` Generatoren mit `e^2 = +1`, `Q` mit `-1`, `R` mit `0`),
  `N = 2^(P+Q+R)` Koeffizienten; die Laenge wird zur Compile-Zeit geprueft. Nur vorzeichenbehaftete Skalare.
* Generierte Aliase (`tools/gen_clifford.py`, dort erweiterbar): `ClComplex`, `ClQuaternion`, `SplitComplex`,
  `DualNumber`, `Cl2`, `Cl3`, `Cl4`, `Sta` (Cl(1,3)), `Cl31`, `Pga2`, `Pga3`, `Cga3`, `Grassmann2..4`.
* Geometrisches Produkt `*`, `wedge`, `left_contract`, `right_contract`, `scalar_product`, `reverse`,
  `grade_involution`, `clifford_conjugate`, `grade_part`, `dual`, `sandwich`, `inverse` (Vektoren, Blades, Versoren).
* Immer assoziativ/alternativ/flexibel/potenzassoziativ; Nullteiler ausser bei `Cl(0,0)`, `Cl(0,1)`, `Cl(0,2)`
  (`try_zero_divisor_pair`, `NonTrivialZero` fuer die benannten Algebren). `Cl(0,1)`/`Cl(0,2)` sind per `From`
  mit `Complex`/`Quaternion` verbunden. Oktonionen und hoeher sind keine Clifford-Algebren.
* Alle Algebren (Cayley-Dickson und Clifford) teilen sich das Supertrait `Algebra` und die Marker-Traits.

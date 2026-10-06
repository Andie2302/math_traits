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
* `tools/gen_ranks.py` erzeugt `src/tensor/ranks.rs`.

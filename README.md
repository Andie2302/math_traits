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

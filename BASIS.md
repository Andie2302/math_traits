# Wann ist die Basis abgeschlossen?

Eine Basis ist nie *endgültig*, denn man kann Gesetze immer weiter zerlegen.
Dank `laws!` ist das Zerlegen nach unten aber jederzeit ohne Bruch möglich.
Deshalb muss die Basis nicht *vollständig* sein, sondern nur **ausreichend für
die Ziele**. Der Abschluss ist also keine Gefühlsfrage, sondern lässt sich
prüfen.

## Kriterium

Die Basis ist für eine Version abgeschlossen, wenn alle drei Punkte gelten:

1. **Zielstrukturen:** Jede Struktur im Zielkatalog (`tests/targets.rs`) ist
   als leerer Alias über Gesetze ausdrückbar.
2. **Zielalgorithmen:** Jeder Algorithmus im Katalog kompiliert *nur* mit
   Bounds aus `structures` (plus `Clone`/`PartialEq`), ohne zusätzliche
   Annahmen im Doc-Kommentar.
3. **Signatur stabil:** Für die letzten Ziele musste nichts mehr in
   `signature.rs` ergänzt werden.

Punkt 3 ist das eigentliche Signal: Solange neue Ziele neue *Signatur*
brauchen, ist die Basis noch nicht fertig. Brauchen sie nur noch neue
*Gesetze* oder *Strukturen*, kannst du zu den Implementierungen wechseln.
Gesetze lassen sich später billig ergänzen, die Signatur nicht.

**Warnsignal in die andere Richtung:** Wenn du ein Gesetz nur deshalb
zerlegen willst, weil es zerlegbar *ist*, und kein Typ und kein Algorithmus
den Unterschied braucht, dann warte damit. Das Makro macht das Zerlegen
jederzeit möglich.

## Stand

### Bausteine

| Ziel | Status | Was dafür nötig war |
|---|---|---|
| Monoid, Gruppe, Ring, Körper, Verband, Ordnung | erreicht | |
| Geordneter Körper (`f64`, `f32`) | erreicht | Gesetz `PositiveProduct` |
| Euklidischer Ring, `gcd` | erreicht | **Signatur** `HasDivRem`, `HasEuclideanSize` |
| Modul, Vektorraum (`[T; N]`) | erreicht | **Signatur** `ScalarMul<S>` |
| ℂ, ℍ, 𝕆, 𝕊 (Cayley-Dickson) | erreicht | **Signatur** `HasConjugate`; Struktur `NonAssociativeRing` |
| Bisektion, Gauß, Newton (n-dim.) | erreicht | `OrderedField` |
| Elementarfunktionen (`RealField`, `ElementaryRing`) | erreicht | **Signatur** `HasSqrt`, `HasExp`, `HasLn`, `HasSinCos`; nachträglich `HasAtan2` (für IAPWS-06 entdeckt) |
| AutoDiff vorwärts, 1. und 2. Ableitung, Jacobi, Hesse-Matrix (N Variablen) | erreicht | duale Zahlen; Gesetz `InverseWhereDefined` |
| Skalarprodukt, Norm als Folgerung | erreicht | **Signatur** `InnerProduct<S>`; Gesetze `ConjugateAdditive`, `SelfConjugate` |
| CG-Löser, RK4, Velocity-Verlet, Gravitation | erreicht | `EuclideanSpace` / `Module` |
| Einheitswurzeln, FFT 1D/2D/3D, Faltung | erreicht | **Signatur** `HasRootsOfUnity` |
| Algebra (Skalare vertauschen mit Produkt) | erreicht | Gesetz `OpHomogeneous` |
| Tensoren Stufe 1–4, Matrix·Matrix, Matrix·Vektor, ⊗ | erreicht | **Signatur** `Contract<B>`, `Outer<B>` (Typen `A × B → C`) |
| Quadratische Matrizen als Ring / Algebra | erreicht | nur Gesetze |
| Determinante, LU, Inverse, QR, Eigenwerte (symmetrisch) | erreicht | `Field` / `OrderedField` / `RealField` |
| Lie-Algebren (so(3), gl(n)), Drehungen per `exp` | erreicht | Marker `Bracket`; Gesetze `Alternating`, `Jacobi`; **Signatur** `ExpMap<G>` |
| Dynamischer Tensor (`alloc`) | erreicht | **Signatur** `TryBinaryOp`, `TryContract` (partielle Operationen) |
| `no_std` mit `alloc`/`libm` | erreicht | nur Infrastruktur |

### Projekte

| Projekt | Basis bereit? | Fehlt noch |
|---|---|---|
| IAPWS-95 | **erledigt** | Crate `iapws`: Einphasen- und Zweiphasengebiet, alle offiziellen Prüfwerte (Tab. 6, 7, 8) auf 9 Stellen |
| IAPWS-06 (Eis Ih) | **erledigt** | komplexes Gibbs-Potential, Tabelle 6 auf 9 Stellen, Schmelzkurve aus IAPWS-06 + IAPWS-95 |
| IAPWS-08 (Meerwasser) | **erledigt** | Salz-Anteil maschinell aus TEOS-10/GSW-C übernommen; AutoDiff-Ableitung gleich der handgeschriebenen; Standardozean, Gefrier- und Siedepunkt |
| IAPWS-10 (feuchte Luft) mit trockener Luft (Lemmon 2000) | **erledigt** | Tabellen 7, 13, 14, 15 auf 9 Stellen; alle Ableitungen nach (A, T, ρ) per `hessian` statt 30 Handformeln |
| Psychrometrie (rel. Feuchte, Tau-/Reifpunkt, Feuchtkugel/Eiskugel) | **erledigt** | streng über chemische Potentiale (IAPWS-10 + 95 + 06); Sättigung gleich ASHRAE auf ≤ 0.1 % |
| Droste-Effekt („Logarithmus eines Bildes“) | **erledigt** | Crate `droste`: Testbild, gerade und spiralige Verschachtelung, nahtlos |
| 2-/3-Körper-Problem | **ja** | erledigt bis auf Animation (Grafik-Crate) |
| Navier-Stokes | **ja** | Gitter, Operatoren (Implementierung) |
| FFT 1D/2D/3D | **ja** | erledigt |
| Neuronale Netze | **ja** | Layer, Rückwärts-AutoDiff, Adam (Implementierung) |

**Kein Projekt der Liste braucht mehr eine neue Signatur.** Nach dem
Kriterium oben ist die Basis damit für diese Ziele abgeschlossen. Ab hier geht
es um Implementierungen.

Was bewusst **nicht** in die Basis gehört: Gitter und Berechnungsgraphen
(Datentypen, die Strukturen *erfüllen*), Zufall
(`rand`-Crate), physikalische Einheiten (z. B. `uom`) und die Vollständigkeit
von ℝ (nur für Beweise nötig).

## Grenzen der Signatur

Die Signatur kennt jetzt diese Formen:

| Form | Beispiele |
|---|---|
| `Self × Self → Self` | `+`, `·`, `∧`, `∨`, Lie-Klammer |
| `S × V → V` | Skalar mal Vektor, Quaternion dreht Vektor |
| `V × V → S` | Skalarprodukt |
| `A × B → C` | Kontraktion, Tensorprodukt |
| `A → B` | Exponentialabbildung Lie-Algebra → Gruppe |
| partiell (`Option`) | Inverse, `ln`, `sqrt`, Operationen auf dynamischen Tensoren |

Was weiterhin **nicht** hineinpasst:

1. **Unendliche Objekte:** Grenzwerte, exakte reelle Zahlen, Potenzreihen,
   Maße und Wahrscheinlichkeit (σ-Algebren), Topologie, Mannigfaltigkeiten
   mit Tangentialräumen, die von Punkt zu Punkt variieren. Im Code arbeitet
   man mit Näherungen, die wieder in die bestehende Signatur passen.
2. **Grenzen von Rust selbst:** Strukturen über Containern (`Vec<T> →
   Vec<U>`, Funktoren, Monaden) brauchen Typkonstruktoren als Parameter.
   Die gibt es in Rust nur eingeschränkt (GATs).
3. **Abbildungen als Objekte mit Gesetzen** (lineare Abbildungen,
   Homomorphismen als eigene Typen): Matrizen decken den linearen Fall ab.
   Für allgemeine Abbildungen bleibt es bei Closures, und Closures kann man
   keine Gesetze deklarieren.

## Bekannte Lücke: Folgerungen zwischen Gesetzen

Mathematisch folgt aus `associative`, dass auch `alternative` und `flexible`
gelten. Das System leitet solche Sätze nicht selbst ab. Man muss sie
mitdeklarieren (siehe ℂ und ℍ in `impls/cayley_dickson.rs`). Ein
Blanket-Impl (`Associative ⇒ Flexible`) geht nicht, weil es mit den
Makro-Impls kollidiert.

Das ist eine Frage der Bequemlichkeit, nicht der Basis. Lösen ließe sie sich
im Makro: Zusammengesetzte Schlüsselwörter können ihre Folgerungen gleich
mitdeklarieren, z. B. könnte `associative` künftig auch `flexible` und
`alternative` erzeugen.

## Gleitkomma

`f64`/`f32` sind als **Modell** von `ℝ` deklariert: Die Gesetze gelten bis
auf Rundung und nur für endliche Werte. Exakte Prüfungen laufen deshalb über
Ganzzahlen (Cayley-Dickson über `i64`) und endliche Körper (`GF(p)`, `bool`).

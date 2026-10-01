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
| Elementarfunktionen (`RealField`, `ElementaryRing`) | erreicht | **Signatur** `HasSqrt`, `HasExp`, `HasLn`, `HasSinCos` |
| AutoDiff vorwärts, 1. und 2. Ableitung, Jacobi | erreicht | duale Zahlen; Gesetz `InverseWhereDefined` |
| Skalarprodukt, Norm als Folgerung | erreicht | **Signatur** `InnerProduct<S>`; Gesetze `ConjugateAdditive`, `SelfConjugate` |
| CG-Löser, RK4, Velocity-Verlet, Gravitation | erreicht | `EuclideanSpace` / `Module` |
| Einheitswurzeln (FFT) | **offen (Schritt 3)** | **Signatur** primitive n-te Einheitswurzel |
| Algebra (Skalare vertauschen mit Produkt) | offen | nur ein Gesetz |

### Projekte

| Projekt | Basis bereit? | Fehlt noch |
|---|---|---|
| IAPWS-95/06/10, trockene Luft | **ja** | nur Implementierung (Formeln, Koeffizienten) |
| Droste-Effekt („Logarithmus eines Bildes“) | **ja** | nur Implementierung (Bild-Abtastung) |
| 2-/3-Körper-Problem | **ja** | erledigt bis auf Animation (Grafik-Crate) |
| Navier-Stokes | **ja**, für spektrale Verfahren nach Schritt 3 | Gitter, Operatoren (Implementierung) |
| FFT 1D/2D/3D | nein | Schritt 3 |
| Neuronale Netze | nach Schritt 3 | Rückwärts-AutoDiff (Implementierung); siehe „Grenzen“ |

Es bleibt **eine Signatur-Erweiterung**: Einheitswurzeln. Danach braucht kein Projekt der Liste mehr eine neue
Signatur. Das ist der Punkt, an dem du zu den Implementierungen wechselst.

Was bewusst **nicht** in die Basis gehört: Tensoren, Matrizen, Gitter und
Berechnungsgraphen (Datentypen, die Strukturen *erfüllen*), Zufall
(`rand`-Crate), physikalische Einheiten (z. B. `uom`) und die Vollständigkeit
von ℝ (nur für Beweise nötig).

## Grenzen der Signatur

Alle Signaturen bisher haben eine von drei Formen: `Self × Self → Self`
(Operationen), `S × V → V` (externe Operation) und `V × V → S`
(Skalarprodukt). Dazu kommen Konstanten und partielle Funktionen per
`Option`. Folgendes passt in **keine** dieser Formen:

1. **Operationen zwischen drei verschiedenen Typen** `A × B → C`, etwa
   Matrix `m×n` mal Matrix `n×k` ergibt `m×k`, oder Tensor-Kontraktionen.
   Das betrifft *neuronale Netze*, sobald man Layer generisch verketten will.
   Lösung: eine heterogene Signatur `Compose<B, Output = C>` plus
   Gesetze für die Assoziativität über drei Typen (mathematisch: eine
   *Kategorie*).
2. **Abbildungen zwischen Strukturen** als eigene Objekte: lineare
   Abbildungen, Homomorphismen, `exp` von einer Lie-Algebra in eine Gruppe
   (z. B. Rotationsvektor → Quaternion). Bisher sind Abbildungen nur
   Closures, und Closures kann man keine Gesetze deklarieren.
3. **Formen, die erst zur Laufzeit feststehen:** ein `Vec`-Tensor, dessen
   Dimension erst beim Lesen einer Datei klar ist. Dann kann auch `+`
   scheitern (verschiedene Formen), und `BinaryOp` ist total. Solange die
   Größen als `const N` im Typ stehen, gibt es das Problem nicht.
4. **Unendliche Objekte:** Grenzwerte, exakte reelle Zahlen, Potenzreihen,
   Maße und Wahrscheinlichkeit (σ-Algebren), Topologie (offene Mengen),
   Mannigfaltigkeiten mit Tangentialräumen, die von Punkt zu Punkt variieren.
   Das sprengt nicht nur die Signatur, sondern endliche Darstellung
   überhaupt. Im Code arbeitet man dann mit Näherungen, die wieder in die
   bestehende Signatur passen.
5. **Grenzen von Rust selbst:** Strukturen über Containern (`Vec<T> →
   Vec<U>`, Funktoren, Monaden) brauchen Typkonstruktoren als Parameter.
   Das gibt es in Rust nur eingeschränkt (GATs).

Von deiner Projektliste berührt nur Punkt 1 etwas: neuronale Netze mit
verschieden großen Layern. Dort reicht aber zunächst eine konkrete
Matrix-Implementierung. Die Signatur braucht man erst, wenn man *generische*
Algorithmen über beliebige Layer-Typen schreiben will.

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

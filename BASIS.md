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
   Bounds aus `structures`, ohne zusätzliche Trait-Bounds oder Annahmen im
   Doc-Kommentar.
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

| Ziel | Status | Was fehlt |
|---|---|---|
| Monoid, Gruppe, abelsche Gruppe | erreicht | |
| Ring, kommutativer Ring, Integritätsbereich | erreicht | |
| Schiefkörper, Körper | erreicht | |
| Verband, Totalordnung | erreicht | |
| `pow`, `pow_signed`, `sum`, `product`, `sub`, `try_div` | erreicht | |
| Geordneter Körper | offen | Gesetz (Verträglichkeit von `·` mit `≤`) |
| Euklidischer Ring, `gcd` | offen | **Signatur**: Division mit Rest |
| Modul, Vektorraum | offen | **Signatur**: externe Operation `S × V → V` |
| Normierter Raum | offen | nur Strukturen/Gesetze auf Basis der beiden vorigen |

Es bleiben also **zwei Signatur-Erweiterungen** (Division mit Rest und die
externe Operation). Danach ist die Basis nach diesem Kriterium abgeschlossen,
und der Schwerpunkt wechselt zu Implementierungen (Typen, Algorithmen,
Ergonomie).

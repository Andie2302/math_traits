#!/usr/bin/env python3
"""Erzeugt src/clifford/named.rs: benannte Aliase fuer haeufig genutzte Clifford-Algebren.

Eintrag: (Name, p, q, r, Beschreibung). N = 2^(p+q+r) wird berechnet.
Neue Algebren: hier eintragen und das Skript erneut ausfuehren.
"""
import sys

# Konvention: p Generatoren mit e^2 = +1, q mit e^2 = -1, r mit e^2 = 0.
ALGEBRAS = [
    ("ClComplex", 0, 1, 0, "Cl(0,1) ~ komplexe Zahlen"),
    ("ClQuaternion", 0, 2, 0, "Cl(0,2) ~ Quaternionen"),
    ("SplitComplex", 1, 0, 0, "Cl(1,0): hyperbolische (split-komplexe) Zahlen, `e^2 = +1`"),
    ("DualNumber", 0, 0, 1, "Cl(0,0,1): duale Zahlen, `e^2 = 0`"),
    ("Cl2", 2, 0, 0, "Cl(2,0): Geometrische Algebra der Ebene"),
    ("Cl3", 3, 0, 0, "Cl(3,0): Geometrische Algebra des 3D-Raums (Pauli-Algebra)"),
    ("Cl4", 4, 0, 0, "Cl(4,0): Euklidische Geometrische Algebra in 4D"),
    ("Sta", 1, 3, 0, "Cl(1,3): Raumzeit-Algebra (STA), Signatur (+,-,-,-)"),
    ("Cl31", 3, 1, 0, "Cl(3,1): Minkowski-Algebra, Signatur (+,+,+,-)"),
    ("Pga2", 2, 0, 1, "Cl(2,0,1): projektive Geometrische Algebra der Ebene"),
    ("Pga3", 3, 0, 1, "Cl(3,0,1): projektive Geometrische Algebra des 3D-Raums"),
    ("Cga3", 4, 1, 0, "Cl(4,1): konforme Geometrische Algebra des 3D-Raums"),
    ("Grassmann2", 0, 0, 2, "Cl(0,0,2): aeussere (Grassmann-)Algebra in 2D"),
    ("Grassmann3", 0, 0, 3, "Cl(0,0,3): aeussere (Grassmann-)Algebra in 3D"),
    ("Grassmann4", 0, 0, 4, "Cl(0,0,4): aeussere (Grassmann-)Algebra in 4D"),
]


def has_zero_divisors(p, q, r):
    return not (p == 0 and r == 0 and q <= 2)


out = [
    "// GENERIERT von tools/gen_clifford.py -- nicht von Hand aendern.",
    "use super::Clifford;",
    "use crate::scalar::Signed;",
    "use crate::traits::NonTrivialZero;",
    "",
]
for name, p, q, r, doc in ALGEBRAS:
    n = 1 << (p + q + r)
    out.append(f"/// {doc} (Dimension {n}).")
    out.append(f"pub type {name}<T> = Clifford<T, {p}, {q}, {r}, {n}>;")
    out.append("")
    if has_zero_divisors(p, q, r):
        out.append(f"impl<T: Signed> NonTrivialZero for {name}<T> {{")
        out.append("    fn zero_divisor_pair() -> (Self, Self) {")
        out.append("        Self::try_zero_divisor_pair().expect(\"Algebra mit Nullteilern\")")
        out.append("    }")
        out.append("}")
        out.append("")
open(sys.argv[1] if len(sys.argv) > 1 else "src/clifford/named.rs", "w").write("\n".join(out))

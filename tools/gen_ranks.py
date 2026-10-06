#!/usr/bin/env python3
"""Erzeugt src/tensor/ranks.rs (Tensor0 .. Tensor32 als Aliase auf Nd<...>)."""
import sys

out = ["// GENERIERT von tools/gen_ranks.py -- nicht von Hand aendern.", "use super::Nd;", ""]
for n in range(33):
    consts = "".join(f", const D{i}: usize" for i in range(1, n + 1))
    ty = "T"
    for i in range(n, 0, -1):
        ty = f"[{ty}; D{i}]"
    if n == 0:
        doc = "Rang 0: Skalar (`repr(transparent)`, mit `From`/`Into` zu `Real<T>`)."
    elif n == 1:
        doc = "Rang 1: Vektor mit `D1` Eintraegen."
    else:
        dims = ", ".join(f"`D{i}`" for i in range(1, n + 1))
        doc = f"Rang {n}: Achsenlaengen {dims}; Speicher `{ty.replace('T', 'T', 1)}`."
    out.append(f"/// {doc}")
    out.append(f"pub type Tensor{n}<T{consts}> = Nd<{ty}>;")
    out.append("")
open(sys.argv[1] if len(sys.argv) > 1 else "src/tensor/ranks.rs", "w").write("\n".join(out))

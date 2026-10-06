// GENERIERT von tools/gen_ranks.py -- nicht von Hand aendern.
use super::Nd;

/// Rang 0: Skalar (`repr(transparent)`, mit `From`/`Into` zu `Real<T>`).
pub type Tensor0<T> = Nd<T>;

/// Rang 1: Vektor mit `D1` Eintraegen.
pub type Tensor1<T, const D1: usize> = Nd<[T; D1]>;

/// Rang 2: Achsenlaengen `D1`, `D2`; Speicher `[[T; D2]; D1]`.
pub type Tensor2<T, const D1: usize, const D2: usize> = Nd<[[T; D2]; D1]>;

/// Rang 3: Achsenlaengen `D1`, `D2`, `D3`; Speicher `[[[T; D3]; D2]; D1]`.
pub type Tensor3<T, const D1: usize, const D2: usize, const D3: usize> = Nd<[[[T; D3]; D2]; D1]>;

/// Rang 4: Achsenlaengen `D1`, `D2`, `D3`, `D4`; Speicher `[[[[T; D4]; D3]; D2]; D1]`.
pub type Tensor4<T, const D1: usize, const D2: usize, const D3: usize, const D4: usize> =
    Nd<[[[[T; D4]; D3]; D2]; D1]>;

/// Rang 5: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`; Speicher `[[[[[T; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor5<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
> = Nd<[[[[[T; D5]; D4]; D3]; D2]; D1]>;

/// Rang 6: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`; Speicher `[[[[[[T; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor6<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
> = Nd<[[[[[[T; D6]; D5]; D4]; D3]; D2]; D1]>;

/// Rang 7: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`; Speicher `[[[[[[[T; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor7<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
> = Nd<[[[[[[[T; D7]; D6]; D5]; D4]; D3]; D2]; D1]>;

/// Rang 8: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`; Speicher `[[[[[[[[T; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor8<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
> = Nd<[[[[[[[[T; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]>;

/// Rang 9: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`; Speicher `[[[[[[[[[T; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor9<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
> = Nd<[[[[[[[[[T; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]>;

/// Rang 10: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`; Speicher `[[[[[[[[[[T; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor10<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
> = Nd<[[[[[[[[[[T; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]>;

/// Rang 11: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`; Speicher `[[[[[[[[[[[T; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor11<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
> = Nd<[[[[[[[[[[[T; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]>;

/// Rang 12: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`; Speicher `[[[[[[[[[[[[T; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor12<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
> = Nd<[[[[[[[[[[[[T; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]>;

/// Rang 13: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`; Speicher `[[[[[[[[[[[[[T; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor13<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
> = Nd<[[[[[[[[[[[[[T; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]>;

/// Rang 14: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`; Speicher `[[[[[[[[[[[[[[T; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor14<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
> = Nd<[[[[[[[[[[[[[[T; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]>;

/// Rang 15: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`; Speicher `[[[[[[[[[[[[[[[T; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor15<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
> = Nd<
    [[[[[[[[[[[[[[[T; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2];
        D1],
>;

/// Rang 16: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`; Speicher `[[[[[[[[[[[[[[[[T; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor16<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
> = Nd<
    [[[[[[[[[[[[[[[[T; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3];
        D2]; D1],
>;

/// Rang 17: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`; Speicher `[[[[[[[[[[[[[[[[[T; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor17<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[T; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4];
        D3]; D2]; D1],
>;

/// Rang 18: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`; Speicher `[[[[[[[[[[[[[[[[[[T; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor18<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[T; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5];
        D4]; D3]; D2]; D1],
>;

/// Rang 19: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`; Speicher `[[[[[[[[[[[[[[[[[[[T; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor19<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[T; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7];
        D6]; D5]; D4]; D3]; D2]; D1],
>;

/// Rang 20: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`; Speicher `[[[[[[[[[[[[[[[[[[[[T; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor20<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[T; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8];
        D7]; D6]; D5]; D4]; D3]; D2]; D1],
>;

/// Rang 21: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`; Speicher `[[[[[[[[[[[[[[[[[[[[[T; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor21<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[T; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9];
        D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1],
>;

/// Rang 22: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`, `D22`; Speicher `[[[[[[[[[[[[[[[[[[[[[[T; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor22<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
    const D22: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[[T; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10];
        D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1],
>;

/// Rang 23: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`, `D22`, `D23`; Speicher `[[[[[[[[[[[[[[[[[[[[[[[T; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor23<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
    const D22: usize,
    const D23: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[[[T; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11];
        D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1],
>;

/// Rang 24: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`, `D22`, `D23`, `D24`; Speicher `[[[[[[[[[[[[[[[[[[[[[[[[T; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor24<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
    const D22: usize,
    const D23: usize,
    const D24: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[[[[T; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12];
        D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1],
>;

/// Rang 25: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`, `D22`, `D23`, `D24`, `D25`; Speicher `[[[[[[[[[[[[[[[[[[[[[[[[[T; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor25<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
    const D22: usize,
    const D23: usize,
    const D24: usize,
    const D25: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[[[[[T; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13];
        D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1],
>;

/// Rang 26: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`, `D22`, `D23`, `D24`, `D25`, `D26`; Speicher `[[[[[[[[[[[[[[[[[[[[[[[[[[T; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor26<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
    const D22: usize,
    const D23: usize,
    const D24: usize,
    const D25: usize,
    const D26: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[[[[[[T; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14];
        D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1],
>;

/// Rang 27: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`, `D22`, `D23`, `D24`, `D25`, `D26`, `D27`; Speicher `[[[[[[[[[[[[[[[[[[[[[[[[[[[T; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor27<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
    const D22: usize,
    const D23: usize,
    const D24: usize,
    const D25: usize,
    const D26: usize,
    const D27: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[[[[[[[T; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15];
        D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1],
>;

/// Rang 28: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`, `D22`, `D23`, `D24`, `D25`, `D26`, `D27`, `D28`; Speicher `[[[[[[[[[[[[[[[[[[[[[[[[[[[[T; D28]; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor28<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
    const D22: usize,
    const D23: usize,
    const D24: usize,
    const D25: usize,
    const D26: usize,
    const D27: usize,
    const D28: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[[[[[[[[T; D28]; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16];
        D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1],
>;

/// Rang 29: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`, `D22`, `D23`, `D24`, `D25`, `D26`, `D27`, `D28`, `D29`; Speicher `[[[[[[[[[[[[[[[[[[[[[[[[[[[[[T; D29]; D28]; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor29<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
    const D22: usize,
    const D23: usize,
    const D24: usize,
    const D25: usize,
    const D26: usize,
    const D27: usize,
    const D28: usize,
    const D29: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[[[[[[[[[T; D29]; D28]; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17];
        D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1],
>;

/// Rang 30: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`, `D22`, `D23`, `D24`, `D25`, `D26`, `D27`, `D28`, `D29`, `D30`; Speicher `[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[T; D30]; D29]; D28]; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor30<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
    const D22: usize,
    const D23: usize,
    const D24: usize,
    const D25: usize,
    const D26: usize,
    const D27: usize,
    const D28: usize,
    const D29: usize,
    const D30: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[[[[[[[[[[T; D30]; D29]; D28]; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18];
        D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2];
        D1],
>;

/// Rang 31: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`, `D22`, `D23`, `D24`, `D25`, `D26`, `D27`, `D28`, `D29`, `D30`, `D31`; Speicher `[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[T; D31]; D30]; D29]; D28]; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor31<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
    const D22: usize,
    const D23: usize,
    const D24: usize,
    const D25: usize,
    const D26: usize,
    const D27: usize,
    const D28: usize,
    const D29: usize,
    const D30: usize,
    const D31: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[T; D31]; D30]; D29]; D28]; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19];
        D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4];
        D3]; D2]; D1],
>;

/// Rang 32: Achsenlaengen `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D16`, `D17`, `D18`, `D19`, `D20`, `D21`, `D22`, `D23`, `D24`, `D25`, `D26`, `D27`, `D28`, `D29`, `D30`, `D31`, `D32`; Speicher `[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[T; D32]; D31]; D30]; D29]; D28]; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20]; D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5]; D4]; D3]; D2]; D1]`.
pub type Tensor32<
    T,
    const D1: usize,
    const D2: usize,
    const D3: usize,
    const D4: usize,
    const D5: usize,
    const D6: usize,
    const D7: usize,
    const D8: usize,
    const D9: usize,
    const D10: usize,
    const D11: usize,
    const D12: usize,
    const D13: usize,
    const D14: usize,
    const D15: usize,
    const D16: usize,
    const D17: usize,
    const D18: usize,
    const D19: usize,
    const D20: usize,
    const D21: usize,
    const D22: usize,
    const D23: usize,
    const D24: usize,
    const D25: usize,
    const D26: usize,
    const D27: usize,
    const D28: usize,
    const D29: usize,
    const D30: usize,
    const D31: usize,
    const D32: usize,
> = Nd<
    [[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[T; D32]; D31]; D30]; D29]; D28]; D27]; D26]; D25]; D24]; D23]; D22]; D21]; D20];
        D19]; D18]; D17]; D16]; D15]; D14]; D13]; D12]; D11]; D10]; D9]; D8]; D7]; D6]; D5];
        D4]; D3]; D2]; D1],
>;

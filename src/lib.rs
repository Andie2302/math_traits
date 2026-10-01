//! # math_traits
//!
//! Algebraische Strukturen als Rust-Traits, aufgebaut in drei strikt
//! getrennten Schichten:
//!
//! | Schicht | Modul | Inhalt | Wer implementiert? |
//! |---|---|---|---|
//! | **Signatur** | [`signature`] | Operationen, ausgezeichnete Elemente, Relationen | der Nutzer, per `impl` |
//! | **Gesetze** | [`laws`] | genau *eine* Aussage über die Signatur | der Nutzer, aber nur über [`laws!`] |
//! | **Strukturen** | [`structures`] | reine Konjunktionen von Gesetzen | niemand, nur Blanket-Impls |
//!
//! ## Die zwei Regeln
//!
//! 1. **Nach oben entsteht nichts Neues.** Jede Struktur ist ein leerer Trait,
//!    der automatisch für jeden Typ gilt, der alle ihre Gesetze erfüllt. Eine
//!    Struktur kann keine eigenen Methoden oder Annahmen einschmuggeln.
//! 2. **Nach unten bleibt alles offen.** Nutzer schreiben nie
//!    `impl Associative<…> for …`, sondern deklarieren Gesetze über das
//!    [`laws!`]-Makro mit *Schlüsselwörtern*. Teilt man ein Gesetz später in
//!    feinere Gesetze auf, ändert sich nur die Zuordnung im Makro. Nutzercode
//!    und alle Strukturen darüber bleiben unverändert.
//!
//! ## Beispiel
//!
//! ```
//! use math_traits::laws;
//! use math_traits::signature::{Additive, BinaryOp, HasIdentity, HasInverse};
//! use math_traits::structures::AbelianGroup;
//!
//! #[derive(Clone, Copy, PartialEq, Debug)]
//! struct Z5(u8);
//!
//! impl BinaryOp<Additive> for Z5 {
//!     fn op(&self, rhs: &Self) -> Self { Z5((self.0 + rhs.0) % 5) }
//! }
//! impl HasIdentity<Additive> for Z5 {
//!     fn identity() -> Self { Z5(0) }
//! }
//! impl HasInverse<Additive> for Z5 {
//!     fn inverse(&self) -> Self { Z5((5 - self.0) % 5) }
//! }
//!
//! laws! {
//!     Z5 {
//!         Additive: associative, commutative, identity, inverse;
//!     }
//! }
//!
//! fn needs_group<T: AbelianGroup<Additive>>() {}
//! needs_group::<Z5>();
//! ```

pub mod autodiff;
pub mod derived;
pub mod geometry;
pub mod impls;
pub mod laws;
mod macros;
pub mod signature;
pub mod solve;
pub mod structures;

/// Interna für das [`laws!`]-Makro. Nicht direkt verwenden.
#[doc(hidden)]
pub mod __private {
    /// Siegel: Nur wer diesen Typ nennt, kann ein Gesetz implementieren.
    /// Das Makro tut das; handgeschriebene `impl`s sollen es nicht.
    pub struct Token;
}

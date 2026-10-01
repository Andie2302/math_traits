//! Thermodynamische Zustandsgleichungen der IAPWS, aufgebaut auf `math_traits`.
//!
//! * [`iapws95`]: gewöhnliches Wasser (IAPWS-95, Revision 2018)
//!
//! Geplant: IAPWS-06 (Eis Ih), IAPWS-08/10 (Meerwasser), trockene Luft.
//! Alle folgen demselben Muster: ein thermodynamisches Potential, generisch
//! über [`math_traits::num::Real`] geschrieben, und Ableitungen per AutoDiff.

pub mod iapws95;

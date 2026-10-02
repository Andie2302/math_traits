//! Thermodynamische Zustandsgleichungen der IAPWS, aufgebaut auf `math_traits`.
//!
//! * [`iapws95`]: gewöhnliches Wasser (IAPWS-95, Revision 2018)
//! * [`iapws06`]: Eis Ih (IAPWS-06, Revision 2009), inkl. Schmelzkurve
//!
//! Geplant: IAPWS-08/10 (Meerwasser), trockene Luft.
//! Alle folgen demselben Muster: ein thermodynamisches Potential, generisch
//! über [`math_traits::num::Real`] geschrieben, und Ableitungen per AutoDiff.

pub mod iapws06;
pub mod iapws95;

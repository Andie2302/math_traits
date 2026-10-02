//! Thermodynamische Zustandsgleichungen der IAPWS, aufgebaut auf `math_traits`.
//!
//! * [`iapws95`]: gewöhnliches Wasser (IAPWS-95, Revision 2018)
//! * [`iapws06`]: Eis Ih (IAPWS-06, Revision 2009), inkl. Schmelzkurve
//! * [`iapws08`]: Meerwasser (IAPWS-08 / TEOS-10), inkl. Gefrier- und Siedepunkt
//!
//! Geplant: trockene Luft und feuchte Luft (IAPWS-10).
//! Alle folgen demselben Muster: ein thermodynamisches Potential, generisch
//! über [`math_traits::num::Real`] geschrieben, und Ableitungen per AutoDiff.

pub mod iapws06;
pub mod iapws08;
pub mod iapws95;

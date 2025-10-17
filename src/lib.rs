//! The purpose of this crate is to provide fission yield data 
//! in Rust
//!
//! Fission yield data is generally energy dependent, and several nuclides 
//! can fission, thus resulting in several radioisotopes
//!
//! Fission yields can be separated first by:
//!
//! 1. Incoming Neutron Energy 
//! 2. Fissioning Nuclide 
//! 3. Resulting Nuclides
//!
//! Of course, we can separate fission yields by library as well
//! In the ENDF library, it is usually MT=454
//!
//! I used the ENDF/B-VIII.0 library from JANIS to make this library
//! There are two kinds of fission yields.
//!
//! First is the independent fission yields, 
//! https://wwwndc.jaea.go.jp/FAQ/FAQ/a209_e.html
//!
//! and then the cumulative fission yields.
//! Parent independent fission yields are yields prior to decay, 
//! whereas cumulative fission yields are yields after decay.
//!
//! For this library we use independent fission yields
//!
//! 


#[warn(missing_docs)]

/// provides endf 8 parent independent fission yields,
/// data taken from ENDF/B-VIII using JANIS
pub(crate) mod endf_8_parent_independent_yields;

pub mod prelude;

#[cfg(test)]
pub(crate) mod tutorials_and_tests;

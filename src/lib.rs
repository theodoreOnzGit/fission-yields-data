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
//! I also used Nudat 3 to obtain a list of nuclides
//! https://www.nndc.bnl.gov/nudat3/guide/
//!
//! Provided under the following terms and conditions:
//!
//!  This disclaimer applies to all BNL web products, including NuDat.
//! 
//! NuDat contains evaluated nuclear data - i.e. recommended values 
//! following a careful analysis of the available data. We cannot, 
//! however, guarantee the accuracy of the data or the absence of errors 
//! in the application. Users are urged to promptly report any problems 
//! encountered when using NuDat.
//! 
//! Users should be aware that NuDat contents change often as new 
//! data is incorporated.
//! 
//! Users should feel free to use the information from NuDat (tables and 
//! plots) in their work, reports, presentations, articles and books. For 
//! those interested in quoting NuDat, we suggest using the following 
//! citation format:
//! 
//! National Nuclear Data Center, information extracted from the 
//! NuDat database, https://www.nndc.bnl.gov/nudat/
//!
//! I also used the periodic table of elements from here:
//! https://gist.github.com/GoodmanSciences/c2dd862cd38f21b0ad36b8f96b4bf1ee
//!
//!
//! This is because JANIS provided nuclide data in the following format
//!
//! eg. Co60
//! and NNDC provided nuclide data in 
//! 60Co
//!
//! This was quite a pain.
//! Hence, I used a lookup table in order to ease some of the pain 
//! This involved doing a periodic table of elements.


#[warn(missing_docs)]

/// provides endf 8 parent independent fission yields,
/// data taken from ENDF/B-VIII using JANIS
pub(crate) mod endf_8_parent_independent_yields;

pub mod prelude;

#[cfg(test)]
pub(crate) mod tutorials_and_tests;

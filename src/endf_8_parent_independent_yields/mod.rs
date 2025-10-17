/// u235 fission yields
pub(crate) mod u235;

/// u232 fission yields
pub(crate) mod u232;
/// u233 fission yields
pub(crate) mod u233;
/// u234 fission yields
pub(crate) mod u234;
/// u236 fission yields
pub(crate) mod u236;
/// u237 fission yields
pub(crate) mod u237;
/// u238 fission yields
pub(crate) mod u238;

/// holds the enums for various nuclides
pub mod nuclides;

/// energy interpolation 
/// since fission yields are energy dependent, 
/// we must account for energy dependence
/// This is done using linear-linear interpolation, the simplest possible
///
pub mod energy_interpolation;

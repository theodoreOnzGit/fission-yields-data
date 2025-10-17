
// uranium fission yields
/// u232 fission yields
pub(crate) mod u232;
/// u233 fission yields
pub(crate) mod u233;
/// u234 fission yields
pub(crate) mod u234;
/// u235 fission yields
pub(crate) mod u235;
/// u236 fission yields
pub(crate) mod u236;
/// u237 fission yields
pub(crate) mod u237;
/// u238 fission yields
pub(crate) mod u238;


// neptunium fission yields
/// np237 fission yields
pub(crate) mod np237;
/// np238 fission yields
pub(crate) mod np238;

// plutonium fission yields
/// pu238 fission yields
pub(crate) mod pu238;
/// pu239 fission yields
pub(crate) mod pu239;
/// pu240 fission yields
pub(crate) mod pu240;
/// pu241 fission yields
pub(crate) mod pu241;
/// pu242 fission yields
pub(crate) mod pu242;


/// holds the enums for various nuclides
pub mod nuclides;

/// energy interpolation 
/// since fission yields are energy dependent, 
/// we must account for energy dependence
/// This is done using linear-linear interpolation, the simplest possible
///
pub mod energy_interpolation;

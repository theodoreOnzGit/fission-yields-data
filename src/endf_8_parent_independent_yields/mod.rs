/// u235 fission yields
pub(crate) mod u235;

/// holds the enums for various nuclides
pub mod nuclides;

/// energy interpolation 
/// since fission yields are energy dependent, 
/// we must account for energy dependence
/// This is done using linear-linear interpolation, the simplest possible
///
pub mod energy_interpolation;

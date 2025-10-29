
use uom::si::f64::*;

use crate::endf_8_parent_independent_yields::nuclides::Nuclide;
use crate::endf_8_parent_independent_yields::pu241::fast_energy::pu241_fast_fission_yield;


/// note, there is no data for high (14.0 MeV) fission yields of pu241
/// just setting it to the fast energy range
pub fn pu241_high_fission_yield(nuc: Nuclide) -> Ratio {

    return pu241_fast_fission_yield(nuc);
}



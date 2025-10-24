use uom::si::f64::*;

use crate::endf_8_parent_independent_yields::nuclides::Nuclide;
use crate::endf_8_parent_independent_yields::energy_interpolation::linear_linear_energy_interpolation;

/// obtains fission yield for of a given nuclide (daughter) 
/// given a fissioning nuclide parent
pub fn fission_yield_linear_interpolation(fissioning_nuclide: Nuclide,
    fission_product_nuclide: Nuclide,
    neutron_energy: Energy) -> Ratio {

    return linear_linear_energy_interpolation(
        neutron_energy, fissioning_nuclide, fission_product_nuclide);

}

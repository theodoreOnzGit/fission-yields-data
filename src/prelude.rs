use uom::si::f64::*;

use crate::endf_8_parent_independent_yields::nuclides::Nuclide;
use crate::endf_8_parent_independent_yields::energy_interpolation::linear_linear_energy_interpolation;

pub fn fission_yield(fissioning_nuclide: Nuclide,
    fission_product_nuclide: Nuclide,
    neutron_energy: Energy) -> Ratio {

    return linear_linear_energy_interpolation(
        neutron_energy, fissioning_nuclide, fission_product_nuclide);

}

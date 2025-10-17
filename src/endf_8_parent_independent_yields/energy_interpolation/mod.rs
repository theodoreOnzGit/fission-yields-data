use uom::si::f64::*;

use crate::endf_8_parent_independent_yields::nuclides::Nuclide;
use crate::endf_8_parent_independent_yields::u233::linear_linear_energy_interpolation_u233;
use crate::endf_8_parent_independent_yields::u235::linear_linear_energy_interpolation_u235;


#[inline]
pub fn linear_linear_energy_interpolation(
    neutron_energy: Energy,
    fissioning_nuclide: Nuclide,
    fission_product_nuclide: Nuclide,
) -> Ratio {

    match fissioning_nuclide {
        Nuclide::U235 => {
            linear_linear_energy_interpolation_u235(neutron_energy, fission_product_nuclide)
        },
        Nuclide::U233 => {
            linear_linear_energy_interpolation_u233(neutron_energy, fission_product_nuclide)
        },
        _ => todo!("fissioning nuclide interpolation not implemented"),
    };

    todo!();
}

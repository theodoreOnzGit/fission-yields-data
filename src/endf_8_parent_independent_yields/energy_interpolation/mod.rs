use uom::si::f64::*;

use crate::endf_8_parent_independent_yields::np237::linear_linear_energy_interpolation_np237;
use crate::endf_8_parent_independent_yields::nuclides::Nuclide;
use crate::endf_8_parent_independent_yields::pu239::linear_linear_energy_interpolation_pu239;
use crate::endf_8_parent_independent_yields::pu240::linear_linear_energy_interpolation_pu240;
use crate::endf_8_parent_independent_yields::pu241::linear_linear_energy_interpolation_pu241;
use crate::endf_8_parent_independent_yields::th232::linear_linear_energy_interpolation_th232;
use crate::endf_8_parent_independent_yields::u233::linear_linear_energy_interpolation_u233;
use crate::endf_8_parent_independent_yields::u234::linear_linear_energy_interpolation_u234;
use crate::endf_8_parent_independent_yields::u235::linear_linear_energy_interpolation_u235;
use crate::endf_8_parent_independent_yields::u236::linear_linear_energy_interpolation_u236;
use crate::endf_8_parent_independent_yields::u238::linear_linear_energy_interpolation_u238;


#[inline]
pub fn linear_linear_energy_interpolation(
    neutron_energy: Energy,
    fissioning_nuclide: Nuclide,
    fission_product_nuclide: Nuclide,
) -> Ratio {

    match fissioning_nuclide {
        Nuclide::U235 => {
            return linear_linear_energy_interpolation_u235(neutron_energy, fission_product_nuclide);
        },
        Nuclide::U233 => {
            return linear_linear_energy_interpolation_u233(neutron_energy, fission_product_nuclide);
        },
        Nuclide::Pu239 => {
            return linear_linear_energy_interpolation_pu239(neutron_energy, fission_product_nuclide);
        },
        Nuclide::U238 => {
            return linear_linear_energy_interpolation_u238(neutron_energy, fission_product_nuclide);
        },
        Nuclide::Th232 => {
            return linear_linear_energy_interpolation_th232(neutron_energy, fission_product_nuclide);
        },
        Nuclide::Np237 => {
            return linear_linear_energy_interpolation_np237(neutron_energy, fission_product_nuclide);
        },
        // code will not work beyond 500 keV
        // as Pu241 has no data in this range
        Nuclide::Pu241 => {
            return linear_linear_energy_interpolation_pu241(neutron_energy, fission_product_nuclide);
        },
        // note: U234 has no thermal fission yield,
        // hence zero yields are supplied at 0.0253 eV 
        // everything else interpolates as per normal
        Nuclide::U234 => {
            return linear_linear_energy_interpolation_u234(neutron_energy, fission_product_nuclide);
        },
        // note: U236 has no thermal fission yield,
        // hence zero yields are supplied at 0.0253 eV 
        // everything else interpolates as per normal
        Nuclide::U236 => {
            return linear_linear_energy_interpolation_u236(neutron_energy, fission_product_nuclide);
        },
        Nuclide::Pu240 => {
            return linear_linear_energy_interpolation_pu240(neutron_energy, fission_product_nuclide);
        },


        _ => todo!("fissioning nuclide interpolation not implemented"),
    };

}

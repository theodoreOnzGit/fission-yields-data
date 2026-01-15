use uom::si::f64::*;

pub use crate::endf_8_parent_independent_yields::nuclides::Nuclide;
pub use crate::endf_8_parent_independent_yields::nuclides::excited_state::ExcitationState;
use crate::endf_8_parent_independent_yields::energy_interpolation::linear_linear_energy_interpolation;

/// obtains fission yield for of a given nuclide (daughter) 
/// given a fissioning nuclide parent
pub fn fission_yield_linear_interpolation(fissioning_nuclide: Nuclide,
    fission_product_nuclide: Nuclide,
    neutron_energy: Energy) -> Ratio {

    return linear_linear_energy_interpolation(
        neutron_energy, fissioning_nuclide, fission_product_nuclide);

}

pub use crate::endf_8_parent_independent_yields::pa231::pa231_fast_fission_yield;
pub use crate::endf_8_parent_independent_yields::u237::u237_fast_fission_yield;
pub use crate::endf_8_parent_independent_yields::u232::thermal_energy::u232_thermal_fission_yield;
pub use crate::endf_8_parent_independent_yields::pu238::pu238_fast_fission_yield;
pub use crate::endf_8_parent_independent_yields::np238::np238_fast_fission_yield;
pub use crate::endf_8_parent_independent_yields::nuclides::string_conversion::parse_nuclide_allow_underscore_isomer;


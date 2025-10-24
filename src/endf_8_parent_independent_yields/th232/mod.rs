
/// fission yields for thermal energy spectrum 
/// 0.0253 eV
///
/// for th232, here is no yield for anything here
///
/// I'm just setting it to zero for convenience
pub(crate) mod thermal_energy;

/// fission yields for fast energy spectrum 
/// 500 keV
pub(crate) mod fast_energy;

/// fission yields for high energy spectrum 
/// Likely fusion
/// 14 MeV
pub(crate) mod high_energy;


use uom::si::f64::*;
use uom::si::energy::{electronvolt, kiloelectronvolt, megaelectronvolt};
use uom::si::ratio::ratio;

use crate::endf_8_parent_independent_yields::nuclides::Nuclide;
use crate::endf_8_parent_independent_yields::th232::fast_energy::th232_fast_fission_yield;
use crate::endf_8_parent_independent_yields::th232::high_energy::th232_high_fission_yield;
use crate::endf_8_parent_independent_yields::th232::thermal_energy::th232_thermal_fission_yield;


pub (crate) fn linear_linear_energy_interpolation_th232(
    neutron_energy: Energy,
    fission_product_nuclide: Nuclide,
) -> Ratio {

    // first we do three cases, of 14 MeV, 500 keV or 0.0253 eV 
    // if neutron energy less than thermal don't bother
    let thermal_neutron_energy = Energy::new::<electronvolt>(0.0253);
    let fast_neutron_energy = Energy::new::<kiloelectronvolt>(500.0);
    let high_neutron_energy = Energy::new::<megaelectronvolt>(14.0);

    // handle edge cases
    if neutron_energy == thermal_neutron_energy {
        // this is the thermal energy case
        return th232_thermal_fission_yield(fission_product_nuclide);

    } else if neutron_energy == fast_neutron_energy {
        // fast energy case
        return th232_fast_fission_yield(fission_product_nuclide);

    } else if neutron_energy == high_neutron_energy {
        // high energy case
        return th232_high_fission_yield(fission_product_nuclide);

    }

    // if neutron energy less than thermal
    // or more than high
    // don't give fission yield

    if neutron_energy < thermal_neutron_energy {
        todo!();
    }
    if neutron_energy > high_neutron_energy {
        todo!();
    }

    // next we decide between thermal to fast and thermal to high 

    // in this case, we interpolate between the high side and low side 

    // we evaluate the fast fission yield first 
    let fast_fission_yield = th232_fast_fission_yield(
        fission_product_nuclide);

    // this is for interpolating between thermal and fast range
    if neutron_energy < fast_neutron_energy {
        let thermal_fission_yield = th232_thermal_fission_yield(
            fission_product_nuclide);

        // the interpolation is 
        // (x - x1)/(x2 - x1)

        let denominator = fast_neutron_energy - thermal_neutron_energy;
        let numerator = neutron_energy - thermal_neutron_energy;

        // basically the degree of fastness is x 
        // where 
        // fission yield = x * fast_yield + (1-x) * thermal_yield
        let degree_of_fastness: f64 = (numerator/denominator).get::<ratio>();

        let fission_yield = degree_of_fastness * fast_fission_yield 
            + (1.0 - degree_of_fastness) * thermal_fission_yield;

        return fission_yield;
    } 

    // this is for interpolating between fast and high range 

    let high_fission_yield = th232_high_fission_yield(
        fission_product_nuclide);

    let denominator = high_neutron_energy - fast_neutron_energy;
    let numerator = neutron_energy - fast_neutron_energy;

    // basically the degree of fastness is x 
    // where 
    // fission yield = x * fast_yield + (1-x) * thermal_yield
    let degree_of_fastness: f64 = (numerator/denominator).get::<ratio>();

    let fission_yield = degree_of_fastness * high_fission_yield 
        + (1.0 - degree_of_fastness) * fast_fission_yield;

    return fission_yield;
}


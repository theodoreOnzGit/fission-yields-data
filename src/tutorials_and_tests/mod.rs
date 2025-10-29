use uom::si::energy::electronvolt;
use uom::si::f64::Energy;
use uom::si::ratio::ratio;

use crate::prelude::fission_yield_linear_interpolation;
use crate::endf_8_parent_independent_yields::nuclides::Nuclide;

#[test]
fn example_1(){
    // in this example we use the test fission product yield 
    // (independent) for Co-66 during U-233 fission by the

    let reference_yield = 1.13997E-9;
    let cobalt_66 = Nuclide::Co66;
    let uranium_233 = Nuclide::U233;
    let neutron_energy = Energy::new::<electronvolt>(0.0253);

    let test_yield: f64 = fission_yield_linear_interpolation(uranium_233, cobalt_66, neutron_energy)
        .get::<ratio>();

    // if coded correctly, the test yield should equal the reference yield 
    assert_eq!(reference_yield,test_yield);

}

/// now, in this example, we wish to generate the fission yield 
/// mass distributions
/// 
/// we can do the following done simply for A=66
/// for U233, for U235 also this works
#[test]
fn example_2(){

    let uranium_233 = Nuclide::U233;
    let neutron_energy = Energy::new::<electronvolt>(0.0253);

    let test_yield = uranium_233.get_mass_yield_66(neutron_energy);

    let reference_yield = 2.502551159E-7 *1e-2;


    // if coded correctly, the test yield should equal the reference yield 
    approx::assert_relative_eq!(
        reference_yield,
        test_yield,
        max_relative=1e-9);

    let uranium_235 = Nuclide::U235;
    let neutron_energy = Energy::new::<electronvolt>(0.0253);

    let test_yield = uranium_235.get_mass_yield_66(neutron_energy);

    let reference_yield = 7.221154783E-8 *1e-2;


    // if coded correctly, the test yield should equal the reference yield 
    approx::assert_relative_eq!(
        reference_yield,
        test_yield,
        max_relative=1e-9);
}


/// now suppose you wanted to have the entire mass distribution
/// this is how you would do it
///
/// this is for the thermal spectrum yields for u233, u235 and pu239
///
/// these serve as important regression tests to check if the individual 
/// yields add up to the mass yields as provided by ENDF/VIII.0
pub mod mass_distributions_thermal_spectrum;



/// now, for fission yields, data is usually given in the neutron energies
///
/// 1. 0.0253 eV 
/// 2. 500 keV
/// 3. 14.0 MeV 
///
/// For Pu239, and some other isotopes, there is also fission data at 2 Mev 
///
/// In this code, we use simple linear interpolation to estimate yields 
/// at certain energies. This is the simplest possible way
pub mod linear_interpolation_examples;

/// for fission yields fast spectrum, some nuclides are tested, eg. U238 
/// since fission only happens at fast spectrum here 
///
/// this is for 500 keV 
pub mod mass_distributions_500_kev_spectrum;

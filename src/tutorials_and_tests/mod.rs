use uom::si::energy::electronvolt;
use uom::si::f64::Energy;
use uom::si::ratio::ratio;

use crate::prelude::fission_yield;
use crate::endf_8_parent_independent_yields::nuclides::Nuclide;

#[test]
fn example_1(){
    // in this example we use the test fission product yield 
    // (independent) for Co-66 during U-233 fission by the

    let reference_yield = 1.13997E-9;
    let cobalt_66 = Nuclide::Co66;
    let uranium_233 = Nuclide::U233;
    let neutron_energy = Energy::new::<electronvolt>(0.0253);

    let test_yield: f64 = fission_yield(uranium_233, cobalt_66, neutron_energy)
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
pub mod mass_distributions;

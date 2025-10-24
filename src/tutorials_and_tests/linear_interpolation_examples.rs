use uom::si::energy::{electronvolt, kiloelectronvolt};
use uom::si::f64::Energy;
use uom::si::ratio::ratio;

use crate::prelude::fission_yield;
use crate::endf_8_parent_independent_yields::nuclides::Nuclide;
/// suppose we are doing yield of Cs-137 for U233
/// at 1 keV incoming neutron
#[test]
fn linear_interpolation_example_1(){

    // first we decide our nuclides
    let cesium_137 = Nuclide::Cs137;
    let uranium_233 = Nuclide::U233;
    let neutron_energy = Energy::new::<kiloelectronvolt>(1.0);
    let test_yield: f64 = fission_yield(
        uranium_233, 
        cesium_137, 
        neutron_energy).get::<ratio>();

    // now behind the scenes, we are linearly interpolating the yields 
    // between 0.0253 eV and 500 keV

    let thermal_energy = Energy::new::<electronvolt>(0.0253);
    let fast_energy = Energy::new::<kiloelectronvolt>(500.0);

    // taken straight from endf 8 libraries, the thermal and fast yields are
    // for cs-137:
    //
    // 
    let thermal_yield: f64 = 0.00710951;
    let fast_yield: f64 = 0.0052946;
    // the high yield for 14 MeV neutron is also provided for reference here
    let _high_yield: f64 = 0.0129206;


    // using linear interpolation, we calculate the reference yield

    let numerator: Energy = neutron_energy - thermal_energy; 
    let denominator: Energy = fast_energy - thermal_energy;

    let reference_yield: f64 = 
        thermal_yield + 
        (fast_yield - thermal_yield) * 
        (numerator/denominator).get::<ratio>();

    // if coded correctly, the test yield should equal the reference yield 
    assert_eq!(reference_yield,test_yield);

}

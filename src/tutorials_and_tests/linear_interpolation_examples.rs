use uom::si::energy::{electronvolt, kiloelectronvolt, megaelectronvolt};
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
/// similarly, suppose 
/// we are doing yield of Cs-137 for U235
/// at 3 MeV incoming neutron
#[test]
fn linear_interpolation_example_2(){

    // first we decide our nuclides
    let cesium_137 = Nuclide::Cs137;
    let uranium_235 = Nuclide::U235;
    let neutron_energy = Energy::new::<megaelectronvolt>(3.0);
    let test_yield: f64 = fission_yield(
        uranium_235, 
        cesium_137, 
        neutron_energy).get::<ratio>();

    // now behind the scenes, we are linearly interpolating the yields 
    // between 0.0253 eV and 500 keV

    let fast_energy = Energy::new::<kiloelectronvolt>(500.0);
    let high_energy = Energy::new::<megaelectronvolt>(14.0);

    // taken straight from endf 8 libraries, the thermal and fast yields are
    // for cs-137:
    //
    // 
    let _thermal_yield: f64 = 5.99988E-4;
    let fast_yield: f64 = 0.00228352;
    // the high yield for 14 MeV neutron is also provided for reference here
    let high_yield: f64 = 0.00947521;


    // using linear interpolation, we calculate the reference yield

    let numerator: Energy = neutron_energy - fast_energy; 
    let denominator: Energy = high_energy - fast_energy;

    let reference_yield: f64 = 
        fast_yield + 
        (high_yield - fast_yield) * 
        (numerator/denominator).get::<ratio>();

    // if coded correctly, the test yield should equal the reference yield 
    approx::assert_relative_eq!(
        reference_yield,
        test_yield,
        max_relative=1e-12
    );

}

/// now, for Pu239, 
/// there are four data points provided for fission yields
///
/// 1. 0.0253 eV
/// 2. 500.0 keV
/// 3. 2.0 MeV
/// 4. 14.0 MeV
///
/// we linearly interpolate between the yields at each of these values for 
/// pu239
///
/// let's use Xe-135 as an example, which is an important reactor poison
///
/// first we interpolate at 1 keV
#[test]
fn linear_interpolation_example_3(){

    // first we decide our nuclides
    let xenon_135 = Nuclide::Xe135;
    let plutonium_239 = Nuclide::Pu239;
    let neutron_energy = Energy::new::<kiloelectronvolt>(1.0);
    let test_yield: f64 = fission_yield(
        plutonium_239, 
        xenon_135, 
        neutron_energy).get::<ratio>();

    // now behind the scenes, we are linearly interpolating the yields 
    // between 0.0253 eV and 500 keV

    let thermal_energy = Energy::new::<electronvolt>(0.0253);
    let fast_energy = Energy::new::<kiloelectronvolt>(500.0);
    let _two_mev_energy = Energy::new::<megaelectronvolt>(2.0);
    let _high_energy = Energy::new::<megaelectronvolt>(14.0);

    // taken straight from endf 8 libraries, the thermal and fast yields are
    // for cs-137:
    //
    // 
    let thermal_yield: f64 = 0.00314131;
    let fast_yield: f64 = 0.006140546;
    let _two_mev_yield: f64 = 0.006147635;
    // the high yield for 14 MeV neutron is also provided for reference here
    let _high_yield: f64 = 0.006461199;


    // using linear interpolation, we calculate the reference yield

    let numerator: Energy = neutron_energy - thermal_energy; 
    let denominator: Energy = fast_energy - thermal_energy;

    let reference_yield: f64 = 
        thermal_yield + 
        (fast_yield - thermal_yield) * 
        (numerator/denominator).get::<ratio>();

    // if coded correctly, the test yield should equal the reference yield 
    approx::assert_relative_eq!(
        reference_yield,
        test_yield,
        max_relative=1e-12
    );

}
/// now, for Pu239, 
/// there are four data points provided for fission yields
///
/// 1. 0.0253 eV
/// 2. 500.0 keV
/// 3. 2.0 MeV
/// 4. 14.0 MeV
///
/// we linearly interpolate between the yields at each of these values for 
/// pu239
///
/// let's use Xe-135 as an example, which is an important reactor poison
///
/// continuing from the previous test, lets do 1.5 MeV, which 
/// is between 500 and 2.0 MeV
#[test]
fn linear_interpolation_example_4(){

    // first we decide our nuclides
    let xenon_135 = Nuclide::Xe135;
    let plutonium_239 = Nuclide::Pu239;
    let neutron_energy = Energy::new::<kiloelectronvolt>(1500.0);
    let test_yield: f64 = fission_yield(
        plutonium_239, 
        xenon_135, 
        neutron_energy).get::<ratio>();

    // now behind the scenes, we are linearly interpolating the yields 
    // between 0.0253 eV and 500 keV

    let _thermal_energy = Energy::new::<electronvolt>(0.0253);
    let fast_energy = Energy::new::<kiloelectronvolt>(500.0);
    let two_mev_energy = Energy::new::<megaelectronvolt>(2.0);
    let _high_energy = Energy::new::<megaelectronvolt>(14.0);

    // taken straight from endf 8 libraries, the thermal and fast yields are
    // for cs-137:
    //
    // 
    let _thermal_yield: f64 = 0.00314131;
    let fast_yield: f64 = 0.006140546;
    let two_mev_yield: f64 = 0.006147635;
    // the high yield for 14 MeV neutron is also provided for reference here
    let _high_yield: f64 = 0.006461199;


    // using linear interpolation, we calculate the reference yield

    let numerator: Energy = neutron_energy - fast_energy; 
    let denominator: Energy = two_mev_energy - fast_energy;

    let reference_yield: f64 = 
        fast_yield + 
        (two_mev_yield - fast_yield) * 
        (numerator/denominator).get::<ratio>();

    // if coded correctly, the test yield should equal the reference yield 
    approx::assert_relative_eq!(
        reference_yield,
        test_yield,
        max_relative=1e-12
    );

}

use uom::si::ratio::ratio;

use crate::endf_8_parent_independent_yields::u235::thermal_energy::u235_thermal_fission_yield;
use crate::endf_8_parent_independent_yields::nuclides::Nuclides;

#[test] 
fn fission_yield_test_se87(){

    let test_nuclide = Nuclides::Se87;
    let reference_yield: f64 = 7.310480E-03;

    let test_yield = u235_thermal_fission_yield(test_nuclide).get::<ratio>();

    assert_eq!(reference_yield,test_yield);
}

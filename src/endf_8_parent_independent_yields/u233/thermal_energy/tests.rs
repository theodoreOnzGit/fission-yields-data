use uom::si::ratio::ratio;

use crate::endf_8_parent_independent_yields::u233::thermal_energy::u233_thermal_fission_yield;
use crate::endf_8_parent_independent_yields::nuclides::Nuclide;

#[test] 
fn fission_yield_test_fe70(){

    let test_nuclide = Nuclide::Fe70;
    let reference_yield: f64 = 1.23997E-10;

    let test_yield = u233_thermal_fission_yield(test_nuclide).get::<ratio>();

    assert_eq!(reference_yield,test_yield);
}

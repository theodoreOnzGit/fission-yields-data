use uom::si::energy::electronvolt;
use uom::si::f64::Energy;
use uom::si::ratio::ratio;

use crate::prelude::fission_yield;
use crate::endf_8_parent_independent_yields::nuclides::Nuclide;

#[test]
fn example_1(){
    // in this example we use the test fission product yield 
    // (independent) for Co-66 during U-233 fission by the

    let reference_yield = 1.95995E-9;
    let cobalt_66 = Nuclide::Co66;
    let uranium_233 = Nuclide::U233;
    let neutron_energy = Energy::new::<electronvolt>(0.0253);

    let test_yield: f64 = fission_yield(uranium_233, cobalt_66, neutron_energy)
        .get::<ratio>();

    // if coded correctly, the test yield should equal the reference yield 
    assert_eq!(reference_yield,test_yield);

}

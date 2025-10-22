use crate::{endf_8_parent_independent_yields::nuclides::Nuclide, prelude::fission_yield};
use uom::si::{f64::*, ratio::ratio};

impl Nuclide {

    /// gets mass yield for A=66
    pub fn get_mass_yield_66(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_66: f64 = 0.0;
        // Z=23 (vanadium)
        distribution_yield_66 += 
            fission_yield(
                *self, 
                Nuclide::V66, 
                neutron_energy).get::<ratio>();
        // Z=24 (Chromium)
        distribution_yield_66 += 
            fission_yield(
                *self, 
                Nuclide::Cr66, 
                neutron_energy).get::<ratio>();
        // Z=25 (Manganese)
        distribution_yield_66 += 
            fission_yield(
                *self, 
                Nuclide::Mn66, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_66 += 
            fission_yield(
                *self, 
                Nuclide::Fe66, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_66 += 
            fission_yield(
                *self, 
                Nuclide::Co66, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_66 += 
            fission_yield(
                *self, 
                Nuclide::Ni66, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_66 += 
            fission_yield(
                *self, 
                Nuclide::Cu66, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_66 += 
            fission_yield(
                *self, 
                Nuclide::Zn66, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_66 += 
            fission_yield(
                *self, 
                Nuclide::Ga66, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_66 += 
            fission_yield(
                *self, 
                Nuclide::Ge66, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic (none)

        return distribution_yield_66;

    }


    /// gets mass yield for A=67
    pub fn get_mass_yield_67(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_67: f64 = 0.0;
        // Z=23 (vanadium)
        distribution_yield_67 += 
            fission_yield(
                *self, 
                Nuclide::V67, 
                neutron_energy).get::<ratio>();
        // Z=24 (Chromium)
        distribution_yield_67 += 
            fission_yield(
                *self, 
                Nuclide::Cr67, 
                neutron_energy).get::<ratio>();
        // Z=25 (Manganese)
        distribution_yield_67 += 
            fission_yield(
                *self, 
                Nuclide::Mn67, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_67 += 
            fission_yield(
                *self, 
                Nuclide::Fe67, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_67 += 
            fission_yield(
                *self, 
                Nuclide::Co67, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_67 += 
            fission_yield(
                *self, 
                Nuclide::Ni67, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_67 += 
            fission_yield(
                *self, 
                Nuclide::Cu67, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_67 += 
            fission_yield(
                *self, 
                Nuclide::Zn67, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_67 += 
            fission_yield(
                *self, 
                Nuclide::Ga67, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_67 += 
            fission_yield(
                *self, 
                Nuclide::Ge67, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic (none)

        return distribution_yield_67;

    }

    /// gets mass yield for A=68
    pub fn get_mass_yield_68(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_68: f64 = 0.0;
        // Z=23 (vanadium)
        distribution_yield_68 += 
            fission_yield(
                *self, 
                Nuclide::V68, 
                neutron_energy).get::<ratio>();
        // Z=24 (Chromium)
        distribution_yield_68 += 
            fission_yield(
                *self, 
                Nuclide::Cr68, 
                neutron_energy).get::<ratio>();
        // Z=25 (Manganese)
        distribution_yield_68 += 
            fission_yield(
                *self, 
                Nuclide::Mn68, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_68 += 
            fission_yield(
                *self, 
                Nuclide::Fe68, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_68 += 
            fission_yield(
                *self, 
                Nuclide::Co68, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_68 += 
            fission_yield(
                *self, 
                Nuclide::Ni68, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_68 += 
            fission_yield(
                *self, 
                Nuclide::Cu68, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_68 += 
            fission_yield(
                *self, 
                Nuclide::Zn68, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_68 += 
            fission_yield(
                *self, 
                Nuclide::Ga68, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_68 += 
            fission_yield(
                *self, 
                Nuclide::Ge68, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic (none)

        return distribution_yield_68;

    }

    /// gets mass yield for A=69
    pub fn get_mass_yield_69(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_69: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium)
        distribution_yield_69 += 
            fission_yield(
                *self, 
                Nuclide::Cr69, 
                neutron_energy).get::<ratio>();
        // Z=25 (Manganese)
        distribution_yield_69 += 
            fission_yield(
                *self, 
                Nuclide::Mn69, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_69 += 
            fission_yield(
                *self, 
                Nuclide::Fe69, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_69 += 
            fission_yield(
                *self, 
                Nuclide::Co69, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_69 += 
            fission_yield(
                *self, 
                Nuclide::Ni69, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_69 += 
            fission_yield(
                *self, 
                Nuclide::Cu69, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_69 += 
            fission_yield(
                *self, 
                Nuclide::Zn69, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_69 += 
            fission_yield(
                *self, 
                Nuclide::Ga69, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_69 += 
            fission_yield(
                *self, 
                Nuclide::Ge69, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_69 += 
            fission_yield(
                *self, 
                Nuclide::As69, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium (none)

        return distribution_yield_69;

    }
}


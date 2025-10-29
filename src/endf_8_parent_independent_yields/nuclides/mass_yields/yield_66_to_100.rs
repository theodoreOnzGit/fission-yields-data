use crate::{endf_8_parent_independent_yields::nuclides::Nuclide, prelude::fission_yield_linear_interpolation};
use uom::si::{f64::*, ratio::ratio};

impl Nuclide {

    /// gets mass yield for A=66
    pub fn get_mass_yield_66(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_66: f64 = 0.0;
        // Z=23 (vanadium)
        distribution_yield_66 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::V66, 
                neutron_energy).get::<ratio>();
        // Z=24 (Chromium)
        distribution_yield_66 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cr66, 
                neutron_energy).get::<ratio>();
        // Z=25 (Manganese)
        distribution_yield_66 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mn66, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_66 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Fe66, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_66 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co66, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_66 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni66, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_66 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu66, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_66 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn66, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_66 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga66, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_66 += 
            fission_yield_linear_interpolation(
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
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::V67, 
                neutron_energy).get::<ratio>();
        // Z=24 (Chromium)
        distribution_yield_67 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cr67, 
                neutron_energy).get::<ratio>();
        // Z=25 (Manganese)
        distribution_yield_67 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mn67, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_67 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Fe67, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_67 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co67, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_67 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni67, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_67 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu67, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_67 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn67, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_67 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga67, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_67 += 
            fission_yield_linear_interpolation(
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
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::V68, 
                neutron_energy).get::<ratio>();
        // Z=24 (Chromium)
        distribution_yield_68 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cr68, 
                neutron_energy).get::<ratio>();
        // Z=25 (Manganese)
        distribution_yield_68 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mn68, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_68 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Fe68, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_68 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co68, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_68 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni68, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_68 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu68, 
                neutron_energy).get::<ratio>();
        distribution_yield_68 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu68m, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_68 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn68, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_68 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga68, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_68 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge68, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic (none)

        return distribution_yield_68;

    }

    /// gets mass yield for A=69
    /// after this no more vanadium
    pub fn get_mass_yield_69(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_69: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium)
        distribution_yield_69 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cr69, 
                neutron_energy).get::<ratio>();
        // Z=25 (Manganese)
        distribution_yield_69 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mn69, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_69 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Fe69, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_69 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co69, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_69 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni69, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_69 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu69, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_69 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn69, 
                neutron_energy).get::<ratio>();
        distribution_yield_69 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn69m, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_69 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga69, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_69 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge69, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_69 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As69, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium (none)

        return distribution_yield_69;

    }
    /// gets mass yield for A=70
    pub fn get_mass_yield_70(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_70: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) 
        distribution_yield_70 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cr70, 
                neutron_energy).get::<ratio>();
        // Z=25 (Manganese)
        distribution_yield_70 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mn70, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_70 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Fe70, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_70 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co70, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_70 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni70, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_70 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu70, 
                neutron_energy).get::<ratio>();
        distribution_yield_70 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu70m, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_70 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn70, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_70 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga70, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_70 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge70, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic (none)
        //distribution_yield_70 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::As70, 
        //        neutron_energy).get::<ratio>();
        // Z=34 Selenium (none)

        return distribution_yield_70;

    }


    /// gets mass yield for A=71
    pub fn get_mass_yield_71(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_71: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese)
        distribution_yield_71 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mn71, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_71 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Fe71, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_71 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co71, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_71 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni71, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_71 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu71, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_71 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn71, 
                neutron_energy).get::<ratio>();
        distribution_yield_71 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn71m, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_71 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga71, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_71 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge71, 
                neutron_energy).get::<ratio>();
        distribution_yield_71 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge71m, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_71 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As71, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium (none)

        return distribution_yield_71;

    }
    /// gets mass yield for A=72
    pub fn get_mass_yield_72(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_72: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese)
        distribution_yield_72 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mn72, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_72 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Fe72, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_72 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co72, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_72 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni72, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_72 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu72, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_72 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn72, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_72 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga72, 
                neutron_energy).get::<ratio>();
        distribution_yield_72 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga72m, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_72 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge72, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_72 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As72, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_72 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se72, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine -- none

        return distribution_yield_72;

    }
    /// gets mass yield for A=73
    pub fn get_mass_yield_73(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_73: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese)
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mn73, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Fe73, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co73, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni73, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu73, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn73, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga73, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge73, 
                neutron_energy).get::<ratio>();
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge73m, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As73, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se73, 
                neutron_energy).get::<ratio>();
        distribution_yield_73 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se73m, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine -- none

        return distribution_yield_73;

    }
    /// gets mass yield for A=74
    pub fn get_mass_yield_74(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_74: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 Iron
        distribution_yield_74 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Fe74, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_74 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co74, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_74 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni74, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_74 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu74, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_74 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn74, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_74 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga74, 
                neutron_energy).get::<ratio>();
        distribution_yield_74 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga74m, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_74 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge74, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_74 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As74, 
                neutron_energy).get::<ratio>();
        distribution_yield_74 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As74m, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_74 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se74, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine (none)

        return distribution_yield_74;

    }
    /// gets mass yield for A=75
    pub fn get_mass_yield_75(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_75: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 Iron
        distribution_yield_75 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Fe75, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_75 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co75, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_75 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni75, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_75 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu75, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_75 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn75, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_75 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga75, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_75 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge75, 
                neutron_energy).get::<ratio>();
        distribution_yield_75 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge75m, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_75 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As75, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_75 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se75, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_75 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br75, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton (none)
        //distribution_yield_75 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Kr75, 
        //        neutron_energy).get::<ratio>();


        return distribution_yield_75;

    }


    /// gets mass yield for A=76
    pub fn get_mass_yield_76(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_76: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 Iron
        distribution_yield_76 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Fe76, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_76 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co76, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_76 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni76, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_76 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu76, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_76 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn76, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_76 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga76, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_76 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge76, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_76 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As76, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_76 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se76, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine (none)
        //distribution_yield_76 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Br76, 
        //        neutron_energy).get::<ratio>();
        // Z=36 Krypton (none)
        //distribution_yield_76 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Kr76, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_76;

    }
    /// gets mass yield for A=77
    pub fn get_mass_yield_77(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_77: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 Cobalt
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co77, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni77, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu77, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn77, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga77, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge77, 
                neutron_energy).get::<ratio>();
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge77m, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As77, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se77, 
                neutron_energy).get::<ratio>();
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se77m, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br77, 
                neutron_energy).get::<ratio>();
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br77m, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_77 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr77, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium (none)
        //distribution_yield_77 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Rb77, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_77;

    }
    /// gets mass yield for A=78
    pub fn get_mass_yield_78(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_78: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 Cobalt
        distribution_yield_78 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Co78, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_78 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni78, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_78 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu78, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_78 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn78, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_78 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga78, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_78 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge78, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_78 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As78, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_78 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se78, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_78 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br78, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_78 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr78, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium (none)
        //distribution_yield_78 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Rb78, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_78;

    }
    /// gets mass yield for A=79
    pub fn get_mass_yield_79(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_79: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 Nickel
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni79, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu79, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn79, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga79, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge79, 
                neutron_energy).get::<ratio>();
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge79m, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As79, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se79, 
                neutron_energy).get::<ratio>();
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se79m, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br79, 
                neutron_energy).get::<ratio>();
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br79m, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr79, 
                neutron_energy).get::<ratio>();
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr79m, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_79 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb79, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        //distribution_yield_79 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sr79, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_79;

    }


    /// gets mass yield for A=80
    pub fn get_mass_yield_80(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_80: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 Nickel
        distribution_yield_80 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni80, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_80 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu80, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_80 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn80, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_80 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga80, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_80 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge80, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_80 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As80, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_80 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se80, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_80 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br80, 
                neutron_energy).get::<ratio>();
        distribution_yield_80 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br80m, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_80 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr80, 
                neutron_energy).get::<ratio>();

        // Z=37 Rubidium 
        //distribution_yield_80 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Rb80, 
        //        neutron_energy).get::<ratio>();
        // Z=38 Strontium
        //distribution_yield_80 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sr80, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_80;

    }
    /// gets mass yield for A=81
    pub fn get_mass_yield_81(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_81: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 Nickel
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni81, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu81, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn81, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga81, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge81, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As81, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se81, 
                neutron_energy).get::<ratio>();
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se81m, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br81, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr81, 
                neutron_energy).get::<ratio>();
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr81m, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_81 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb81, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        //distribution_yield_81 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sr81, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_81;

    }
    /// gets mass yield for A=82
    pub fn get_mass_yield_82(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_82: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 Nickel
        distribution_yield_82 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ni82, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_82 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu82, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_82 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn82, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_82 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga82, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_82 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge82, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_82 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As82, 
                neutron_energy).get::<ratio>();
        distribution_yield_82 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As82m, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_82 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se82, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_82 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br82, 
                neutron_energy).get::<ratio>();
        distribution_yield_82 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br82m, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_82 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr82, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        //distribution_yield_82 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Rb82, 
        //        neutron_energy).get::<ratio>();
        // Z=38 Strontium
        //distribution_yield_82 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sr82, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_82;

    }
    /// gets mass yield for A=83
    pub fn get_mass_yield_83(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_83: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 Copper
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Cu83, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn83, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga83, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge83, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As83, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se83, 
                neutron_energy).get::<ratio>();
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se83m, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br83, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr83, 
                neutron_energy).get::<ratio>();
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr83m, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb83, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_83 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr83, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        //distribution_yield_83 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Y83, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_83;

    }
    /// gets mass yield for A=84
    pub fn get_mass_yield_84(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_84: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 Zinc
        distribution_yield_84 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn84, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_84 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga84, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_84 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge84, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_84 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As84, 
                neutron_energy).get::<ratio>();
        distribution_yield_84 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As84m, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_84 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se84, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_84 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br84, 
                neutron_energy).get::<ratio>();
        distribution_yield_84 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br84m, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_84 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr84, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_84 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb84, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_84 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr84, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        //distribution_yield_84 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Y84, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_84;

    }
    /// gets mass yield for A=85
    pub fn get_mass_yield_85(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_85: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 Zinc
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn85, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga85, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge85, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As85, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se85, 
                neutron_energy).get::<ratio>();
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se85m, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br85, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr85, 
                neutron_energy).get::<ratio>();
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr85m, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb85, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr85, 
                neutron_energy).get::<ratio>();
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr85m, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_85 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y85, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        //distribution_yield_85 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Zr85, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_85;

    }

    /// gets mass yield for A=86
    pub fn get_mass_yield_86(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_86: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 Zinc
        distribution_yield_86 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zn86, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_86 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga86, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_86 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge86, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_86 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As86, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_86 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se86, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_86 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br86, 
                neutron_energy).get::<ratio>();
        distribution_yield_86 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br86m, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_86 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr86, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_86 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb86, 
                neutron_energy).get::<ratio>();
        distribution_yield_86 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb86m, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_86 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr86, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        //distribution_yield_86 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Y86, 
        //        neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        //distribution_yield_86 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Zr86, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_86;

    }
    /// gets mass yield for A=87
    pub fn get_mass_yield_87(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_87: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 Gallium
        distribution_yield_87 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga87, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_87 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge87, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_87 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As87, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_87 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se87, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_87 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br87, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_87 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr87, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_87 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb87, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_87 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr87, 
                neutron_energy).get::<ratio>();
        distribution_yield_87 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr87m, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_87 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y87, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_87 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr87, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        //distribution_yield_87 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Nb87, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_87;

    }
    /// gets mass yield for A=88
    pub fn get_mass_yield_88(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_88: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 Gallium
        distribution_yield_88 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ga88, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_88 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge88, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_88 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As88, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_88 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se88, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_88 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br88, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_88 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr88, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_88 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb88, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_88 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr88, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_88 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y88, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_88 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr88, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        //distribution_yield_88 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Nb88, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_88;

    }
    /// gets mass yield for A=89
    pub fn get_mass_yield_89(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_89: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 Germanium
        distribution_yield_89 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge89, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_89 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As89, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_89 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se89, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_89 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br89, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_89 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr89, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_89 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb89, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_89 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr89, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_89 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y89, 
                neutron_energy).get::<ratio>();
        distribution_yield_89 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y89m, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_89 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr89, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_89 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb89, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        //distribution_yield_89 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Mo89, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_89;

    }
    /// gets mass yield for A=90
    pub fn get_mass_yield_90(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_90: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 Germanium
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge90, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As90, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se90, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br90, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr90, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb90, 
                neutron_energy).get::<ratio>();
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb90m, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr90, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y90, 
                neutron_energy).get::<ratio>();
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y90m, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr90, 
                neutron_energy).get::<ratio>();
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr90m, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb90, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_90 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo90, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        //distribution_yield_90 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tc90, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_90;

    }
    /// gets mass yield for A=91
    pub fn get_mass_yield_91(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_91: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 Germanium
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ge91, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As91, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se91, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br91, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr91, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb91, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr91, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y91, 
                neutron_energy).get::<ratio>();
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y91m, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr91, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb91, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_91 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo91, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        //distribution_yield_91 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tc91, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_91;

    }
    /// gets mass yield for A=92
    pub fn get_mass_yield_92(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_92: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 (Germanium) -- none
        // Z=33 Arsenic 
        distribution_yield_92 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As92, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_92 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se92, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_92 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br92, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_92 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr92, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_92 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb92, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_92 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr92, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_92 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y92, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_92 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr92, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_92 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb92, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_92 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo92, 
                neutron_energy).get::<ratio>();

        // Z=43 Technetium
        //distribution_yield_92 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tc92, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_92;

    }
    /// gets mass yield for A=93
    pub fn get_mass_yield_93(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_93: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 (Germanium) -- none
        // Z=33 Arsenic 
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::As93, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se93, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br93, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr93, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb93, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr93, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y93, 
                neutron_energy).get::<ratio>();
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y93m, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr93, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb93, 
                neutron_energy).get::<ratio>();
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb93m, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo93, 
                neutron_energy).get::<ratio>();
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo93m, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_93 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Tc93, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        //distribution_yield_93 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ru93, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_93;

    }
    /// gets mass yield for A=94
    pub fn get_mass_yield_94(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_94: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 (Germanium) -- none
        // Z=33 (Arsenic) -- none
        // Z=34 Selenium 
        distribution_yield_94 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se94, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_94 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br94, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_94 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr94, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_94 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb94, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_94 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr94, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_94 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y94, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_94 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr94, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_94 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb94, 
                neutron_energy).get::<ratio>();
        distribution_yield_94 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb94m, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_94 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo94, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        //distribution_yield_94 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tc94, 
        //        neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        //distribution_yield_94 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ru94, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_94;

    }
    /// gets mass yield for A=95
    pub fn get_mass_yield_95(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_95: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 (Germanium) -- none
        // Z=33 (Arsenic) -- none
        // Z=34 Selenium 
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se95, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br95, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr95, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb95, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr95, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y95, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr95, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb95, 
                neutron_energy).get::<ratio>();
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb95m, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo95, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Tc95, 
                neutron_energy).get::<ratio>();
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Tc95m, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_95 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ru95, 
                neutron_energy).get::<ratio>();
        // Z=45 Rhodium
        //distribution_yield_95 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Rh95, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_95;

    }
    /// gets mass yield for A=96
    pub fn get_mass_yield_96(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_96: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 (Germanium) -- none
        // Z=33 (Arsenic) -- none
        // Z=34 Selenium 
        distribution_yield_96 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Se96, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_96 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br96, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_96 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr96, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_96 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb96, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_96 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr96, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_96 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y96, 
                neutron_energy).get::<ratio>();
        distribution_yield_96 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y96m, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_96 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr96, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_96 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb96, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_96 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo96, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        //distribution_yield_96 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tc96, 
        //        neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_96 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ru96, 
                neutron_energy).get::<ratio>();
        // Z=45 Rhodium
        //distribution_yield_96 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Rh96, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_96;

    }
    /// gets mass yield for A=97
    pub fn get_mass_yield_97(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_97: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 (Germanium) -- none
        // Z=33 (Arsenic) -- none
        // Z=34 (Selenium) -- none
        // Z=35 Bromine 
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br97, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr97, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb97, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr97, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y97, 
                neutron_energy).get::<ratio>();
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y97m, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr97, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb97, 
                neutron_energy).get::<ratio>();
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb97m, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo97, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Tc97, 
                neutron_energy).get::<ratio>();
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Tc97m, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_97 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ru97, 
                neutron_energy).get::<ratio>();
        // Z=45 Rhodium
        //distribution_yield_97 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Rh97, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_97;

    }
    /// gets mass yield for A=98
    pub fn get_mass_yield_98(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_98: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 (Germanium) -- none
        // Z=33 (Arsenic) -- none
        // Z=34 (Selenium) -- none
        // Z=35 Bromine 
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Br98, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr98, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb98, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr98, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y98, 
                neutron_energy).get::<ratio>();
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y98m, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr98, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb98, 
                neutron_energy).get::<ratio>();
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb98m, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo98, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Tc98, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_98 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ru98, 
                neutron_energy).get::<ratio>();
        // Z=45 Rhodium
        //distribution_yield_98 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Rh98, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_98;

    }
    /// gets mass yield for A=99
    pub fn get_mass_yield_99(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_99: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 (Germanium) -- none
        // Z=33 (Arsenic) -- none
        // Z=34 (Selenium) -- none
        // Z=35 (Bromine) -- none
        // Z=36 Krypton 
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr99, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb99, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr99, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y99, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr99, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb99, 
                neutron_energy).get::<ratio>();
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb99m, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo99, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Tc99, 
                neutron_energy).get::<ratio>();
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Tc99m, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ru99, 
                neutron_energy).get::<ratio>();
        // Z=45 Rhodium
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rh99, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_99 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Pd99, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        //distribution_yield_99 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ag99, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_99;

    }
    /// gets mass yield for A=100
    pub fn get_mass_yield_100(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_100: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) -- none
        // Z=25 (Manganese) -- none
        // Z=26 (Iron) -- none
        // Z=27 (Cobalt) -- none
        // Z=28 (Nickel) -- none
        // Z=29 (Copper) -- none
        // Z=30 (Zinc) -- none
        // Z=31 (Gallium) -- none
        // Z=32 (Germanium) -- none
        // Z=33 (Arsenic) -- none
        // Z=34 (Selenium) -- none
        // Z=35 (Bromine) -- none
        // Z=36 Krypton 
        distribution_yield_100 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Kr100, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_100 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Rb100, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_100 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Sr100, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_100 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Y100, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_100 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Zr100, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_100 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb100, 
                neutron_energy).get::<ratio>();
        distribution_yield_100 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Nb100m, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_100 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Mo100, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_100 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Tc100, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_100 += 
            fission_yield_linear_interpolation(
                *self, 
                Nuclide::Ru100, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        //distribution_yield_100 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Rh100, 
        //        neutron_energy).get::<ratio>();
        // Z=46 Paladium
        //distribution_yield_100 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Pd100, 
        //        neutron_energy).get::<ratio>();
        // Z=47 Silver
        //distribution_yield_100 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ag100, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_100;

    }
}

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
    /// after this no more vanadium
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
    /// gets mass yield for A=70
    pub fn get_mass_yield_70(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_70: f64 = 0.0;
        // Z=23 (vanadium) -- none
        // Z=24 (Chromium) 
        distribution_yield_70 += 
            fission_yield(
                *self, 
                Nuclide::Cr70, 
                neutron_energy).get::<ratio>();
        // Z=25 (Manganese)
        distribution_yield_70 += 
            fission_yield(
                *self, 
                Nuclide::Mn70, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_70 += 
            fission_yield(
                *self, 
                Nuclide::Fe70, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_70 += 
            fission_yield(
                *self, 
                Nuclide::Co70, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_70 += 
            fission_yield(
                *self, 
                Nuclide::Ni70, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_70 += 
            fission_yield(
                *self, 
                Nuclide::Cu70, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_70 += 
            fission_yield(
                *self, 
                Nuclide::Zn70, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_70 += 
            fission_yield(
                *self, 
                Nuclide::Ga70, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_70 += 
            fission_yield(
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
            fission_yield(
                *self, 
                Nuclide::Mn71, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_71 += 
            fission_yield(
                *self, 
                Nuclide::Fe71, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_71 += 
            fission_yield(
                *self, 
                Nuclide::Co71, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_71 += 
            fission_yield(
                *self, 
                Nuclide::Ni71, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_71 += 
            fission_yield(
                *self, 
                Nuclide::Cu71, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_71 += 
            fission_yield(
                *self, 
                Nuclide::Zn71, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_71 += 
            fission_yield(
                *self, 
                Nuclide::Ga71, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_71 += 
            fission_yield(
                *self, 
                Nuclide::Ge71, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_71 += 
            fission_yield(
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
            fission_yield(
                *self, 
                Nuclide::Mn72, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_72 += 
            fission_yield(
                *self, 
                Nuclide::Fe72, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_72 += 
            fission_yield(
                *self, 
                Nuclide::Co72, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_72 += 
            fission_yield(
                *self, 
                Nuclide::Ni72, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_72 += 
            fission_yield(
                *self, 
                Nuclide::Cu72, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_72 += 
            fission_yield(
                *self, 
                Nuclide::Zn72, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_72 += 
            fission_yield(
                *self, 
                Nuclide::Ga72, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_72 += 
            fission_yield(
                *self, 
                Nuclide::Ge72, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_72 += 
            fission_yield(
                *self, 
                Nuclide::As72, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_72 += 
            fission_yield(
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
            fission_yield(
                *self, 
                Nuclide::Mn73, 
                neutron_energy).get::<ratio>();
        // Z=26 Iron
        distribution_yield_73 += 
            fission_yield(
                *self, 
                Nuclide::Fe73, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_73 += 
            fission_yield(
                *self, 
                Nuclide::Co73, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_73 += 
            fission_yield(
                *self, 
                Nuclide::Ni73, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_73 += 
            fission_yield(
                *self, 
                Nuclide::Cu73, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_73 += 
            fission_yield(
                *self, 
                Nuclide::Zn73, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_73 += 
            fission_yield(
                *self, 
                Nuclide::Ga73, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_73 += 
            fission_yield(
                *self, 
                Nuclide::Ge73, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_73 += 
            fission_yield(
                *self, 
                Nuclide::As73, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_73 += 
            fission_yield(
                *self, 
                Nuclide::Se73, 
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
            fission_yield(
                *self, 
                Nuclide::Fe74, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_74 += 
            fission_yield(
                *self, 
                Nuclide::Co74, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_74 += 
            fission_yield(
                *self, 
                Nuclide::Ni74, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_74 += 
            fission_yield(
                *self, 
                Nuclide::Cu74, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_74 += 
            fission_yield(
                *self, 
                Nuclide::Zn74, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_74 += 
            fission_yield(
                *self, 
                Nuclide::Ga74, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_74 += 
            fission_yield(
                *self, 
                Nuclide::Ge74, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_74 += 
            fission_yield(
                *self, 
                Nuclide::As74, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_74 += 
            fission_yield(
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
            fission_yield(
                *self, 
                Nuclide::Fe75, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_75 += 
            fission_yield(
                *self, 
                Nuclide::Co75, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_75 += 
            fission_yield(
                *self, 
                Nuclide::Ni75, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_75 += 
            fission_yield(
                *self, 
                Nuclide::Cu75, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_75 += 
            fission_yield(
                *self, 
                Nuclide::Zn75, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_75 += 
            fission_yield(
                *self, 
                Nuclide::Ga75, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_75 += 
            fission_yield(
                *self, 
                Nuclide::Ge75, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_75 += 
            fission_yield(
                *self, 
                Nuclide::As75, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_75 += 
            fission_yield(
                *self, 
                Nuclide::Se75, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_75 += 
            fission_yield(
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
            fission_yield(
                *self, 
                Nuclide::Fe76, 
                neutron_energy).get::<ratio>();

        // Z=27 Cobalt
        distribution_yield_76 += 
            fission_yield(
                *self, 
                Nuclide::Co76, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_76 += 
            fission_yield(
                *self, 
                Nuclide::Ni76, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_76 += 
            fission_yield(
                *self, 
                Nuclide::Cu76, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_76 += 
            fission_yield(
                *self, 
                Nuclide::Zn76, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_76 += 
            fission_yield(
                *self, 
                Nuclide::Ga76, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_76 += 
            fission_yield(
                *self, 
                Nuclide::Ge76, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_76 += 
            fission_yield(
                *self, 
                Nuclide::As76, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_76 += 
            fission_yield(
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
            fission_yield(
                *self, 
                Nuclide::Co77, 
                neutron_energy).get::<ratio>();
        // Z=28 Nickel
        distribution_yield_77 += 
            fission_yield(
                *self, 
                Nuclide::Ni77, 
                neutron_energy).get::<ratio>();
        // Z=29 Copper
        distribution_yield_77 += 
            fission_yield(
                *self, 
                Nuclide::Cu77, 
                neutron_energy).get::<ratio>();
        // Z=30 Zinc
        distribution_yield_77 += 
            fission_yield(
                *self, 
                Nuclide::Zn77, 
                neutron_energy).get::<ratio>();
        // Z=31 Gallium
        distribution_yield_77 += 
            fission_yield(
                *self, 
                Nuclide::Ga77, 
                neutron_energy).get::<ratio>();
        // Z=32 Germanium
        distribution_yield_77 += 
            fission_yield(
                *self, 
                Nuclide::Ge77, 
                neutron_energy).get::<ratio>();
        // Z=33 Arsenic 
        distribution_yield_77 += 
            fission_yield(
                *self, 
                Nuclide::As77, 
                neutron_energy).get::<ratio>();
        // Z=34 Selenium 
        distribution_yield_77 += 
            fission_yield(
                *self, 
                Nuclide::Se77, 
                neutron_energy).get::<ratio>();
        // Z=35 Bromine 
        distribution_yield_77 += 
            fission_yield(
                *self, 
                Nuclide::Br77, 
                neutron_energy).get::<ratio>();
        // Z=36 Krypton 
        distribution_yield_77 += 
            fission_yield(
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
}


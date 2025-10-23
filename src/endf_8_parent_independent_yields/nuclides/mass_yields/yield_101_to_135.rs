use crate::{endf_8_parent_independent_yields::nuclides::Nuclide, prelude::fission_yield};
use uom::si::{f64::*, ratio::ratio};
impl Nuclide {

    /// gets mass yield for A=101
    pub fn get_mass_yield_101(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_101: f64 = 0.0;
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
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Kr101, 
                neutron_energy).get::<ratio>();
        // Z=37 Rubidium 
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Rb101, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Sr101, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Y101, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Zr101, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Nb101, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Mo101, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Tc101, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Ru101, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Rh101, 
                neutron_energy).get::<ratio>();
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Rh101m, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_101 += 
            fission_yield(
                *self, 
                Nuclide::Pd101, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        //distribution_yield_101 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ag101, 
        //        neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        //distribution_yield_101 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cd101, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_101;

    }
    /// gets mass yield for A=102
    pub fn get_mass_yield_102(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_102: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 Rubidium 
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Rb102, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Sr102, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Y102, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Zr102, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Nb102, 
                neutron_energy).get::<ratio>();
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Nb102m, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Mo102, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Tc102, 
                neutron_energy).get::<ratio>();
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Tc102m, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Ru102, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Rh102, 
                neutron_energy).get::<ratio>();
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Rh102m, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_102 += 
            fission_yield(
                *self, 
                Nuclide::Pd102, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        //distribution_yield_102 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ag102, 
        //        neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        //distribution_yield_102 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cd102, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_102;

    }
    /// gets mass yield for A=103
    pub fn get_mass_yield_103(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_103: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 Rubidium 
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Rb103, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Sr103, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Y103, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Zr103, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Nb103, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Mo103, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Tc103, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Ru103, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Rh103, 
                neutron_energy).get::<ratio>();
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Rh103m, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Pd103, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_103 += 
            fission_yield(
                *self, 
                Nuclide::Ag103, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        //distribution_yield_103 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cd103, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_103;

    }
    /// gets mass yield for A=104
    pub fn get_mass_yield_104(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_104: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 Rubidium 
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Rb104, 
                neutron_energy).get::<ratio>();
        // Z=38 Strontium
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Sr104, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Y104, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Zr104, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Nb104, 
                neutron_energy).get::<ratio>();
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Nb104m, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Mo104, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Tc104, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Ru104, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Rh104, 
                neutron_energy).get::<ratio>();
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Rh104m, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_104 += 
            fission_yield(
                *self, 
                Nuclide::Pd104, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        //distribution_yield_104 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ag104, 
        //        neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        //distribution_yield_104 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cd104, 
        //        neutron_energy).get::<ratio>();

        return distribution_yield_104;

    }
    /// gets mass yield for A=105
    pub fn get_mass_yield_105(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_105: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 Strontium
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Sr105, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Y105, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Zr105, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Nb105, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Mo105, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Tc105, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Ru105, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Rh105, 
                neutron_energy).get::<ratio>();
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Rh105m, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Pd105, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Ag105, 
                neutron_energy).get::<ratio>();
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Ag105m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_105 += 
            fission_yield(
                *self, 
                Nuclide::Cd105, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        //distribution_yield_105 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::In105, 
        //        neutron_energy).get::<ratio>();


        return distribution_yield_105;
    }
    /// gets mass yield for A=106
    pub fn get_mass_yield_106(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_106: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 Strontium
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Sr106, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Y106, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Zr106, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Nb106, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Mo106, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Tc106, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Ru106, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Rh106, 
                neutron_energy).get::<ratio>();
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Rh106m, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Pd106, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Ag106, 
                neutron_energy).get::<ratio>();
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Ag106m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_106 += 
            fission_yield(
                *self, 
                Nuclide::Cd106, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        //distribution_yield_106 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::In106, 
        //        neutron_energy).get::<ratio>();
        // Z=50 Tin
        //distribution_yield_106 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sn106, 
        //        neutron_energy).get::<ratio>();


        return distribution_yield_106;
    }
    /// gets mass yield for A=107
    pub fn get_mass_yield_107(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_107: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 Strontium
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Sr107, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Y107, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Zr107, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Nb107, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Mo107, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Tc107, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Ru107, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Rh107, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Pd107, 
                neutron_energy).get::<ratio>();
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Pd107m, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Ag107, 
                neutron_energy).get::<ratio>();
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Ag107m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::Cd107, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_107 += 
            fission_yield(
                *self, 
                Nuclide::In107, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        //distribution_yield_107 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sn107, 
        //        neutron_energy).get::<ratio>();
        // Z=51 Antimony
        //distribution_yield_107 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sb107, 
        //        neutron_energy).get::<ratio>();


        return distribution_yield_107;
    }
    /// gets mass yield for A=108
    pub fn get_mass_yield_108(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_108: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 Strontium
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Sr108, 
                neutron_energy).get::<ratio>();
        // Z=39 Yttrium
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Y108, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Zr108, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Nb108, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Mo108, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Tc108, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Ru108, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Rh108, 
                neutron_energy).get::<ratio>();
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Rh108m, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Pd108, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Ag108, 
                neutron_energy).get::<ratio>();
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Ag108m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_108 += 
            fission_yield(
                *self, 
                Nuclide::Cd108, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        //distribution_yield_108 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::In108, 
        //        neutron_energy).get::<ratio>();
        // Z=50 Tin
        //distribution_yield_108 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sn108, 
        //        neutron_energy).get::<ratio>();
        // Z=51 Antimony
        //distribution_yield_108 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sb108, 
        //        neutron_energy).get::<ratio>();


        return distribution_yield_108;
    }
    /// gets mass yield for A=109
    pub fn get_mass_yield_109(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_109: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 Yttrium
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Y109, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Zr109, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Nb109, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Mo109, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Tc109, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Ru109, 
                neutron_energy).get::<ratio>();
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Ru109m, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Rh109, 
                neutron_energy).get::<ratio>();
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Rh109m, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Pd109, 
                neutron_energy).get::<ratio>();
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Pd109m, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Ag109, 
                neutron_energy).get::<ratio>();
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Ag109m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::Cd109, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_109 += 
            fission_yield(
                *self, 
                Nuclide::In109, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        //distribution_yield_109 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sn109, 
        //        neutron_energy).get::<ratio>();
        // Z=51 Antimony
        //distribution_yield_109 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sb109, 
        //        neutron_energy).get::<ratio>();


        return distribution_yield_109;
    }
    /// gets mass yield for A=110
    pub fn get_mass_yield_110(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_110: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 Yttrium
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Y110, 
                neutron_energy).get::<ratio>();
        // Z=40 Zirconium
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Zr110, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Nb110, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Mo110, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Tc110, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Ru110, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Rh110, 
                neutron_energy).get::<ratio>();
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Rh110m, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Pd110, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Ag110, 
                neutron_energy).get::<ratio>();
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Ag110m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_110 += 
            fission_yield(
                *self, 
                Nuclide::Cd110, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        //distribution_yield_110 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::In110, 
        //        neutron_energy).get::<ratio>();
        // Z=50 Tin
        //distribution_yield_110 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sn110, 
        //        neutron_energy).get::<ratio>();
        // Z=51 Antimony
        //distribution_yield_110 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sb110, 
        //        neutron_energy).get::<ratio>();


        return distribution_yield_110;
    }
    /// gets mass yield for A=111
    pub fn get_mass_yield_111(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_111: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 Zirconium
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Zr111, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Nb111, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Mo111, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Tc111, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Ru111, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Rh111, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Pd111, 
                neutron_energy).get::<ratio>();
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Pd111m, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Ag111, 
                neutron_energy).get::<ratio>();
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Ag111m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Cd111, 
                neutron_energy).get::<ratio>();
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Cd111m, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::In111, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_111 += 
            fission_yield(
                *self, 
                Nuclide::Sn111, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        //distribution_yield_111 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sb111, 
        //        neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        //distribution_yield_111 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Te111, 
        //        neutron_energy).get::<ratio>();


        return distribution_yield_111;
    }
    /// gets mass yield for A=112
    pub fn get_mass_yield_112(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_112: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 Zirconium
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::Zr112, 
                neutron_energy).get::<ratio>();
        // Z=41 Niobium
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::Nb112, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::Mo112, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::Tc112, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::Ru112, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::Rh112, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::Pd112, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::Ag112, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::Cd112, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::In112, 
                neutron_energy).get::<ratio>();
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::In112m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_112 += 
            fission_yield(
                *self, 
                Nuclide::Sn112, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        //distribution_yield_112 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sb112, 
        //        neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        //distribution_yield_112 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Te112, 
        //        neutron_energy).get::<ratio>();


        return distribution_yield_112;
    }
    /// gets mass yield for A=113
    pub fn get_mass_yield_113(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_113: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 Niobium
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Nb113, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Mo113, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Tc113, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Ru113, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Rh113, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Pd113, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Ag113, 
                neutron_energy).get::<ratio>();
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Ag113m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Cd113, 
                neutron_energy).get::<ratio>();
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Cd113m, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::In113, 
                neutron_energy).get::<ratio>();
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::In113m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Sn113, 
                neutron_energy).get::<ratio>();
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Sn113m, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_113 += 
            fission_yield(
                *self, 
                Nuclide::Sb113, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        //distribution_yield_113 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Te113, 
        //        neutron_energy).get::<ratio>();


        return distribution_yield_113;
    }
    /// gets mass yield for A=114
    pub fn get_mass_yield_114(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_114: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 Niobium
        distribution_yield_114 += 
            fission_yield(
                *self, 
                Nuclide::Nb114, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_114 += 
            fission_yield(
                *self, 
                Nuclide::Mo114, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_114 += 
            fission_yield(
                *self, 
                Nuclide::Tc114, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_114 += 
            fission_yield(
                *self, 
                Nuclide::Ru114, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_114 += 
            fission_yield(
                *self, 
                Nuclide::Rh114, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_114 += 
            fission_yield(
                *self, 
                Nuclide::Pd114, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_114 += 
            fission_yield(
                *self, 
                Nuclide::Ag114, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_114 += 
            fission_yield(
                *self, 
                Nuclide::Cd114, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_114 += 
            fission_yield(
                *self, 
                Nuclide::In114, 
                neutron_energy).get::<ratio>();
        distribution_yield_114 += 
            fission_yield(
                *self, 
                Nuclide::In114m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_114 += 
            fission_yield(
                *self, 
                Nuclide::Sn114, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        //distribution_yield_114 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sb114, 
        //        neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        //distribution_yield_114 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Te114, 
        //        neutron_energy).get::<ratio>();
        // Z=53 Iodine
        //distribution_yield_114 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::I114, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_114;
    }
    /// gets mass yield for A=115
    pub fn get_mass_yield_115(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_115: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 Niobium
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Nb115, 
                neutron_energy).get::<ratio>();
        // Z=42 Molybdenum
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Mo115, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Tc115, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Ru115, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Rh115, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Pd115, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Ag115, 
                neutron_energy).get::<ratio>();
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Ag115m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Cd115, 
                neutron_energy).get::<ratio>();
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Cd115m, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::In115, 
                neutron_energy).get::<ratio>();
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::In115m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Sn115, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Sb115, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_115 += 
            fission_yield(
                *self, 
                Nuclide::Te115, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        //distribution_yield_115 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::I115, 
        //        neutron_energy).get::<ratio>();
        // Z=54 Xe
        //distribution_yield_115 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Xe115, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_115;
    }
    /// gets mass yield for A=116
    pub fn get_mass_yield_116(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_116: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 Molybdenum
        distribution_yield_116 += 
            fission_yield(
                *self, 
                Nuclide::Mo116, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_116 += 
            fission_yield(
                *self, 
                Nuclide::Tc116, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_116 += 
            fission_yield(
                *self, 
                Nuclide::Ru116, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_116 += 
            fission_yield(
                *self, 
                Nuclide::Rh116, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_116 += 
            fission_yield(
                *self, 
                Nuclide::Pd116, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_116 += 
            fission_yield(
                *self, 
                Nuclide::Ag116, 
                neutron_energy).get::<ratio>();
        distribution_yield_116 += 
            fission_yield(
                *self, 
                Nuclide::Ag116m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_116 += 
            fission_yield(
                *self, 
                Nuclide::Cd116, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_116 += 
            fission_yield(
                *self, 
                Nuclide::In116, 
                neutron_energy).get::<ratio>();
        distribution_yield_116 += 
            fission_yield(
                *self, 
                Nuclide::In116m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_116 += 
            fission_yield(
                *self, 
                Nuclide::Sn116, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        //distribution_yield_116 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sb116, 
        //        neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        //distribution_yield_116 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Te116, 
        //        neutron_energy).get::<ratio>();
        // Z=53 Iodine
        //distribution_yield_116 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::I116, 
        //        neutron_energy).get::<ratio>();
        // Z=54 Xe
        //distribution_yield_116 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Xe116, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_116;
    }
    /// gets mass yield for A=117
    pub fn get_mass_yield_117(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_117: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 Molybdenum
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Mo117, 
                neutron_energy).get::<ratio>();
        // Z=43 Technetium
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Tc117, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Ru117, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Rh117, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Pd117, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Ag117, 
                neutron_energy).get::<ratio>();
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Ag117m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Cd117, 
                neutron_energy).get::<ratio>();
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Cd117m, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::In117, 
                neutron_energy).get::<ratio>();
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::In117m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Sn117, 
                neutron_energy).get::<ratio>();
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Sn117m, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Sb117, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_117 += 
            fission_yield(
                *self, 
                Nuclide::Te117, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        //distribution_yield_117 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::I117, 
        //        neutron_energy).get::<ratio>();
        // Z=54 Xe
        //distribution_yield_117 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Xe117, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_117;
    }
    /// gets mass yield for A=118
    pub fn get_mass_yield_118(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_118: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 Technetium
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::Tc118, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::Ru118, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::Rh118, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::Pd118, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::Ag118, 
                neutron_energy).get::<ratio>();
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::Ag118m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::Cd118, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::In118, 
                neutron_energy).get::<ratio>();
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::In118m, 
                neutron_energy).get::<ratio>();
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::In118n, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::Sn118, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::Sb118, 
                neutron_energy).get::<ratio>();
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::Sb118m, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_118 += 
            fission_yield(
                *self, 
                Nuclide::Te118, 
                neutron_energy).get::<ratio>();

        // Z=53 Iodine
        //distribution_yield_118 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::I118, 
        //        neutron_energy).get::<ratio>();
        // Z=54 Xe
        //distribution_yield_118 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Xe118, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_118;
    }
    /// gets mass yield for A=119
    pub fn get_mass_yield_119(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_119: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 Technetium
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::Tc119, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::Ru119, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::Rh119, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::Pd119, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::Ag119, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::Cd119, 
                neutron_energy).get::<ratio>();
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::Cd119m, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::In119, 
                neutron_energy).get::<ratio>();
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::In119m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::Sn119, 
                neutron_energy).get::<ratio>();
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::Sn119m, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::Sb119, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_119 += 
            fission_yield(
                *self, 
                Nuclide::Te119, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        //distribution_yield_119 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::I119, 
        //        neutron_energy).get::<ratio>();
        // Z=54 Xe
        //distribution_yield_119 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Xe119, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_119;
    }
    /// gets mass yield for A=120
    pub fn get_mass_yield_120(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_120: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 Technetium
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::Tc120, 
                neutron_energy).get::<ratio>();
        // Z=44 Ruthenium
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::Ru120, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::Rh120, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::Pd120, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::Ag120, 
                neutron_energy).get::<ratio>();
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::Ag120m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::Cd120, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::In120, 
                neutron_energy).get::<ratio>();
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::In120m, 
                neutron_energy).get::<ratio>();
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::In120n, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::Sn120, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::Sb120, 
                neutron_energy).get::<ratio>();
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::Sb120m, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_120 += 
            fission_yield(
                *self, 
                Nuclide::Te120, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        //distribution_yield_120 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::I120, 
        //        neutron_energy).get::<ratio>();
        // Z=54 Xe
        //distribution_yield_120 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Xe120, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_120;
    }
    /// gets mass yield for A=121
    pub fn get_mass_yield_121(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_121: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 Ruthenium
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::Ru121, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::Rh121, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::Pd121, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::Ag121, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::Cd121, 
                neutron_energy).get::<ratio>();
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::Cd121m, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::In121, 
                neutron_energy).get::<ratio>();
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::In121m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::Sn121, 
                neutron_energy).get::<ratio>();
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::Sn121m, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::Sb121, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::Te121, 
                neutron_energy).get::<ratio>();
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::Te121m, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_121 += 
            fission_yield(
                *self, 
                Nuclide::I121, 
                neutron_energy).get::<ratio>();
        // Z=54 Xe
        //distribution_yield_121 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Xe121, 
        //        neutron_energy).get::<ratio>();
        // Z=55 Cs
        //distribution_yield_121 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cs121, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_121;
    }
    /// gets mass yield for A=122
    pub fn get_mass_yield_122(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_122: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 Ruthenium
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::Ru122, 
                neutron_energy).get::<ratio>();

        // Z=45 Rhodium
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::Rh122, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::Pd122, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::Ag122, 
                neutron_energy).get::<ratio>();
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::Ag122m, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::Cd122, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::In122, 
                neutron_energy).get::<ratio>();
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::In122m, 
                neutron_energy).get::<ratio>();
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::In122n, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::Sn122, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::Sb122, 
                neutron_energy).get::<ratio>();
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::Sb122m, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_122 += 
            fission_yield(
                *self, 
                Nuclide::Te122, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        //distribution_yield_122 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::I122, 
        //        neutron_energy).get::<ratio>();
        // Z=54 Xe
        //distribution_yield_122 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Xe122, 
        //        neutron_energy).get::<ratio>();
        // Z=55 Cs
        //distribution_yield_122 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cs122, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_122;
    }
    /// gets mass yield for A=123
    pub fn get_mass_yield_123(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_123: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none

        // Z=45 Rhodium
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::Rh123, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::Pd123, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::Ag123, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::Cd123, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::In123, 
                neutron_energy).get::<ratio>();
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::In123m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::Sn123, 
                neutron_energy).get::<ratio>();
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::Sn123m, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::Sb123, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::Te123, 
                neutron_energy).get::<ratio>();
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::Te123m, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_123 += 
            fission_yield(
                *self, 
                Nuclide::I123, 
                neutron_energy).get::<ratio>();
        // Z=54 Xe
        //distribution_yield_123 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Xe123, 
        //        neutron_energy).get::<ratio>();
        // Z=55 Cs
        //distribution_yield_123 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cs123, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_123;
    }
    /// gets mass yield for A=124
    pub fn get_mass_yield_124(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_124: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none

        // Z=45 Rhodium
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::Rh124, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::Pd124, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::Ag124, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::Cd124, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::In124, 
                neutron_energy).get::<ratio>();
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::In124m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::Sn124, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::Sb124, 
                neutron_energy).get::<ratio>();
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::Sb124m, 
                neutron_energy).get::<ratio>();
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::Sb124n, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::Te124, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        //distribution_yield_124 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::I124, 
        //        neutron_energy).get::<ratio>();
        // Z=54 Xe
        distribution_yield_124 += 
            fission_yield(
                *self, 
                Nuclide::Xe124, 
                neutron_energy).get::<ratio>();
        // Z=55 Cs
        //distribution_yield_124 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cs124, 
        //        neutron_energy).get::<ratio>();
        // Z=56 Ba
        //distribution_yield_124 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ba124, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_124;
    }
    /// gets mass yield for A=125
    pub fn get_mass_yield_125(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_125: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none

        // Z=45 Rhodium
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::Rh125, 
                neutron_energy).get::<ratio>();
        // Z=46 Paladium
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::Pd125, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::Ag125, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::Cd125, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::In125, 
                neutron_energy).get::<ratio>();
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::In125m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::Sn125, 
                neutron_energy).get::<ratio>();
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::Sn125m, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::Sb125, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::Te125, 
                neutron_energy).get::<ratio>();
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::Te125m, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::I125, 
                neutron_energy).get::<ratio>();
        // Z=54 Xe
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::Xe125, 
                neutron_energy).get::<ratio>();
        distribution_yield_125 += 
            fission_yield(
                *self, 
                Nuclide::Xe125m, 
                neutron_energy).get::<ratio>();
        // Z=55 Cs
        //distribution_yield_125 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cs125, 
        //        neutron_energy).get::<ratio>();
        // Z=56 Ba
        //distribution_yield_125 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ba125, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_125;
    }
    /// gets mass yield for A=126
    pub fn get_mass_yield_126(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_126: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none
        // Z=45 (Rhodium) -- none
        // Z=46 Paladium
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::Pd126, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::Ag126, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::Cd126, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::In126, 
                neutron_energy).get::<ratio>();
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::In126m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::Sn126, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::Sb126, 
                neutron_energy).get::<ratio>();
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::Sb126m, 
                neutron_energy).get::<ratio>();
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::Sb126n, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::Te126, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::I126, 
                neutron_energy).get::<ratio>();
        // Z=54 Xe
        distribution_yield_126 += 
            fission_yield(
                *self, 
                Nuclide::Xe126, 
                neutron_energy).get::<ratio>();
        // Z=55 Cs
        //distribution_yield_126 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cs126, 
        //        neutron_energy).get::<ratio>();
        // Z=56 Ba
        //distribution_yield_126 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ba126, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_126;
    }
    /// gets mass yield for A=127
    pub fn get_mass_yield_127(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_127: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none
        // Z=45 (Rhodium) -- none
        // Z=46 Paladium
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::Pd127, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::Ag127, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::Cd127, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::In127, 
                neutron_energy).get::<ratio>();
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::In127m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::Sn127, 
                neutron_energy).get::<ratio>();
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::Sn127m, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::Sb127, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::Te127, 
                neutron_energy).get::<ratio>();
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::Te127m, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::I127, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::Xe127, 
                neutron_energy).get::<ratio>();
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::Xe127m, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_127 += 
            fission_yield(
                *self, 
                Nuclide::Cs127, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        //distribution_yield_127 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ba127, 
        //        neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        //distribution_yield_127 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::La127, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_127;
    }
    /// gets mass yield for A=128
    pub fn get_mass_yield_128(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_128: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none
        // Z=45 (Rhodium) -- none
        // Z=46 Paladium
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::Pd128, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::Ag128, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::Cd128, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::In128, 
                neutron_energy).get::<ratio>();
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::In128m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::Sn128, 
                neutron_energy).get::<ratio>();
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::Sn128m, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::Sb128, 
                neutron_energy).get::<ratio>();
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::Sb128m, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::Te128, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::I128, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_128 += 
            fission_yield(
                *self, 
                Nuclide::Xe128, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        //distribution_yield_128 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cs128, 
        //        neutron_energy).get::<ratio>();
        // Z=56 Barium
        //distribution_yield_128 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ba128, 
        //        neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        //distribution_yield_128 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::La128, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_128;
    }
    /// gets mass yield for A=129
    pub fn get_mass_yield_129(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_129: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none
        // Z=45 (Rhodium) -- none
        // Z=46 Paladium
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Pd129, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Ag129, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Cd129, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::In129, 
                neutron_energy).get::<ratio>();
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::In129m, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Sn129, 
                neutron_energy).get::<ratio>();
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Sn129m, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Sb129, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Te129, 
                neutron_energy).get::<ratio>();
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Te129m, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::I129, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Xe129, 
                neutron_energy).get::<ratio>();
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Xe129m, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Cs129, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_129 += 
            fission_yield(
                *self, 
                Nuclide::Ba129, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        //distribution_yield_129 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::La129, 
        //        neutron_energy).get::<ratio>();
        // Z=58 Cerium
        //distribution_yield_129 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ce129, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_129;
    }
    /// gets mass yield for A=130
    pub fn get_mass_yield_130(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_130: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none
        // Z=45 (Rhodium) -- none
        // Z=46 Paladium
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::Pd130, 
                neutron_energy).get::<ratio>();
        // Z=47 Silver
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::Ag130, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::Cd130, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::In130, 
                neutron_energy).get::<ratio>();
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::In130m, 
                neutron_energy).get::<ratio>();
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::In130n, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::Sn130, 
                neutron_energy).get::<ratio>();
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::Sn130m, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::Sb130, 
                neutron_energy).get::<ratio>();
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::Sb130m, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::Te130, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::I130, 
                neutron_energy).get::<ratio>();
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::I130m, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_130 += 
            fission_yield(
                *self, 
                Nuclide::Xe130, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        //distribution_yield_130 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Cs130, 
        //        neutron_energy).get::<ratio>();
        // Z=56 Barium
        //distribution_yield_130 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ba130, 
        //        neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        //distribution_yield_130 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::La130, 
        //        neutron_energy).get::<ratio>();
        // Z=58 Cerium
        //distribution_yield_130 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ce130, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_130;
    }
    /// gets mass yield for A=131
    pub fn get_mass_yield_131(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_131: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none
        // Z=45 (Rhodium) -- none
        // Z=46 (Paladium) -- none
        // Z=47 Silver
        distribution_yield_131 += 
            fission_yield(
                *self, 
                Nuclide::Ag131, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_131 += 
            fission_yield(
                *self, 
                Nuclide::Cd131, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_131 += 
            fission_yield(
                *self, 
                Nuclide::In131, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_131 += 
            fission_yield(
                *self, 
                Nuclide::Sn131, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_131 += 
            fission_yield(
                *self, 
                Nuclide::Sb131, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_131 += 
            fission_yield(
                *self, 
                Nuclide::Te131, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_131 += 
            fission_yield(
                *self, 
                Nuclide::I131, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_131 += 
            fission_yield(
                *self, 
                Nuclide::Xe131, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_131 += 
            fission_yield(
                *self, 
                Nuclide::Cs131, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_131 += 
            fission_yield(
                *self, 
                Nuclide::Ba131, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        //distribution_yield_131 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::La131, 
        //        neutron_energy).get::<ratio>();
        // Z=58 Cerium
        //distribution_yield_131 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ce131, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_131;
    }
    /// gets mass yield for A=132
    pub fn get_mass_yield_132(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_132: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none
        // Z=45 (Rhodium) -- none
        // Z=46 (Paladium) -- none
        // Z=47 Silver
        distribution_yield_132 += 
            fission_yield(
                *self, 
                Nuclide::Ag132, 
                neutron_energy).get::<ratio>();
        // Z=48 Cadmium
        distribution_yield_132 += 
            fission_yield(
                *self, 
                Nuclide::Cd132, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_132 += 
            fission_yield(
                *self, 
                Nuclide::In132, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_132 += 
            fission_yield(
                *self, 
                Nuclide::Sn132, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_132 += 
            fission_yield(
                *self, 
                Nuclide::Sb132, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_132 += 
            fission_yield(
                *self, 
                Nuclide::Te132, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_132 += 
            fission_yield(
                *self, 
                Nuclide::I132, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_132 += 
            fission_yield(
                *self, 
                Nuclide::Xe132, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_132 += 
            fission_yield(
                *self, 
                Nuclide::Cs132, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_132 += 
            fission_yield(
                *self, 
                Nuclide::Ba132, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        //distribution_yield_132 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::La132, 
        //        neutron_energy).get::<ratio>();
        // Z=58 Cerium
        //distribution_yield_132 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ce132, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_132;
    }
    /// gets mass yield for A=133
    pub fn get_mass_yield_133(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_133: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none
        // Z=45 (Rhodium) -- none
        // Z=46 (Paladium) -- none
        // Z=47 (Silver) -- none
        // Z=48 Cadmium
        distribution_yield_133 += 
            fission_yield(
                *self, 
                Nuclide::Cd133, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_133 += 
            fission_yield(
                *self, 
                Nuclide::In133, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_133 += 
            fission_yield(
                *self, 
                Nuclide::Sn133, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_133 += 
            fission_yield(
                *self, 
                Nuclide::Sb133, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_133 += 
            fission_yield(
                *self, 
                Nuclide::Te133, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_133 += 
            fission_yield(
                *self, 
                Nuclide::I133, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_133 += 
            fission_yield(
                *self, 
                Nuclide::Xe133, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_133 += 
            fission_yield(
                *self, 
                Nuclide::Cs133, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_133 += 
            fission_yield(
                *self, 
                Nuclide::Ba133, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_133 += 
            fission_yield(
                *self, 
                Nuclide::La133, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        //distribution_yield_133 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ce133, 
        //        neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        //distribution_yield_133 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Pr133, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_133;
    }
    /// gets mass yield for A=134
    pub fn get_mass_yield_134(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_134: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none
        // Z=45 (Rhodium) -- none
        // Z=46 (Paladium) -- none
        // Z=47 (Silver) -- none
        // Z=48 Cadmium
        distribution_yield_134 += 
            fission_yield(
                *self, 
                Nuclide::Cd134, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_134 += 
            fission_yield(
                *self, 
                Nuclide::In134, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_134 += 
            fission_yield(
                *self, 
                Nuclide::Sn134, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_134 += 
            fission_yield(
                *self, 
                Nuclide::Sb134, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_134 += 
            fission_yield(
                *self, 
                Nuclide::Te134, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_134 += 
            fission_yield(
                *self, 
                Nuclide::I134, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_134 += 
            fission_yield(
                *self, 
                Nuclide::Xe134, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_134 += 
            fission_yield(
                *self, 
                Nuclide::Cs134, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_134 += 
            fission_yield(
                *self, 
                Nuclide::Ba134, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        //distribution_yield_134 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::La134, 
        //        neutron_energy).get::<ratio>();
        // Z=58 Cerium
        //distribution_yield_134 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ce134, 
        //        neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        //distribution_yield_134 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Pr134, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_134;
    }
    /// gets mass yield for A=135
    pub fn get_mass_yield_135(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_135: f64 = 0.0;
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
        // Z=36 (Krypton) -- none
        // Z=37 (Rubidium) -- none
        // Z=38 (Strontium) -- none
        // Z=39 (Yttrium) -- none
        // Z=40 (Zirconium) -- none
        // Z=41 (Niobium) -- none
        // Z=42 (Molybdenum) -- none
        // Z=43 (Technetium) -- none
        // Z=44 (Ruthenium) -- none
        // Z=45 (Rhodium) -- none
        // Z=46 (Paladium) -- none
        // Z=47 (Silver) -- none
        // Z=48 Cadmium
        distribution_yield_135 += 
            fission_yield(
                *self, 
                Nuclide::Cd135, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_135 += 
            fission_yield(
                *self, 
                Nuclide::In135, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_135 += 
            fission_yield(
                *self, 
                Nuclide::Sn135, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_135 += 
            fission_yield(
                *self, 
                Nuclide::Sb135, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_135 += 
            fission_yield(
                *self, 
                Nuclide::Te135, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_135 += 
            fission_yield(
                *self, 
                Nuclide::I135, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_135 += 
            fission_yield(
                *self, 
                Nuclide::Xe135, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_135 += 
            fission_yield(
                *self, 
                Nuclide::Cs135, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_135 += 
            fission_yield(
                *self, 
                Nuclide::Ba135, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_135 += 
            fission_yield(
                *self, 
                Nuclide::La135, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_135 += 
            fission_yield(
                *self, 
                Nuclide::Ce135, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        //distribution_yield_135 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Pr135, 
        //        neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        //distribution_yield_135 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Nd135, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_135;
    }
}



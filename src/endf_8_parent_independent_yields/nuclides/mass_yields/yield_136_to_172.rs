use crate::{endf_8_parent_independent_yields::nuclides::Nuclide, prelude::fission_yield};
use uom::si::{f64::*, ratio::ratio};
impl Nuclide {

    /// gets mass yield for A=136
    pub fn get_mass_yield_136(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_136: f64 = 0.0;
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
        distribution_yield_136 += 
            fission_yield(
                *self, 
                Nuclide::Cd136, 
                neutron_energy).get::<ratio>();
        // Z=49 Indium 
        distribution_yield_136 += 
            fission_yield(
                *self, 
                Nuclide::In136, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_136 += 
            fission_yield(
                *self, 
                Nuclide::Sn136, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_136 += 
            fission_yield(
                *self, 
                Nuclide::Sb136, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_136 += 
            fission_yield(
                *self, 
                Nuclide::Te136, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_136 += 
            fission_yield(
                *self, 
                Nuclide::I136, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_136 += 
            fission_yield(
                *self, 
                Nuclide::Xe136, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_136 += 
            fission_yield(
                *self, 
                Nuclide::Cs136, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_136 += 
            fission_yield(
                *self, 
                Nuclide::Ba136, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        //distribution_yield_136 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::La136, 
        //        neutron_energy).get::<ratio>();
        // Z=58 Cerium
        //distribution_yield_136 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ce136, 
        //        neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        //distribution_yield_136 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Pr136, 
        //        neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        //distribution_yield_136 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Nd136, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_136;
    }
    /// gets mass yield for A=137
    pub fn get_mass_yield_137(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_137: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 Indium 
        distribution_yield_137 += 
            fission_yield(
                *self, 
                Nuclide::In137, 
                neutron_energy).get::<ratio>();
        // Z=50 Tin
        distribution_yield_137 += 
            fission_yield(
                *self, 
                Nuclide::Sn137, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_137 += 
            fission_yield(
                *self, 
                Nuclide::Sb137, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_137 += 
            fission_yield(
                *self, 
                Nuclide::Te137, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_137 += 
            fission_yield(
                *self, 
                Nuclide::I137, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_137 += 
            fission_yield(
                *self, 
                Nuclide::Xe137, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_137 += 
            fission_yield(
                *self, 
                Nuclide::Cs137, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_137 += 
            fission_yield(
                *self, 
                Nuclide::Ba137, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_137 += 
            fission_yield(
                *self, 
                Nuclide::La137, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_137 += 
            fission_yield(
                *self, 
                Nuclide::Ce137, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        //distribution_yield_137 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Pr137, 
        //        neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        //distribution_yield_137 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Nd137, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_137;
    }
    /// gets mass yield for A=138
    pub fn get_mass_yield_138(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_138: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 Tin
        distribution_yield_138 += 
            fission_yield(
                *self, 
                Nuclide::Sn138, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_138 += 
            fission_yield(
                *self, 
                Nuclide::Sb138, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_138 += 
            fission_yield(
                *self, 
                Nuclide::Te138, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_138 += 
            fission_yield(
                *self, 
                Nuclide::I138, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_138 += 
            fission_yield(
                *self, 
                Nuclide::Xe138, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_138 += 
            fission_yield(
                *self, 
                Nuclide::Cs138, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_138 += 
            fission_yield(
                *self, 
                Nuclide::Ba138, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_138 += 
            fission_yield(
                *self, 
                Nuclide::La138, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_138 += 
            fission_yield(
                *self, 
                Nuclide::Ce138, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        //distribution_yield_138 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Pr138, 
        //        neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        //distribution_yield_138 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Nd138, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_138;
    }
    /// gets mass yield for A=139
    pub fn get_mass_yield_139(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_139: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 Tin
        distribution_yield_139 += 
            fission_yield(
                *self, 
                Nuclide::Sn139, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_139 += 
            fission_yield(
                *self, 
                Nuclide::Sb139, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_139 += 
            fission_yield(
                *self, 
                Nuclide::Te139, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_139 += 
            fission_yield(
                *self, 
                Nuclide::I139, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_139 += 
            fission_yield(
                *self, 
                Nuclide::Xe139, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_139 += 
            fission_yield(
                *self, 
                Nuclide::Cs139, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_139 += 
            fission_yield(
                *self, 
                Nuclide::Ba139, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_139 += 
            fission_yield(
                *self, 
                Nuclide::La139, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_139 += 
            fission_yield(
                *self, 
                Nuclide::Ce139, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_139 += 
            fission_yield(
                *self, 
                Nuclide::Pr139, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        //distribution_yield_139 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Nd139, 
        //        neutron_energy).get::<ratio>();
        // Z=61 Promethium
        //distribution_yield_139 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Pm139, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_139;
    }
    /// gets mass yield for A=140
    pub fn get_mass_yield_140(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_140: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 Tin
        distribution_yield_140 += 
            fission_yield(
                *self, 
                Nuclide::Sn140, 
                neutron_energy).get::<ratio>();
        // Z=51 Antimony
        distribution_yield_140 += 
            fission_yield(
                *self, 
                Nuclide::Sb140, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_140 += 
            fission_yield(
                *self, 
                Nuclide::Te140, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_140 += 
            fission_yield(
                *self, 
                Nuclide::I140, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_140 += 
            fission_yield(
                *self, 
                Nuclide::Xe140, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_140 += 
            fission_yield(
                *self, 
                Nuclide::Cs140, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_140 += 
            fission_yield(
                *self, 
                Nuclide::Ba140, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_140 += 
            fission_yield(
                *self, 
                Nuclide::La140, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_140 += 
            fission_yield(
                *self, 
                Nuclide::Ce140, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_140 += 
            fission_yield(
                *self, 
                Nuclide::Pr140, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_140 += 
            fission_yield(
                *self, 
                Nuclide::Nd140, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        //distribution_yield_140 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Pm140, 
        //        neutron_energy).get::<ratio>();
        // Z=62 Samarium
        //distribution_yield_140 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sm140, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_140;
    }
    /// gets mass yield for A=141
    pub fn get_mass_yield_141(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_141: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 Antimony
        distribution_yield_141 += 
            fission_yield(
                *self, 
                Nuclide::Sb141, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_141 += 
            fission_yield(
                *self, 
                Nuclide::Te141, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_141 += 
            fission_yield(
                *self, 
                Nuclide::I141, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_141 += 
            fission_yield(
                *self, 
                Nuclide::Xe141, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_141 += 
            fission_yield(
                *self, 
                Nuclide::Cs141, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_141 += 
            fission_yield(
                *self, 
                Nuclide::Ba141, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_141 += 
            fission_yield(
                *self, 
                Nuclide::La141, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_141 += 
            fission_yield(
                *self, 
                Nuclide::Ce141, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_141 += 
            fission_yield(
                *self, 
                Nuclide::Pr141, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_141 += 
            fission_yield(
                *self, 
                Nuclide::Nd141, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_141 += 
            fission_yield(
                *self, 
                Nuclide::Pm141, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        //distribution_yield_141 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sm141, 
        //        neutron_energy).get::<ratio>();
        // Z=63 Europium
        //distribution_yield_141 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Eu141, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_141;
    }
    /// gets mass yield for A=142
    pub fn get_mass_yield_142(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_142: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 Antimony
        distribution_yield_142 += 
            fission_yield(
                *self, 
                Nuclide::Sb142, 
                neutron_energy).get::<ratio>();
        // Z=52 Tellurium
        distribution_yield_142 += 
            fission_yield(
                *self, 
                Nuclide::Te142, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_142 += 
            fission_yield(
                *self, 
                Nuclide::I142, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_142 += 
            fission_yield(
                *self, 
                Nuclide::Xe142, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_142 += 
            fission_yield(
                *self, 
                Nuclide::Cs142, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_142 += 
            fission_yield(
                *self, 
                Nuclide::Ba142, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_142 += 
            fission_yield(
                *self, 
                Nuclide::La142, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_142 += 
            fission_yield(
                *self, 
                Nuclide::Ce142, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_142 += 
            fission_yield(
                *self, 
                Nuclide::Pr142, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_142 += 
            fission_yield(
                *self, 
                Nuclide::Nd142, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        //distribution_yield_142 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Pm142, 
        //        neutron_energy).get::<ratio>();
        // Z=62 Samarium
        //distribution_yield_142 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Sm142, 
        //        neutron_energy).get::<ratio>();
        // Z=63 Europium
        //distribution_yield_142 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Eu142, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_142;
    }
    /// gets mass yield for A=143
    pub fn get_mass_yield_143(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_143: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 Tellurium
        distribution_yield_143 += 
            fission_yield(
                *self, 
                Nuclide::Te143, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_143 += 
            fission_yield(
                *self, 
                Nuclide::I143, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_143 += 
            fission_yield(
                *self, 
                Nuclide::Xe143, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_143 += 
            fission_yield(
                *self, 
                Nuclide::Cs143, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_143 += 
            fission_yield(
                *self, 
                Nuclide::Ba143, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_143 += 
            fission_yield(
                *self, 
                Nuclide::La143, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_143 += 
            fission_yield(
                *self, 
                Nuclide::Ce143, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_143 += 
            fission_yield(
                *self, 
                Nuclide::Pr143, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_143 += 
            fission_yield(
                *self, 
                Nuclide::Nd143, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_143 += 
            fission_yield(
                *self, 
                Nuclide::Pm143, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_143 += 
            fission_yield(
                *self, 
                Nuclide::Sm143, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        //distribution_yield_143 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Eu143, 
        //        neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        //distribution_yield_143 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Gd143, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_143;
    }
    /// gets mass yield for A=144
    pub fn get_mass_yield_144(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_144: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 Tellurium
        distribution_yield_144 += 
            fission_yield(
                *self, 
                Nuclide::Te144, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_144 += 
            fission_yield(
                *self, 
                Nuclide::I144, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_144 += 
            fission_yield(
                *self, 
                Nuclide::Xe144, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_144 += 
            fission_yield(
                *self, 
                Nuclide::Cs144, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_144 += 
            fission_yield(
                *self, 
                Nuclide::Ba144, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_144 += 
            fission_yield(
                *self, 
                Nuclide::La144, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_144 += 
            fission_yield(
                *self, 
                Nuclide::Ce144, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_144 += 
            fission_yield(
                *self, 
                Nuclide::Pr144, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_144 += 
            fission_yield(
                *self, 
                Nuclide::Nd144, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_144 += 
            fission_yield(
                *self, 
                Nuclide::Pm144, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_144 += 
            fission_yield(
                *self, 
                Nuclide::Sm144, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        //distribution_yield_144 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Eu144, 
        //        neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        //distribution_yield_144 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Gd144, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_144;
    }
    /// gets mass yield for A=145
    pub fn get_mass_yield_145(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_145: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 Tellurium
        distribution_yield_145 += 
            fission_yield(
                *self, 
                Nuclide::Te145, 
                neutron_energy).get::<ratio>();
        // Z=53 Iodine
        distribution_yield_145 += 
            fission_yield(
                *self, 
                Nuclide::I145, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_145 += 
            fission_yield(
                *self, 
                Nuclide::Xe145, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_145 += 
            fission_yield(
                *self, 
                Nuclide::Cs145, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_145 += 
            fission_yield(
                *self, 
                Nuclide::Ba145, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_145 += 
            fission_yield(
                *self, 
                Nuclide::La145, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_145 += 
            fission_yield(
                *self, 
                Nuclide::Ce145, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_145 += 
            fission_yield(
                *self, 
                Nuclide::Pr145, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_145 += 
            fission_yield(
                *self, 
                Nuclide::Nd145, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_145 += 
            fission_yield(
                *self, 
                Nuclide::Pm145, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_145 += 
            fission_yield(
                *self, 
                Nuclide::Sm145, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        //distribution_yield_145 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Eu145, 
        //        neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        //distribution_yield_145 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Gd145, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_145;
    }
    /// gets mass yield for A=146
    pub fn get_mass_yield_146(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_146: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 Iodine
        distribution_yield_146 += 
            fission_yield(
                *self, 
                Nuclide::I146, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_146 += 
            fission_yield(
                *self, 
                Nuclide::Xe146, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_146 += 
            fission_yield(
                *self, 
                Nuclide::Cs146, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_146 += 
            fission_yield(
                *self, 
                Nuclide::Ba146, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_146 += 
            fission_yield(
                *self, 
                Nuclide::La146, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_146 += 
            fission_yield(
                *self, 
                Nuclide::Ce146, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_146 += 
            fission_yield(
                *self, 
                Nuclide::Pr146, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_146 += 
            fission_yield(
                *self, 
                Nuclide::Nd146, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_146 += 
            fission_yield(
                *self, 
                Nuclide::Pm146, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_146 += 
            fission_yield(
                *self, 
                Nuclide::Sm146, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        //distribution_yield_146 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Eu146, 
        //        neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        //distribution_yield_146 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Gd146, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_146;
    }
    /// gets mass yield for A=147
    pub fn get_mass_yield_147(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_147: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 Iodine
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::I147, 
                neutron_energy).get::<ratio>();
        // Z=54 Xenon
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::Xe147, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::Cs147, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::Ba147, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::La147, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::Ce147, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::Pr147, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::Nd147, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::Pm147, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::Sm147, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::Eu147, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_147 += 
            fission_yield(
                *self, 
                Nuclide::Gd147, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        //distribution_yield_147 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tb147, 
        //        neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        //distribution_yield_147 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Dy147, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_147;
    }
    /// gets mass yield for A=148
    pub fn get_mass_yield_148(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_148: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 Xenon
        distribution_yield_148 += 
            fission_yield(
                *self, 
                Nuclide::Xe148, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_148 += 
            fission_yield(
                *self, 
                Nuclide::Cs148, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_148 += 
            fission_yield(
                *self, 
                Nuclide::Ba148, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_148 += 
            fission_yield(
                *self, 
                Nuclide::La148, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_148 += 
            fission_yield(
                *self, 
                Nuclide::Ce148, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_148 += 
            fission_yield(
                *self, 
                Nuclide::Pr148, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_148 += 
            fission_yield(
                *self, 
                Nuclide::Nd148, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_148 += 
            fission_yield(
                *self, 
                Nuclide::Pm148, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_148 += 
            fission_yield(
                *self, 
                Nuclide::Sm148, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        //distribution_yield_148 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Eu148, 
        //        neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        //distribution_yield_148 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Gd148, 
        //        neutron_energy).get::<ratio>();
        // Z=65 Terbium
        //distribution_yield_148 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tb148, 
        //        neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        //distribution_yield_148 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Dy148, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_148;
    }
    /// gets mass yield for A=149
    pub fn get_mass_yield_149(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_149: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 Xenon
        distribution_yield_149 += 
            fission_yield(
                *self, 
                Nuclide::Xe149, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_149 += 
            fission_yield(
                *self, 
                Nuclide::Cs149, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_149 += 
            fission_yield(
                *self, 
                Nuclide::Ba149, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_149 += 
            fission_yield(
                *self, 
                Nuclide::La149, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_149 += 
            fission_yield(
                *self, 
                Nuclide::Ce149, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_149 += 
            fission_yield(
                *self, 
                Nuclide::Pr149, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_149 += 
            fission_yield(
                *self, 
                Nuclide::Nd149, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_149 += 
            fission_yield(
                *self, 
                Nuclide::Pm149, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_149 += 
            fission_yield(
                *self, 
                Nuclide::Sm149, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_149 += 
            fission_yield(
                *self, 
                Nuclide::Eu149, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_149 += 
            fission_yield(
                *self, 
                Nuclide::Gd149, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        //distribution_yield_149 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tb149, 
        //        neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        //distribution_yield_149 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Dy149, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_149;
    }
    /// gets mass yield for A=150
    pub fn get_mass_yield_150(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_150: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 Xenon
        distribution_yield_150 += 
            fission_yield(
                *self, 
                Nuclide::Xe150, 
                neutron_energy).get::<ratio>();
        // Z=55 Cesium
        distribution_yield_150 += 
            fission_yield(
                *self, 
                Nuclide::Cs150, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_150 += 
            fission_yield(
                *self, 
                Nuclide::Ba150, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_150 += 
            fission_yield(
                *self, 
                Nuclide::La150, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_150 += 
            fission_yield(
                *self, 
                Nuclide::Ce150, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_150 += 
            fission_yield(
                *self, 
                Nuclide::Pr150, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_150 += 
            fission_yield(
                *self, 
                Nuclide::Nd150, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_150 += 
            fission_yield(
                *self, 
                Nuclide::Pm150, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_150 += 
            fission_yield(
                *self, 
                Nuclide::Sm150, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        //distribution_yield_150 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Eu150, 
        //        neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        //distribution_yield_150 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Gd150, 
        //        neutron_energy).get::<ratio>();
        // Z=65 Terbium
        //distribution_yield_150 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tb150, 
        //        neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        //distribution_yield_150 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Dy150, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_150;
    }
    /// gets mass yield for A=151
    pub fn get_mass_yield_151(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_151: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 Cesium
        distribution_yield_151 += 
            fission_yield(
                *self, 
                Nuclide::Cs151, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_151 += 
            fission_yield(
                *self, 
                Nuclide::Ba151, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_151 += 
            fission_yield(
                *self, 
                Nuclide::La151, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_151 += 
            fission_yield(
                *self, 
                Nuclide::Ce151, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_151 += 
            fission_yield(
                *self, 
                Nuclide::Pr151, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_151 += 
            fission_yield(
                *self, 
                Nuclide::Nd151, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_151 += 
            fission_yield(
                *self, 
                Nuclide::Pm151, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_151 += 
            fission_yield(
                *self, 
                Nuclide::Sm151, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_151 += 
            fission_yield(
                *self, 
                Nuclide::Eu151, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_151 += 
            fission_yield(
                *self, 
                Nuclide::Gd151, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_151 += 
            fission_yield(
                *self, 
                Nuclide::Tb151, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        //distribution_yield_151 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Dy151, 
        //        neutron_energy).get::<ratio>();
        // Z=67 Holonium
        //distribution_yield_151 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ho151, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_151;
    }
    /// gets mass yield for A=152
    pub fn get_mass_yield_152(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_152: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 Cesium
        distribution_yield_152 += 
            fission_yield(
                *self, 
                Nuclide::Cs152, 
                neutron_energy).get::<ratio>();
        // Z=56 Barium
        distribution_yield_152 += 
            fission_yield(
                *self, 
                Nuclide::Ba152, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_152 += 
            fission_yield(
                *self, 
                Nuclide::La152, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_152 += 
            fission_yield(
                *self, 
                Nuclide::Ce152, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_152 += 
            fission_yield(
                *self, 
                Nuclide::Pr152, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_152 += 
            fission_yield(
                *self, 
                Nuclide::Nd152, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_152 += 
            fission_yield(
                *self, 
                Nuclide::Pm152, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_152 += 
            fission_yield(
                *self, 
                Nuclide::Sm152, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_152 += 
            fission_yield(
                *self, 
                Nuclide::Eu152, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_152 += 
            fission_yield(
                *self, 
                Nuclide::Gd152, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        //distribution_yield_152 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tb152, 
        //        neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        //distribution_yield_152 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Dy152, 
        //        neutron_energy).get::<ratio>();
        // Z=67 Holonium
        //distribution_yield_152 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ho152, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_152;
    }
    /// gets mass yield for A=153
    pub fn get_mass_yield_153(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_153: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 Barium
        distribution_yield_153 += 
            fission_yield(
                *self, 
                Nuclide::Ba153, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_153 += 
            fission_yield(
                *self, 
                Nuclide::La153, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_153 += 
            fission_yield(
                *self, 
                Nuclide::Ce153, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_153 += 
            fission_yield(
                *self, 
                Nuclide::Pr153, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_153 += 
            fission_yield(
                *self, 
                Nuclide::Nd153, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_153 += 
            fission_yield(
                *self, 
                Nuclide::Pm153, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_153 += 
            fission_yield(
                *self, 
                Nuclide::Sm153, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_153 += 
            fission_yield(
                *self, 
                Nuclide::Eu153, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_153 += 
            fission_yield(
                *self, 
                Nuclide::Gd153, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_153 += 
            fission_yield(
                *self, 
                Nuclide::Tb153, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        //distribution_yield_153 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Dy153, 
        //        neutron_energy).get::<ratio>();
        // Z=67 Holonium
        //distribution_yield_153 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ho153, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_153;
    }
    /// gets mass yield for A=154
    pub fn get_mass_yield_154(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_154: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 Barium
        distribution_yield_154 += 
            fission_yield(
                *self, 
                Nuclide::Ba154, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_154 += 
            fission_yield(
                *self, 
                Nuclide::La154, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_154 += 
            fission_yield(
                *self, 
                Nuclide::Ce154, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_154 += 
            fission_yield(
                *self, 
                Nuclide::Pr154, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_154 += 
            fission_yield(
                *self, 
                Nuclide::Nd154, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_154 += 
            fission_yield(
                *self, 
                Nuclide::Pm154, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_154 += 
            fission_yield(
                *self, 
                Nuclide::Sm154, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_154 += 
            fission_yield(
                *self, 
                Nuclide::Eu154, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_154 += 
            fission_yield(
                *self, 
                Nuclide::Gd154, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        //distribution_yield_154 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tb154, 
        //        neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        //distribution_yield_154 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Dy154, 
        //        neutron_energy).get::<ratio>();
        // Z=67 Holonium
        //distribution_yield_154 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ho154, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_154;
    }
    /// gets mass yield for A=155
    pub fn get_mass_yield_155(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_155: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 Barium
        distribution_yield_155 += 
            fission_yield(
                *self, 
                Nuclide::Ba155, 
                neutron_energy).get::<ratio>();
        // Z=57 Lathanium
        distribution_yield_155 += 
            fission_yield(
                *self, 
                Nuclide::La155, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_155 += 
            fission_yield(
                *self, 
                Nuclide::Ce155, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_155 += 
            fission_yield(
                *self, 
                Nuclide::Pr155, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_155 += 
            fission_yield(
                *self, 
                Nuclide::Nd155, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_155 += 
            fission_yield(
                *self, 
                Nuclide::Pm155, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_155 += 
            fission_yield(
                *self, 
                Nuclide::Sm155, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_155 += 
            fission_yield(
                *self, 
                Nuclide::Eu155, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_155 += 
            fission_yield(
                *self, 
                Nuclide::Gd155, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_155 += 
            fission_yield(
                *self, 
                Nuclide::Tb155, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_155 += 
            fission_yield(
                *self, 
                Nuclide::Dy155, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        //distribution_yield_155 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ho155, 
        //        neutron_energy).get::<ratio>();
        // Z=68 Erbium
        //distribution_yield_155 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Er155, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_155;
    }
    /// gets mass yield for A=156
    pub fn get_mass_yield_156(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_156: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 Lathanium
        distribution_yield_156 += 
            fission_yield(
                *self, 
                Nuclide::La156, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_156 += 
            fission_yield(
                *self, 
                Nuclide::Ce156, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_156 += 
            fission_yield(
                *self, 
                Nuclide::Pr156, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_156 += 
            fission_yield(
                *self, 
                Nuclide::Nd156, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_156 += 
            fission_yield(
                *self, 
                Nuclide::Pm156, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_156 += 
            fission_yield(
                *self, 
                Nuclide::Sm156, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_156 += 
            fission_yield(
                *self, 
                Nuclide::Eu156, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_156 += 
            fission_yield(
                *self, 
                Nuclide::Gd156, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_156 += 
            fission_yield(
                *self, 
                Nuclide::Tb156, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_156 += 
            fission_yield(
                *self, 
                Nuclide::Dy156, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        //distribution_yield_156 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ho156, 
        //        neutron_energy).get::<ratio>();
        // Z=68 Erbium
        //distribution_yield_156 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Er156, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_156;
    }
    /// gets mass yield for A=157
    pub fn get_mass_yield_157(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_157: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 Lathanium
        distribution_yield_157 += 
            fission_yield(
                *self, 
                Nuclide::La157, 
                neutron_energy).get::<ratio>();
        // Z=58 Cerium
        distribution_yield_157 += 
            fission_yield(
                *self, 
                Nuclide::Ce157, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_157 += 
            fission_yield(
                *self, 
                Nuclide::Pr157, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_157 += 
            fission_yield(
                *self, 
                Nuclide::Nd157, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_157 += 
            fission_yield(
                *self, 
                Nuclide::Pm157, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_157 += 
            fission_yield(
                *self, 
                Nuclide::Sm157, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_157 += 
            fission_yield(
                *self, 
                Nuclide::Eu157, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_157 += 
            fission_yield(
                *self, 
                Nuclide::Gd157, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_157 += 
            fission_yield(
                *self, 
                Nuclide::Tb157, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_157 += 
            fission_yield(
                *self, 
                Nuclide::Dy157, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        //distribution_yield_157 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ho157, 
        //        neutron_energy).get::<ratio>();
        // Z=68 Erbium
        //distribution_yield_157 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Er157, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_157;
    }
    /// gets mass yield for A=158
    pub fn get_mass_yield_158(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_158: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 Cerium
        distribution_yield_158 += 
            fission_yield(
                *self, 
                Nuclide::Ce158, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_158 += 
            fission_yield(
                *self, 
                Nuclide::Pr158, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_158 += 
            fission_yield(
                *self, 
                Nuclide::Nd158, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_158 += 
            fission_yield(
                *self, 
                Nuclide::Pm158, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_158 += 
            fission_yield(
                *self, 
                Nuclide::Sm158, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_158 += 
            fission_yield(
                *self, 
                Nuclide::Eu158, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_158 += 
            fission_yield(
                *self, 
                Nuclide::Gd158, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_158 += 
            fission_yield(
                *self, 
                Nuclide::Tb158, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_158 += 
            fission_yield(
                *self, 
                Nuclide::Dy158, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        //distribution_yield_158 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ho158, 
        //        neutron_energy).get::<ratio>();
        // Z=68 Erbium
        //distribution_yield_158 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Er158, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_158;
    }
    /// gets mass yield for A=159
    pub fn get_mass_yield_159(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_159: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 Cerium
        distribution_yield_159 += 
            fission_yield(
                *self, 
                Nuclide::Ce159, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_159 += 
            fission_yield(
                *self, 
                Nuclide::Pr159, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_159 += 
            fission_yield(
                *self, 
                Nuclide::Nd159, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_159 += 
            fission_yield(
                *self, 
                Nuclide::Pm159, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_159 += 
            fission_yield(
                *self, 
                Nuclide::Sm159, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_159 += 
            fission_yield(
                *self, 
                Nuclide::Eu159, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_159 += 
            fission_yield(
                *self, 
                Nuclide::Gd159, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_159 += 
            fission_yield(
                *self, 
                Nuclide::Tb159, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_159 += 
            fission_yield(
                *self, 
                Nuclide::Dy159, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_159 += 
            fission_yield(
                *self, 
                Nuclide::Ho159, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        //distribution_yield_159 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Er159, 
        //        neutron_energy).get::<ratio>();
        // Z=69 Thulium
        //distribution_yield_159 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tm159, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_159;
    }
    /// gets mass yield for A=160
    pub fn get_mass_yield_160(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_160: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 Cerium
        distribution_yield_160 += 
            fission_yield(
                *self, 
                Nuclide::Ce160, 
                neutron_energy).get::<ratio>();
        // Z=59 Praseodymium
        distribution_yield_160 += 
            fission_yield(
                *self, 
                Nuclide::Pr160, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_160 += 
            fission_yield(
                *self, 
                Nuclide::Nd160, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_160 += 
            fission_yield(
                *self, 
                Nuclide::Pm160, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_160 += 
            fission_yield(
                *self, 
                Nuclide::Sm160, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_160 += 
            fission_yield(
                *self, 
                Nuclide::Eu160, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_160 += 
            fission_yield(
                *self, 
                Nuclide::Gd160, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_160 += 
            fission_yield(
                *self, 
                Nuclide::Tb160, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_160 += 
            fission_yield(
                *self, 
                Nuclide::Dy160, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        //distribution_yield_160 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Ho160, 
        //        neutron_energy).get::<ratio>();
        // Z=68 Erbium
        //distribution_yield_160 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Er160, 
        //        neutron_energy).get::<ratio>();
        // Z=69 Thulium
        //distribution_yield_160 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tm160, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_160;
    }
    /// gets mass yield for A=161
    pub fn get_mass_yield_161(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_161: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 Praseodymium
        distribution_yield_161 += 
            fission_yield(
                *self, 
                Nuclide::Pr161, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_161 += 
            fission_yield(
                *self, 
                Nuclide::Nd161, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_161 += 
            fission_yield(
                *self, 
                Nuclide::Pm161, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_161 += 
            fission_yield(
                *self, 
                Nuclide::Sm161, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_161 += 
            fission_yield(
                *self, 
                Nuclide::Eu161, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_161 += 
            fission_yield(
                *self, 
                Nuclide::Gd161, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_161 += 
            fission_yield(
                *self, 
                Nuclide::Tb161, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_161 += 
            fission_yield(
                *self, 
                Nuclide::Dy161, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_161 += 
            fission_yield(
                *self, 
                Nuclide::Ho161, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_161 += 
            fission_yield(
                *self, 
                Nuclide::Er161, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        //distribution_yield_161 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tm161, 
        //        neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        //distribution_yield_161 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Yb161, 
        //        neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        //distribution_yield_161 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Lu161, 
        //        neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        //distribution_yield_161 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Hf161, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_161;
    }
    /// gets mass yield for A=162
    pub fn get_mass_yield_162(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_162: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 Praseodymium
        distribution_yield_162 += 
            fission_yield(
                *self, 
                Nuclide::Pr162, 
                neutron_energy).get::<ratio>();
        // Z=60 Neodymium
        distribution_yield_162 += 
            fission_yield(
                *self, 
                Nuclide::Nd162, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_162 += 
            fission_yield(
                *self, 
                Nuclide::Pm162, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_162 += 
            fission_yield(
                *self, 
                Nuclide::Sm162, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_162 += 
            fission_yield(
                *self, 
                Nuclide::Eu162, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_162 += 
            fission_yield(
                *self, 
                Nuclide::Gd162, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_162 += 
            fission_yield(
                *self, 
                Nuclide::Tb162, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_162 += 
            fission_yield(
                *self, 
                Nuclide::Dy162, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_162 += 
            fission_yield(
                *self, 
                Nuclide::Ho162, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_162 += 
            fission_yield(
                *self, 
                Nuclide::Er162, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        //distribution_yield_162 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tm162, 
        //        neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        //distribution_yield_162 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Yb162, 
        //        neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        //distribution_yield_162 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Lu162, 
        //        neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        //distribution_yield_162 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Hf162, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_162;
    }
    /// gets mass yield for A=163
    pub fn get_mass_yield_163(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_163: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 (Praseodymium) -- none
        // Z=60 Neodymium
        distribution_yield_163 += 
            fission_yield(
                *self, 
                Nuclide::Nd163, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_163 += 
            fission_yield(
                *self, 
                Nuclide::Pm163, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_163 += 
            fission_yield(
                *self, 
                Nuclide::Sm163, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_163 += 
            fission_yield(
                *self, 
                Nuclide::Eu163, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_163 += 
            fission_yield(
                *self, 
                Nuclide::Gd163, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_163 += 
            fission_yield(
                *self, 
                Nuclide::Tb163, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_163 += 
            fission_yield(
                *self, 
                Nuclide::Dy163, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_163 += 
            fission_yield(
                *self, 
                Nuclide::Ho163, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_163 += 
            fission_yield(
                *self, 
                Nuclide::Er163, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        //distribution_yield_163 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tm163, 
        //        neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        //distribution_yield_163 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Yb163, 
        //        neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        //distribution_yield_163 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Lu163, 
        //        neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        //distribution_yield_163 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Hf163, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_163;
    }
    /// gets mass yield for A=164
    pub fn get_mass_yield_164(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_164: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 (Praseodymium) -- none
        // Z=60 Neodymium
        distribution_yield_164 += 
            fission_yield(
                *self, 
                Nuclide::Nd164, 
                neutron_energy).get::<ratio>();
        // Z=61 Promethium
        distribution_yield_164 += 
            fission_yield(
                *self, 
                Nuclide::Pm164, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_164 += 
            fission_yield(
                *self, 
                Nuclide::Sm164, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_164 += 
            fission_yield(
                *self, 
                Nuclide::Eu164, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_164 += 
            fission_yield(
                *self, 
                Nuclide::Gd164, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_164 += 
            fission_yield(
                *self, 
                Nuclide::Tb164, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_164 += 
            fission_yield(
                *self, 
                Nuclide::Dy164, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_164 += 
            fission_yield(
                *self, 
                Nuclide::Ho164, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_164 += 
            fission_yield(
                *self, 
                Nuclide::Er164, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        //distribution_yield_164 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Tm164, 
        //        neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        //distribution_yield_164 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Yb164, 
        //        neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        //distribution_yield_164 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Lu164, 
        //        neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        //distribution_yield_164 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Hf164, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_164;
    }
    /// gets mass yield for A=165
    pub fn get_mass_yield_165(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_165: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 (Praseodymium) -- none
        // Z=60 (Neodymium) -- none
        // Z=61 Promethium
        distribution_yield_165 += 
            fission_yield(
                *self, 
                Nuclide::Pm165, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_165 += 
            fission_yield(
                *self, 
                Nuclide::Sm165, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_165 += 
            fission_yield(
                *self, 
                Nuclide::Eu165, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_165 += 
            fission_yield(
                *self, 
                Nuclide::Gd165, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_165 += 
            fission_yield(
                *self, 
                Nuclide::Tb165, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_165 += 
            fission_yield(
                *self, 
                Nuclide::Dy165, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_165 += 
            fission_yield(
                *self, 
                Nuclide::Ho165, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_165 += 
            fission_yield(
                *self, 
                Nuclide::Er165, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        distribution_yield_165 += 
            fission_yield(
                *self, 
                Nuclide::Tm165, 
                neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        //distribution_yield_165 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Yb165, 
        //        neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        //distribution_yield_165 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Lu165, 
        //        neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        //distribution_yield_165 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Hf165, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_165;
    }
    /// gets mass yield for A=166
    pub fn get_mass_yield_166(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_166: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 (Praseodymium) -- none
        // Z=60 (Neodymium) -- none
        // Z=61 Promethium
        distribution_yield_166 += 
            fission_yield(
                *self, 
                Nuclide::Pm166, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_166 += 
            fission_yield(
                *self, 
                Nuclide::Sm166, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_166 += 
            fission_yield(
                *self, 
                Nuclide::Eu166, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_166 += 
            fission_yield(
                *self, 
                Nuclide::Gd166, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_166 += 
            fission_yield(
                *self, 
                Nuclide::Tb166, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_166 += 
            fission_yield(
                *self, 
                Nuclide::Dy166, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_166 += 
            fission_yield(
                *self, 
                Nuclide::Ho166, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_166 += 
            fission_yield(
                *self, 
                Nuclide::Er166, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        distribution_yield_166 += 
            fission_yield(
                *self, 
                Nuclide::Tm166, 
                neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        distribution_yield_166 += 
            fission_yield(
                *self, 
                Nuclide::Yb166, 
                neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        //distribution_yield_166 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Lu166, 
        //        neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        //distribution_yield_166 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Hf166, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_166;
    }
    /// gets mass yield for A=167
    pub fn get_mass_yield_167(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_167: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 (Praseodymium) -- none
        // Z=60 (Neodymium) -- none
        // Z=61 Promethium
        distribution_yield_167 += 
            fission_yield(
                *self, 
                Nuclide::Pm167, 
                neutron_energy).get::<ratio>();
        // Z=62 Samarium
        distribution_yield_167 += 
            fission_yield(
                *self, 
                Nuclide::Sm167, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_167 += 
            fission_yield(
                *self, 
                Nuclide::Eu167, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_167 += 
            fission_yield(
                *self, 
                Nuclide::Gd167, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_167 += 
            fission_yield(
                *self, 
                Nuclide::Tb167, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_167 += 
            fission_yield(
                *self, 
                Nuclide::Dy167, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_167 += 
            fission_yield(
                *self, 
                Nuclide::Ho167, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_167 += 
            fission_yield(
                *self, 
                Nuclide::Er167, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        distribution_yield_167 += 
            fission_yield(
                *self, 
                Nuclide::Tm167, 
                neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        distribution_yield_167 += 
            fission_yield(
                *self, 
                Nuclide::Yb167, 
                neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        //distribution_yield_167 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Lu167, 
        //        neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        //distribution_yield_167 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Hf167, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_167;
    }
    /// gets mass yield for A=168
    pub fn get_mass_yield_168(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_168: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 (Praseodymium) -- none
        // Z=60 (Neodymium) -- none
        // Z=61 (Promethium) -- none
        // Z=62 Samarium
        distribution_yield_168 += 
            fission_yield(
                *self, 
                Nuclide::Sm168, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_168 += 
            fission_yield(
                *self, 
                Nuclide::Eu168, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_168 += 
            fission_yield(
                *self, 
                Nuclide::Gd168, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_168 += 
            fission_yield(
                *self, 
                Nuclide::Tb168, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_168 += 
            fission_yield(
                *self, 
                Nuclide::Dy168, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_168 += 
            fission_yield(
                *self, 
                Nuclide::Ho168, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_168 += 
            fission_yield(
                *self, 
                Nuclide::Er168, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        distribution_yield_168 += 
            fission_yield(
                *self, 
                Nuclide::Tm168, 
                neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        distribution_yield_168 += 
            fission_yield(
                *self, 
                Nuclide::Yb168, 
                neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        //distribution_yield_168 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Lu168, 
        //        neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        //distribution_yield_168 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Hf168, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_168;
    }
    /// gets mass yield for A=169
    pub fn get_mass_yield_169(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_169: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 (Praseodymium) -- none
        // Z=60 (Neodymium) -- none
        // Z=61 (Promethium) -- none
        // Z=62 Samarium
        distribution_yield_169 += 
            fission_yield(
                *self, 
                Nuclide::Sm169, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_169 += 
            fission_yield(
                *self, 
                Nuclide::Eu169, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_169 += 
            fission_yield(
                *self, 
                Nuclide::Gd169, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_169 += 
            fission_yield(
                *self, 
                Nuclide::Tb169, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_169 += 
            fission_yield(
                *self, 
                Nuclide::Dy169, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_169 += 
            fission_yield(
                *self, 
                Nuclide::Ho169, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_169 += 
            fission_yield(
                *self, 
                Nuclide::Er169, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        distribution_yield_169 += 
            fission_yield(
                *self, 
                Nuclide::Tm169, 
                neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        distribution_yield_169 += 
            fission_yield(
                *self, 
                Nuclide::Yb169, 
                neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        distribution_yield_169 += 
            fission_yield(
                *self, 
                Nuclide::Lu169, 
                neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        //distribution_yield_169 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Hf169, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_169;
    }
    /// gets mass yield for A=170
    pub fn get_mass_yield_170(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_170: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 (Praseodymium) -- none
        // Z=60 (Neodymium) -- none
        // Z=61 (Promethium) -- none
        // Z=62 Samarium
        distribution_yield_170 += 
            fission_yield(
                *self, 
                Nuclide::Sm170, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_170 += 
            fission_yield(
                *self, 
                Nuclide::Eu170, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_170 += 
            fission_yield(
                *self, 
                Nuclide::Gd170, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_170 += 
            fission_yield(
                *self, 
                Nuclide::Tb170, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_170 += 
            fission_yield(
                *self, 
                Nuclide::Dy170, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_170 += 
            fission_yield(
                *self, 
                Nuclide::Ho170, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_170 += 
            fission_yield(
                *self, 
                Nuclide::Er170, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        distribution_yield_170 += 
            fission_yield(
                *self, 
                Nuclide::Tm170, 
                neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        distribution_yield_170 += 
            fission_yield(
                *self, 
                Nuclide::Yb170, 
                neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        //distribution_yield_170 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Lu170, 
        //        neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        //distribution_yield_170 += 
        //    fission_yield(
        //        *self, 
        //        Nuclide::Hf170, 
        //        neutron_energy).get::<ratio>();



        return distribution_yield_170;
    }
    /// gets mass yield for A=171
    pub fn get_mass_yield_171(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_171: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 (Praseodymium) -- none
        // Z=60 (Neodymium) -- none
        // Z=61 (Promethium) -- none
        // Z=62 Samarium
        distribution_yield_171 += 
            fission_yield(
                *self, 
                Nuclide::Sm171, 
                neutron_energy).get::<ratio>();
        // Z=63 Europium
        distribution_yield_171 += 
            fission_yield(
                *self, 
                Nuclide::Eu171, 
                neutron_energy).get::<ratio>();
        // Z=64 Gadolinium
        distribution_yield_171 += 
            fission_yield(
                *self, 
                Nuclide::Gd171, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_171 += 
            fission_yield(
                *self, 
                Nuclide::Tb171, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_171 += 
            fission_yield(
                *self, 
                Nuclide::Dy171, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_171 += 
            fission_yield(
                *self, 
                Nuclide::Ho171, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_171 += 
            fission_yield(
                *self, 
                Nuclide::Er171, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        distribution_yield_171 += 
            fission_yield(
                *self, 
                Nuclide::Tm171, 
                neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        distribution_yield_171 += 
            fission_yield(
                *self, 
                Nuclide::Yb171, 
                neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        distribution_yield_171 += 
            fission_yield(
                *self, 
                Nuclide::Lu171, 
                neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        distribution_yield_171 += 
            fission_yield(
                *self, 
                Nuclide::Hf171, 
                neutron_energy).get::<ratio>();



        return distribution_yield_171;
    }
    /// gets mass yield for A=172
    pub fn get_mass_yield_172(&self, 
        neutron_energy: Energy) -> f64 {

        let mut distribution_yield_172: f64 = 0.0;
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
        // Z=48 (Cadmium) -- none
        // Z=49 (Indium) -- none
        // Z=50 (Tin) -- none
        // Z=51 (Antimony) -- none
        // Z=52 (Tellurium) -- none
        // Z=53 (Iodine) -- none
        // Z=54 (Xenon) -- none
        // Z=55 (Cesium) -- none
        // Z=56 (Barium) -- none
        // Z=57 (Lanthanum) -- none
        // Z=58 (Cerium) -- none
        // Z=59 (Praseodymium) -- none
        // Z=60 (Neodymium) -- none
        // Z=61 (Promethium) -- none
        // Z=62 (Samarium) -- none
        // Z=63 (Europium) -- none
        // Z=64 Gadolinium
        distribution_yield_172 += 
            fission_yield(
                *self, 
                Nuclide::Gd172, 
                neutron_energy).get::<ratio>();
        // Z=65 Terbium
        distribution_yield_172 += 
            fission_yield(
                *self, 
                Nuclide::Tb172, 
                neutron_energy).get::<ratio>();
        // Z=66 Dysprosium
        distribution_yield_172 += 
            fission_yield(
                *self, 
                Nuclide::Dy172, 
                neutron_energy).get::<ratio>();
        // Z=67 Holonium
        distribution_yield_172 += 
            fission_yield(
                *self, 
                Nuclide::Ho172, 
                neutron_energy).get::<ratio>();
        // Z=68 Erbium
        distribution_yield_172 += 
            fission_yield(
                *self, 
                Nuclide::Er172, 
                neutron_energy).get::<ratio>();
        // Z=69 Thulium
        distribution_yield_172 += 
            fission_yield(
                *self, 
                Nuclide::Tm172, 
                neutron_energy).get::<ratio>();
        // Z=70 Ytterbium
        distribution_yield_172 += 
            fission_yield(
                *self, 
                Nuclide::Yb172, 
                neutron_energy).get::<ratio>();
        // Z=71 Lutetium
        distribution_yield_172 += 
            fission_yield(
                *self, 
                Nuclide::Lu172, 
                neutron_energy).get::<ratio>();
        // Z=72 Hafnium
        distribution_yield_172 += 
            fission_yield(
                *self, 
                Nuclide::Hf172, 
                neutron_energy).get::<ratio>();



        return distribution_yield_172;
    }
}



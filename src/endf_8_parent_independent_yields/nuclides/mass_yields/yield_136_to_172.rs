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
}



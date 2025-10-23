use uom::si::f64::Energy;

use crate::endf_8_parent_independent_yields::nuclides::Nuclide;


/// contains functions for masses 66 to 100
///
/// note that for metastable isotopes, eg. Tc99m as opposed to Tc99,
/// their yields are different, there is a difference.
///
/// However, where it is not so important to differentiate the 
/// metastable states from the normal counterpart, eg. Nb104, and Nb104m,
/// the ENDF 8 library just has equal yields for both 
/// this happens for many metastable isotopes,
/// eg Nb102 and Nb102m
pub mod yield_66_to_100;

/// contains functions for masses 101 to 135
pub mod yield_101_to_135;

/// contains functions for masses 136 to 172
pub mod yield_136_to_172;

/// generates a yield vector for all nuclides with mass 
/// 66 to 172 
impl Nuclide {

    pub fn get_mass_yield_vector(&self, neutron_energy: Energy)
        -> Vec<(u32,f64)>
    {
        let mut fission_distribution: Vec<(u32,f64)> = vec![];

        fission_distribution.push((66,self.get_mass_yield_66(neutron_energy)));
        fission_distribution.push((67,self.get_mass_yield_67(neutron_energy)));
        fission_distribution.push((68,self.get_mass_yield_68(neutron_energy)));
        fission_distribution.push((69,self.get_mass_yield_69(neutron_energy)));
        fission_distribution.push((70,self.get_mass_yield_70(neutron_energy)));
        fission_distribution.push((71,self.get_mass_yield_71(neutron_energy)));
        fission_distribution.push((72,self.get_mass_yield_72(neutron_energy)));
        fission_distribution.push((73,self.get_mass_yield_73(neutron_energy)));
        fission_distribution.push((74,self.get_mass_yield_74(neutron_energy)));
        fission_distribution.push((75,self.get_mass_yield_75(neutron_energy)));
        fission_distribution.push((76,self.get_mass_yield_76(neutron_energy)));
        fission_distribution.push((77,self.get_mass_yield_77(neutron_energy)));
        fission_distribution.push((78,self.get_mass_yield_78(neutron_energy)));
        fission_distribution.push((79,self.get_mass_yield_79(neutron_energy)));
        fission_distribution.push((80,self.get_mass_yield_80(neutron_energy)));
        fission_distribution.push((81,self.get_mass_yield_81(neutron_energy)));
        fission_distribution.push((82,self.get_mass_yield_82(neutron_energy)));
        fission_distribution.push((83,self.get_mass_yield_83(neutron_energy)));
        fission_distribution.push((84,self.get_mass_yield_84(neutron_energy)));
        fission_distribution.push((85,self.get_mass_yield_85(neutron_energy)));
        fission_distribution.push((86,self.get_mass_yield_86(neutron_energy)));
        fission_distribution.push((87,self.get_mass_yield_87(neutron_energy)));
        fission_distribution.push((88,self.get_mass_yield_88(neutron_energy)));
        fission_distribution.push((89,self.get_mass_yield_89(neutron_energy)));
        fission_distribution.push((90,self.get_mass_yield_90(neutron_energy)));
        fission_distribution.push((91,self.get_mass_yield_91(neutron_energy)));
        fission_distribution.push((92,self.get_mass_yield_92(neutron_energy)));
        fission_distribution.push((93,self.get_mass_yield_93(neutron_energy)));
        fission_distribution.push((94,self.get_mass_yield_94(neutron_energy)));
        fission_distribution.push((95,self.get_mass_yield_95(neutron_energy)));
        fission_distribution.push((96,self.get_mass_yield_96(neutron_energy)));
        fission_distribution.push((97,self.get_mass_yield_97(neutron_energy)));
        fission_distribution.push((98,self.get_mass_yield_98(neutron_energy)));
        fission_distribution.push((99,self.get_mass_yield_99(neutron_energy)));
        fission_distribution.push((100,self.get_mass_yield_100(neutron_energy)));
        fission_distribution.push((101,self.get_mass_yield_101(neutron_energy)));
        fission_distribution.push((102,self.get_mass_yield_102(neutron_energy)));
        fission_distribution.push((103,self.get_mass_yield_103(neutron_energy)));
        fission_distribution.push((104,self.get_mass_yield_104(neutron_energy)));
        fission_distribution.push((105,self.get_mass_yield_105(neutron_energy)));
        fission_distribution.push((106,self.get_mass_yield_106(neutron_energy)));
        fission_distribution.push((107,self.get_mass_yield_107(neutron_energy)));
        fission_distribution.push((108,self.get_mass_yield_108(neutron_energy)));
        fission_distribution.push((109,self.get_mass_yield_109(neutron_energy)));
        fission_distribution.push((110,self.get_mass_yield_110(neutron_energy)));
        fission_distribution.push((111,self.get_mass_yield_111(neutron_energy)));
        fission_distribution.push((112,self.get_mass_yield_112(neutron_energy)));
        fission_distribution.push((113,self.get_mass_yield_113(neutron_energy)));
        fission_distribution.push((114,self.get_mass_yield_114(neutron_energy)));
        fission_distribution.push((115,self.get_mass_yield_115(neutron_energy)));
        fission_distribution.push((116,self.get_mass_yield_116(neutron_energy)));
        fission_distribution.push((117,self.get_mass_yield_117(neutron_energy)));
        fission_distribution.push((118,self.get_mass_yield_118(neutron_energy)));
        fission_distribution.push((119,self.get_mass_yield_119(neutron_energy)));
        fission_distribution.push((120,self.get_mass_yield_120(neutron_energy)));
        fission_distribution.push((121,self.get_mass_yield_121(neutron_energy)));
        fission_distribution.push((122,self.get_mass_yield_122(neutron_energy)));
        fission_distribution.push((123,self.get_mass_yield_123(neutron_energy)));
        fission_distribution.push((124,self.get_mass_yield_124(neutron_energy)));
        fission_distribution.push((125,self.get_mass_yield_125(neutron_energy)));
        fission_distribution.push((126,self.get_mass_yield_126(neutron_energy)));
        fission_distribution.push((127,self.get_mass_yield_127(neutron_energy)));
        fission_distribution.push((128,self.get_mass_yield_128(neutron_energy)));
        fission_distribution.push((129,self.get_mass_yield_129(neutron_energy)));
        fission_distribution.push((130,self.get_mass_yield_130(neutron_energy)));
        fission_distribution.push((131,self.get_mass_yield_131(neutron_energy)));
        fission_distribution.push((132,self.get_mass_yield_132(neutron_energy)));
        fission_distribution.push((133,self.get_mass_yield_133(neutron_energy)));
        fission_distribution.push((134,self.get_mass_yield_134(neutron_energy)));
        fission_distribution.push((135,self.get_mass_yield_135(neutron_energy)));
        fission_distribution.push((136,self.get_mass_yield_136(neutron_energy)));
        fission_distribution.push((137,self.get_mass_yield_137(neutron_energy)));
        fission_distribution.push((138,self.get_mass_yield_138(neutron_energy)));
        fission_distribution.push((139,self.get_mass_yield_139(neutron_energy)));
        fission_distribution.push((140,self.get_mass_yield_140(neutron_energy)));
        fission_distribution.push((141,self.get_mass_yield_141(neutron_energy)));
        fission_distribution.push((142,self.get_mass_yield_142(neutron_energy)));
        fission_distribution.push((143,self.get_mass_yield_143(neutron_energy)));
        fission_distribution.push((144,self.get_mass_yield_144(neutron_energy)));
        fission_distribution.push((145,self.get_mass_yield_145(neutron_energy)));
        fission_distribution.push((146,self.get_mass_yield_146(neutron_energy)));
        fission_distribution.push((147,self.get_mass_yield_147(neutron_energy)));
        fission_distribution.push((148,self.get_mass_yield_148(neutron_energy)));
        fission_distribution.push((149,self.get_mass_yield_149(neutron_energy)));
        fission_distribution.push((150,self.get_mass_yield_150(neutron_energy)));
        fission_distribution.push((151,self.get_mass_yield_151(neutron_energy)));
        fission_distribution.push((152,self.get_mass_yield_152(neutron_energy)));
        fission_distribution.push((153,self.get_mass_yield_153(neutron_energy)));
        fission_distribution.push((154,self.get_mass_yield_154(neutron_energy)));
        fission_distribution.push((155,self.get_mass_yield_155(neutron_energy)));
        fission_distribution.push((156,self.get_mass_yield_156(neutron_energy)));
        fission_distribution.push((157,self.get_mass_yield_157(neutron_energy)));
        fission_distribution.push((158,self.get_mass_yield_158(neutron_energy)));
        fission_distribution.push((159,self.get_mass_yield_159(neutron_energy)));
        fission_distribution.push((160,self.get_mass_yield_160(neutron_energy)));
        fission_distribution.push((161,self.get_mass_yield_161(neutron_energy)));
        fission_distribution.push((162,self.get_mass_yield_162(neutron_energy)));
        fission_distribution.push((163,self.get_mass_yield_163(neutron_energy)));
        fission_distribution.push((164,self.get_mass_yield_164(neutron_energy)));
        fission_distribution.push((165,self.get_mass_yield_165(neutron_energy)));
        fission_distribution.push((166,self.get_mass_yield_166(neutron_energy)));
        fission_distribution.push((167,self.get_mass_yield_167(neutron_energy)));
        fission_distribution.push((168,self.get_mass_yield_168(neutron_energy)));
        fission_distribution.push((169,self.get_mass_yield_169(neutron_energy)));
        fission_distribution.push((170,self.get_mass_yield_170(neutron_energy)));
        fission_distribution.push((171,self.get_mass_yield_171(neutron_energy)));
        fission_distribution.push((172,self.get_mass_yield_172(neutron_energy)));

        return fission_distribution;
    }

}

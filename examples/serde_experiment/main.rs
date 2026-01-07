
pub fn main() {

    println!("the serde example is here");
    let xml = r#"
<nuclide name="Mo99" half_life="237513.6" decay_modes="2" decay_energy="543174.1000000001" reactions="5">
  <decay type="beta-" target="Tc99" branching_ratio="0.1211359"/>
  <decay type="beta-" target="Tc99_m1" branching_ratio="0.8788641"/>
  <reaction type="(n,2n)" Q="-5930200.0" target="Mo98"/>
  <reaction type="(n,3n)" Q="-14577200.0" target="Mo97"/>
  <reaction type="(n,gamma)" Q="8290890.0" target="Mo100"/>
  <reaction type="(n,p)" Q="-2841030.0" target="Nb99"/>
  <reaction type="(n,a)" Q="5129400.0" target="Zr96"/>
</nuclide>
"#;

    let decay_data: NuclideDepletionData = from_str(&xml).unwrap();

    dbg!(&decay_data);
}
use serde::{Serialize,Deserialize};
use serde_xml_rs::from_str;

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename = "nuclide")]
pub struct NuclideDepletionData {
    // Attributes
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@half_life")]
    half_life_seconds: f64,

    #[serde(rename = "@decay_modes")]
    decay_modes: u32,

    #[serde(rename = "@decay_energy")]
    decay_energy: f64,

    // Child elements

    #[serde(default)]
    decay: Vec<DecayData>,

    #[serde(default)]
    reaction: Vec<ReactionData>,

}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename = "decay")]
struct DecayData {

    #[serde(rename = "@type")]
    decay_type: String,

    #[serde(rename = "@target")]
    target: String,

    #[serde(rename = "@branching_ratio")]
    branching_ratio: f64,

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "reaction")]
struct ReactionData {

    #[serde(rename = "@type")]
    reaction_type: String,

    #[serde(rename = "@Q")]
    q_value_electronvolt: f64,

    #[serde(rename = "@target")]
    target: String,

}


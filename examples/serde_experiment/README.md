
# Notes for 07 Jan 2026

Basically, I have a large xml file from openmc.

https://openmc.org/data/

This is the ENDF b/VIII.0, and in xml format. 

## Available information

As far as i know, there are no cross sections available for this. However,
the branching ratios themselves take into account an assumed neutron 
spectrum. Either PWR or SFR spectrum, these are both extremes of the 
neutron spectrum. Though of course, HTGR is even more thermalised than 
either of them. Even PWR.

However, it also contains all the decay information and decay yields.

## Sample

Suppose that for hydrogen, we have these seven nuclides. 

```
  <nuclide name="H1" reactions="1">
    <reaction type="(n,gamma)" Q="2224648.0" target="H2"/>
  </nuclide>
  <nuclide name="H2" reactions="2">
    <reaction type="(n,2n)" Q="-2225002.0" target="H1"/>
    <reaction type="(n,gamma)" Q="6257402.0" target="H3"/>
  </nuclide>
  <nuclide name="H3" half_life="388789600.0" decay_modes="1" decay_energy="5690.0" reactions="1">
    <decay type="beta-" target="He3" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>18590.0 1.7828336471961835e-09</parameters>
    </source>
    <reaction type="(n,2n)" Q="-6257557.0" target="H2"/>
  </nuclide>
  <nuclide name="H4" half_life="9.90652e-23" decay_modes="1" decay_energy="2880390.0" reactions="0">
    <decay type="n" target="H3" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="H5" half_life="7.99473e-23" decay_modes="1" decay_energy="200000.0" reactions="0">
    <decay type="n" target="H4" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="H6" half_life="2.84812e-22" decay_modes="1" decay_energy="900000.0" reactions="0">
    <decay type="n" target="H5" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="H7" half_life="2.3e-23" decay_modes="1" decay_energy="107400.0" reactions="0">
    <decay type="n,n" target="H5" branching_ratio="1.0"/>
  </nuclide>

```

They have reaction types and Q values as well as branching ratios.

For multiple decay types, we can look at Mo99:

```
  <nuclide name="Mo99" half_life="237513.6" decay_modes="2" decay_energy="543174.1000000001" reactions="5">
    <decay type="beta-" target="Tc99" branching_ratio="0.1211359"/>
    <decay type="beta-" target="Tc99_m1" branching_ratio="0.8788641"/>
    <source type="discrete" particle="photon">
      <parameters>2162.542 2414.132 2548.688 2827.143 18214.1 18334.4 20566.8 20587.0 20745.0 20748.9 20962.4 20965.4 21003.9 21004.2 40583.23 140511.0 158782.0 162370.0 181068.0 242290.0 249030.0 366421.0 380130.0 391700.0 410270.0 411491.0 457600.0 469630.0 528788.0 537790.0 580510.0 620030.0 621771.0 689600.0 739500.0 761770.0 777921.0 822972.0 861200.0 960754.0 986440.0 1001343.0 1017000.0 1056200.0 7.245182470258116e-10 3.9239777190102644e-09 2.0555230587923245e-09 1.1643128979243949e-10 3.091609611701348e-08 5.8784445382705717e-08 4.862197202350237e-09 9.462729546337137e-09 2.536468449896612e-11 3.580003792532461e-11 9.380770679902352e-10 1.8027582842833607e-09 1.3420166117493637e-12 1.872811752782644e-12 3.105611842193946e-08 1.3238207161425804e-07 5.581514370763312e-10 3.5063359508641323e-10 1.7925248075336023e-07 7.513577037565998e-11 1.1449260247719614e-10 3.513491738518956e-08 3.0769886915746465e-10 9.302523951272187e-11 5.724630123859807e-11 4.2934725928948553e-10 2.397188864366294e-10 7.871366420307235e-11 1.6816100988838184e-09 9.660313334013425e-11 9.302523951272187e-11 6.797998272083522e-11 5.366840741118569e-10 1.2522628395943328e-11 3.5778938274123796e-07 1.1807049630460854e-11 1.2558407334217452e-07 3.935683210153618e-09 2.1467362964474276e-10 2.790757185381656e-09 4.2934725928948554e-11 1.610052222335571e-10 1.7889469137061898e-11 3.1843255063970176e-11</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>106.1654 110.7337 204.1852 2053.998 2332.701 2641.852 15423.27 17823.92 19539.23 20227.04 37540.73 40039.23 40515.23 119467.0 137468.5 139967.0 140443.0 158120.0 160024.0 178025.5 180524.0 181000.0 184780.0 215137.9 221246.0 227877.0 239247.5 241746.0 242222.0 345377.0 352925.0 359086.0 363378.5 365877.0 366353.0 377087.5 379586.0 380062.0 389226.0 390447.0 407227.5 408448.5 409726.0 410202.0 410947.0 411423.0 436363.0 436556.0 454557.5 457056.0 457532.0 507744.0 525745.6 528244.0 528720.1 685500.0 718456.0 736457.5 738956.0 756877.0 774878.6 777377.0 801928.0 819929.5 822428.0 822904.0 847875.0 1214317.0 2.9332425754547247e-08 1.318651447515749e-07 3.2912245691197876e-07 1.3031112479798834e-07 2.032379397121108e-08 9.190033335063836e-10 2.138578640153981e-08 8.083670351919466e-09 1.0062181434837238e-07 7.368535175231254e-10 1.223611089171193e-08 2.2267235274256085e-09 3.524870374761274e-10 1.3079349842827627e-08 1.5885848593710965e-09 2.8859291611908256e-10 4.567181470691902e-11 6.128529394425772e-11 2.2442411991127418e-08 3.3699469883648517e-09 6.166284053842766e-10 9.374904801767686e-11 5.544859928289985e-11 3.239365537053623e-09 6.927517444966383e-13 3.5020167968147273e-10 7.911797204226461e-14 1.4275796371375393e-14 2.2540731112697992e-15 2.779171930148327e-10 4.2607871027912515e-09 2.8000597093329285e-12 3.211331892595121e-11 5.818341174995239e-12 9.275616788883326e-13 3.4154586149867896e-13 6.215516573311319e-14 9.846363813038869e-15 3.6637632792702767e-13 9.660313334013425e-13 4.2934725928948555e-14 1.0905420385952933e-13 7.785496968449338e-15 1.2594186272491575e-15 1.9707039201387387e-14 3.125649214966387e-15 4.78608962231346e-07 1.246538209470473e-12 1.4862570959071023e-13 2.6848515280902497e-14 4.2190518176152114e-15 5.885635346093364e-12 6.894601405423655e-13 1.2612075741628637e-13 1.9842999166829055e-14 1.6634579784869953e-09 5.370418634945982e-10 6.282781560936138e-11 1.1377702371171367e-11 6.467580068956721e-11 7.2336433249126114e-12 1.3060742460247218e-12 1.8064785934605104e-12 2.015069803598652e-13 3.6365701188430104e-14 5.793325101676659e-15 3.385282903587569e-08 2.398881505818088e-06</parameters>
    </source>
    <reaction type="(n,2n)" Q="-5930200.0" target="Mo98"/>
    <reaction type="(n,3n)" Q="-14577200.0" target="Mo97"/>
    <reaction type="(n,gamma)" Q="8290890.0" target="Mo100"/>
    <reaction type="(n,p)" Q="-2841030.0" target="Nb99"/>
    <reaction type="(n,a)" Q="5129400.0" target="Zr96"/>
  </nuclide>
```


Now, we have branching ratio for beta minus decay. Also, we have multiple 
reaction types. 

However, the source type for photons and electrons, I'm not sure what 
that is nor what those numbers mean.

Nevertheless, Rust is able to take these datafiles and convert them to 
a Rust struct. This is done using serde library.

Consider this video, this is for basics:

https://www.youtube.com/watch?v=VSTF69iq6Qc

Moreover, we can copy and paste the xml directly into rust code.
Now, we can use Rust serde to conver them into a struct. 

However, the name of the struct may be nuclide, which then clashes with the 
existing fission product data.

Moreover, the source type discrete photons and electrons is something I 
want to get rid of. Probably want to do something about that later.


## making a serde example 

1. I want to build a simple serde example which deals with nuclides from 
fission-yields-data crate.

2. However, when i do this serde example, and use the v0.1.1 version of 
fission-yields-data, then the build function has some warnings. 

3. I would like to remove the warnings. So I looked backed at my 
fission-yields-data crate. It seems i have not merged my develop branch 
changes back to main. Furthermore, I am not sure if the changes actually 
need more testing. 

I don't know if i wanted to push a version not fully unit tested to main.
Probably the linear interpolation function isn't quite fully tested yet.

Well, in the interest of expediency, I will just use the v0.1.2 version first.
Else, I will just do the serde example in the fission-yields-data rather 
than over here.

For serde, i can only convert the xml to floats (Numbers) and strings (Texts).
Wherein fact, i would rather convert them to enums. I wonder what mechanism 
I should use though...

okay, the serde example is useful (I used AI to vibe code at 
least the first few drafts, so it is generated faster):

```

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
```

Basically, in the vibe coding process, the raw chatgpt 5 version 
has an error. But the code is useful enough to be debugged in 
a few steps. Thus saving drafting efforts. Good to be learning from 
interns and juniors like ZhiZheng who know how to use this stuff!

As far as i can see, the serde rename part takes the name of the 
data type, for example "reaction". I can convert it into a Rust 
struct called ReactionData as shown in the above code.

Okay great, I can convert the xml code to strings and floats.

Now, tricky thing here is i want to convert the floats to enums. How?

From chatgpt 5:
```
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
enum DecayType {
    #[serde(rename = "beta-")]
    BetaMinus,
    #[serde(rename = "beta+")]
    BetaPlus,
    #[serde(rename = "alpha")]
    Alpha,

    // Optional: accept multiple spellings
    #[serde(alias = "γ")]
    #[serde(alias = "gamma")]
    Gamma,
}
```

For unknowns:

```
#[derive(Debug, Deserialize, PartialEq)]

enum DecayType {

    #[serde(rename = "beta-")]
    BetaMinus,

    #[serde(rename = "beta+")]
    BetaPlus,

    #[serde(rename = "alpha")]
    Alpha,

    #[serde(other)]
    Unknown, // any other string maps here

}
```

For Serde, we can also use a custom deserialiser:


```
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
enum DecayType {
    BetaMinus,
    BetaPlus,
    Alpha,
    Other(String),
}

fn de_decay_type<'de, D>(de: D) -> Result<DecayType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    Ok(match s.as_str() {
        "beta-" => DecayType::BetaMinus,
        "beta+" => DecayType::BetaPlus,
        "alpha" => DecayType::Alpha,
        _ => DecayType::Other(s),
    })
}
```
At the back of things, it looks as if it is a custom match statement.







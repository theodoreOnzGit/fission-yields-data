// this part is vibe coded to deal with boilerplate
//

use std::str::FromStr;

use super::Nuclide;

pub fn parse_nuclide_allow_underscore_isomer(s: &str) -> Option<Nuclide> {
    let s = s.trim();
    match s {
        // Any string containing an underscore gets normalized and parsed
        u if u.contains('_') => {
            let normalized = u.replace('_', "");
            Nuclide::from_str(&normalized).ok()
        }
        // Otherwise parse directly
        _ => Nuclide::from_str(s).ok(),
    }
}


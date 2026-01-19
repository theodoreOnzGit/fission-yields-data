# fission-yields-data
Fission Yields for various nuclides using ENDF or other library data


# nuclides completed 

1. U235
2. U233 
3. Pu239
4. U238 (note that for the mass distribution curve, some nuclides were ignored)

# License 

```
This provides fission yield data for some important nuclides 
based on ENDF libraries
Copyright (C) 2025 SNRSI, Theodore Ong

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

```

# patch notes 

v0.1.4

Included some isomers in the nuclide databank for more complete version.
Namely:

```rust

    Ru103m,
    Na24m,
    Na36,
    K32,
    K33,
    K34,
    Rb71,
    Cs125m,
    Cs144m,
    Fr214m,
    Fr218m,

    Be5,
    Mg39,
    Ba130m,
    Ra203m,
    Ra207m,
    Ra213m,
    Ca34,

    Xe132m,
    Rn197m,

    Br67,
    At196m,

    O27,
    Po191m,
    Po205m1,
    Po205m,

    As60,
    As61,
    As62,
    Sb103,
    Bi184m,
    Bi186m,
    Bi187m,
    Bi189m,
    Bi204m1,
    Bi204m,
    Bi208m,
    P24,
    N25,

    Zn61m1,
    Zn61m,
    Zn73m1,
    Zn73m,
    Cu52,
    Cu76m,
    Co49,
    V40,
    V41,
    Mn45,
    V46m,
    Ti38,
    Sc36,
    Sc37,
    Sc38,
    Y88m,
    Y88m1,
    Nb90m1,
    Tc86m,
    Tc117m,
    Pd117m,
    Ag95m,
    Ag95m1,
    Ag95m2,
    Ag114m,
    Ta157m,
    Ta157m1,
    Ta158m,
    Ta176m,
    Ta176m1,
    Ta179m,
    Ta179m1,
    Ta185m,
    W180m,
    W186m,
    W190m,
    Re183m,
    Ir164m,
    Ir188m,
    Ir189m,
    Ir189m1,
    Ir194m1,
    Pt184m,
    Au169,
    Au192m1,
    Hg205m,

    La117m,
    La118,
    La119,
    Ce119,
    Ce120,
    Ce132m,
    Pr122,
    Pr123,
    Nd124,
    Pm126,
    Pm127,
    Pm138m,
    Pm142m,
    Sm128,
    Sm143m1,
    Sm153m,
    Eu133,
    Eu136m1,
    Gd155m,
    Tb136,
    Tb137,
    Tb141m,
    Tb145m,
    Tb146m1,
    Dy138,
    Dy157m,
    Ho148m1,
    Ho155m,
    Er157m,
    Yb148,
    Yb171m,
    Yb175m,
    Lu153m,
    Lu155m1,
    Lu161m,
    Lu177m1,
    Lu179m,
    Lu180m,

    B6,
    Ga56,
    Ga57,
    Ga58,
    In114m1,
    Tl179m,
    Tl181m,
    Tl183m,
    Tl188m1,
    Tl198m1,
    Tl199m,
    Tl200m,
    Tl201m,

    Ac208m,
    Ac216m,
    Pa217m,
    Pa240,
    U220,
    Am231,
    Am242m1,
    Am248,
    Am249,
    Cm244m,
    Bk235,
    Bk254,
    Es247m,
    Es258,
    Fm260,
    Md245m,
    Md261,
    No261,
    Lr263,

    C21,
    Ge58,
    Pb181m,
    Pb205m,

    Rf264,
    Db264,
    Db265,
    Bh263,
    Bh269,
    Hs265m,
    Mt265,
    Mt266m,
    Mt267,
    Mt269,
    Mt271,
    Mt273,
    Ds268,
    Ds270m,
    Ds272,
```

v0.1.3

Derived EnumString to help the convert enums matching that into strings.

v0.1.2

ensured that Nuclide is compatible with Serde

v0.1.1

fixed that the Nuclide class was not publicly available

v0.1.0

First version

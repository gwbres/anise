/*
 * ANISE Toolkit
 * Copyright (C) 2021 Christopher Rabotin <christopher.rabotin@gmail.com> et al. (cf. AUTHORS.md)
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Documentation: https://nyxspace.com/
 */

pub mod celestial_bodies {
    /// Source bytes: `Mercury`
    pub const MERCURY: u32 = 753059387;
    /// Source bytes: `Venus`
    pub const VENUS: u32 = 2760147288;
    /// Source bytes: `Earth-Moon Barycenter`
    pub const EARTH_MOON_BARYCENTER: u32 = 46073813;
    /// Source bytes: `Mars Barycenter`
    pub const MARS_BARYCENTER: u32 = 1223981629;
    /// Source bytes: `Jupyter Barycenter`
    pub const JUPITER_BARYCENTER: u32 = 2905700239;
    /// Source bytes: `Saturn Barycenter`
    pub const SATURN_BARYCENTER: u32 = 2400246439;
    /// Source bytes: `Uranus Barycenter`
    pub const URANUS_BARYCENTER: u32 = 1449143244;
    /// Source bytes: `Neptune Barycenter`
    pub const NEPTUNE_BARYCENTER: u32 = 199396881;
    /// Source bytes: `Pluto Barycenter`
    pub const PLUTO_BARYCENTER: u32 = 1544737610;
    /// Source bytes: `Sun`
    pub const SUN: u32 = 1777960983;
    /// Source bytes: `Luna`
    pub const LUNA: u32 = 1668777413;
    /// Source bytes: `Earth`
    pub const EARTH: u32 = 2330221028;
}

pub mod orientations {
    /// Source bytes: `J2000`
    pub const J2000: u32 = 1404527632;
}

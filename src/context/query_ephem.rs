use crate::constants::celestial_objects::SOLAR_SYSTEM_BARYCENTER;
/*
 * ANISE Toolkit
 * Copyright (C) 2021 Christopher Rabotin <christopher.rabotin@gmail.com> et al. (cf. AUTHORS.md)
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Documentation: https://nyxspace.com/
 */
use crate::hifitime::Epoch;
use crate::{
    asn1::{context::AniseContext, ephemeris::Ephemeris},
    errors::{AniseError, IntegrityErrorKind},
    frame::Frame,
};

/// **Limitation:** no translation or rotation may have more than 8 nodes.
pub const MAX_TREE_DEPTH: usize = 8;

impl<'a> AniseContext<'a> {
    /// Try to return the ephemeris for the provided index, or returns an error.
    pub fn try_ephemeris_data(&self, idx: usize) -> Result<&'a Ephemeris, AniseError> {
        self.ephemeris_data
            .get(idx)
            .ok_or(AniseError::IntegrityError(IntegrityErrorKind::LookupTable))
    }

    /// Try to return the orientation for the provided index, or returns an error.
    pub fn try_orientation_data(&self, idx: usize) -> Result<&'a Ephemeris, AniseError> {
        self.orientation_data
            .get(idx)
            .ok_or(AniseError::IntegrityError(IntegrityErrorKind::LookupTable))
    }

    /// Try to construct the path from the source frame all the way to the solar system barycenter
    pub fn try_ephemeris_path(
        &self,
        source: &Frame,
    ) -> Result<(usize, [Option<u32>; MAX_TREE_DEPTH]), AniseError> {
        // Build a tree, set a fixed depth to avoid allocations
        let mut of_path = [None; MAX_TREE_DEPTH];
        let mut of_path_len = 0;
        let mut prev_ephem_hash = source.ephemeris_hash;
        for _ in 0..MAX_TREE_DEPTH {
            // The solar system barycenter has a hash of 0.
            // TODO: Find a way to specify the true root of the context.
            let idx = self.ephemeris_lut.index_for_hash(&prev_ephem_hash)?;
            let parent_hash = self.try_ephemeris_data(idx.into())?.parent_ephemeris_hash;
            of_path[of_path_len] = Some(parent_hash);
            of_path_len += 1;
            if parent_hash == SOLAR_SYSTEM_BARYCENTER {
                return Ok((of_path_len, of_path));
            }
            prev_ephem_hash = parent_hash;
        }
        Err(AniseError::MaxTreeDepth)
    }

    /// Returns the position vector and velocity vector needed to translate the `from_frame` to the `to_frame`.
    pub fn translate_from_to(
        &self,
        from_frame: Frame,
        to_frame: Frame,
        epoch: Epoch,
    ) -> Result<([f64; 3], [f64; 3]), AniseError> {
        if from_frame == to_frame {
            // Both frames match, return a vector of zeros.
            return Ok(([0.0; 3], [0.0; 3]));
        }
        // Grab the paths
        let (of_path_len, of_path) = self.try_ephemeris_path(&from_frame)?;
        let (wrt_path_len, wrt_path) = self.try_ephemeris_path(&to_frame)?;
        // Now that we have the paths, we can find the matching origin. (I can probably get that from the Nyx code)

        todo!()
    }

    /// Provided a state with its origin and orientation, returns that state with respect to the requested frame
    pub fn state_in_frame(
        &self,
        position_km: [f64; 3],
        velocity_kmps: [f64; 3],
        wrt_frame: Frame,
        epoch: Epoch,
    ) -> Result<[f64; 6], AniseError> {
        todo!()
    }
}

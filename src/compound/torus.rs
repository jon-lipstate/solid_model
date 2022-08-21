use super::circle;
use crate::{euler::rsweep, structs::Solid};

pub fn torus(solid_id: usize, r1: f32, r2: f32, nf1: usize, nf2: usize) -> Result<Solid, String> {
    let mut s = circle(solid_id, 0., r1, r2, 0., nf2)?;
    rsweep(&mut s, nf1)?;

    Ok(s)
}

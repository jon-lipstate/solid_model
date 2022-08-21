use super::arc;
use crate::{
    euler::{mvfs, rsweep},
    structs::Solid,
};

pub fn ball(r: f32, nver: usize, nhor: usize) -> Result<Solid, String> {
    let mut s = mvfs(1, 1, -r, 0., 0.);
    arc(&mut s, 1, 1, 0., 0., 1., 1., 0., 180., nver)?;
    rsweep(&mut s, nhor)?;

    Ok(s)
}

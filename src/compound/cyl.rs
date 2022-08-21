use super::circle;
use crate::{euler::sweep, structs::Solid};

pub fn cyl(solid_id: usize, radius: f32, h: f32, n: usize) -> Result<Solid, String> {
    let s = circle(solid_id, 0., 0., radius, h, n)?;
    let f = s.find_face(1)?.unwrap();

    sweep(f, 0., 0., h);

    Ok(s)
}

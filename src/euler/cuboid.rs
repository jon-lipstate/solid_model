use super::{mef, mev, mvfs, sweep};
use crate::structs::Solid;

pub fn cuboid(dx: f32, dy: f32, dz: f32) -> Result<Solid, String> {
    let s = mvfs(1, 1, 0., 0., 0.);
    mev(&s, 1, 1, 2, dx, 0., 0.)?;

    mev(&s, 1, 2, 3, dx, dy, 0.)?;
    mev(&s, 1, 3, 4, 0., dy, 0.)?;
    mef(&s, 1, 4, 1, 2)?;
    let f = s.find_face(1)?.unwrap();
    sweep(f, 0.0, 0.0, dz);

    Ok(s)
}

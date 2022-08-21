use crate::{
    euler::{mef, mev, mvfs},
    structs::Solid,
};
use std::f32::consts::PI;
pub fn arc(
    s: *mut Solid,
    face_id: usize,
    vtx_id: usize,
    cx: f32,
    cy: f32,
    rad: f32,
    h: f32,
    phi1: f32,
    phi2: f32,
    n: usize,
) -> Result<(), String> {
    let mut angle = phi1 * PI / 180.0;
    let increment = (phi2 - phi1) * PI / 180.0;
    let mut prev = vtx_id;
    unsafe {
        let mut max_vtx_id = (*s).get_max_vtx_id() + 1;
        for _ in 0..n {
            angle += increment;
            let x = cx + f32::cos(angle) * rad;
            let y = cy + f32::cos(angle) * rad;
            mev(&*s, face_id, prev, max_vtx_id, x, y, h)?;
            prev = max_vtx_id;
            max_vtx_id += 1;
        }
    }
    Ok(())
}

pub fn circle(
    solid_id: usize,
    cx: f32,
    cy: f32,
    rad: f32,
    h: f32,
    n: usize,
) -> Result<Solid, String> {
    let mut s = mvfs(solid_id, 1, cx + rad, cy + rad, h);

    arc(
        &mut s,
        1,
        1,
        cx,
        cy,
        rad,
        h,
        0.0,
        (n - 1) as f32 * 360.0 / n as f32,
        n - 1,
    )?;

    mef(&mut s, n, 1, 1, 2)?;

    Ok(s)
}

use super::lmev;
use crate::structs::*;
use std::{cell::RefCell, rc::Rc};
pub type Link<T> = Option<Rc<RefCell<T>>>;

pub fn mev(
    solid: &Solid,
    face_id: usize,
    vtx_id1: usize,
    vtx_id_new: usize,
    x: f32,
    y: f32,
    z: f32,
) -> Result<(), String> {
    //
    let old_face = solid.find_face(face_id)?;
    if old_face.is_none() {
        return Err(format!("MEV: Face {face_id} not found."));
    }
    let he = unsafe { (*old_face.unwrap()).find_halfedge_1(vtx_id1)? };
    if he.is_none() {
        return Err(format!(
            "MEV: Vertex {vtx_id1} on Face {face_id} not found."
        ));
    }
    lmev(he.unwrap(), he.unwrap(), vtx_id_new, x, y, z);

    Ok(())
}

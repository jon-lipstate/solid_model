use super::low_level::{lkef, lmef};
use crate::{euler::mate, structs::Solid};

pub fn mef(
    s: &Solid,
    vtx_id1: usize,
    vtx_id2: usize,
    face_id1: usize,
    face_id2: usize,
) -> Result<(), String> {
    unsafe {
        let old_face = (*s).find_face(face_id1)?;
        if old_face.is_none() {
            return Err(format!("mef: face [{face_id1}] not found."));
        }
        let h1 = (*old_face.unwrap()).find_halfedge_1(vtx_id1)?;
        if h1.is_none() {
            return Err(format!(
                "mef: vertex [{}] not found in face [{}]",
                vtx_id1,
                (*old_face.unwrap()).id
            ));
        }
        let mut h2 = h1.unwrap();
        if (*(*h2).vertex).id != vtx_id2 {
            loop {
                h2 = (*h2).next;
                if h2 == h1.unwrap() {
                    return Err(format!("mef:[{}] vertex not found.", vtx_id2));
                }
                if (*(*h2).vertex).id != vtx_id2 {
                    break;
                }
            }
        }
        let _new_face = lmef(h1.unwrap(), h2, face_id2);
        println!("mef: new_face goes nowhere..?");
        Ok(())
    }
}

pub fn kef(s: *mut Solid, vtx_id1: usize, vtx_id2: usize, face_id: usize) -> Result<(), String> {
    unsafe {
        let f = (*s).find_face(face_id)?;
        if f.is_none() {
            return Err(format!("kef: face {face_id} not found"));
        }
        let he = (*f.unwrap()).find_halfedge_2(vtx_id1, vtx_id2)?;
        if he.is_none() {
            return Err(format!(
                "kef: edge {vtx_id1}-{vtx_id2} on face {face_id} not found"
            ));
        }
        lkef(he.unwrap(), mate(he.unwrap()));
    }
    unimplemented!();
    Ok(())
}

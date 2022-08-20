use crate::Solid;

use super::low_level::lmef;

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

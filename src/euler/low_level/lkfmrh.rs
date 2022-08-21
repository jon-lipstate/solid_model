use crate::structs::{delete, Face, Loop};
pub fn lkfmrh(f1: *mut Face, f2: *mut Face) {
    unsafe {
        let mut lp = (*f2).loop_list;
        while !lp.is_null() {
            delete(&mut lp, f2);
            addlist(lp, f1);
            lp = (*f2).loop_list;
        }
        let s = (*f1).solid;
        delete(&mut f2, s);
    }
}

pub fn lkfkrh(lp: *mut Loop, face_id: usize) {
    unsafe {
        let new_face = Face::new(face_id, (*(*lp).face).solid);
        (*new_face).outer_loop = lp;
        delete(&mut lp, (*lp).face);
        addlist(lp, new_face);
    }
}

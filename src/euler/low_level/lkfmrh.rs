use crate::structs::{delete, Face, Loop};
pub fn lkfmrh(f1: *mut Face, f2: &mut *mut Face) {
    unsafe {
        let mut lp = (*(*f2)).loop_list;
        while !lp.is_null() {
            delete(&mut lp);
            (*f1).add_loop(lp);
            lp = (*(*f2)).loop_list;
        }
        let s = (*f1).solid;
        (*s).remove_face(*f2);
        delete(f2);
    }
}

pub fn lkfkrh(lp: &mut *mut Loop, face_id: usize) {
    unsafe {
        let s = (*(*(*lp)).face).solid;
        let new_face = (*s).new_face(face_id);
        (*new_face).outer_loop = *lp;
        (*new_face).add_loop(*lp);
        delete(lp);
    }
}

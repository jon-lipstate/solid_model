use crate::structs::Face;

use super::{
    low_level::{lmef, lmev},
    mate,
};

pub fn sweep(f: *mut Face, dx: f32, dy: f32, dz: f32) {
    unsafe {
        let mut next_face_id = (*(*f).solid).get_max_face_id() + 1;
        let mut next_vtx_id = (*(*f).solid).get_max_vtx_id() + 1;

        let mut l = (*f).loop_list;
        while !l.is_null() {
            let h_first = (*l).ledg;
            let mut scan = (*h_first).next;
            let mut v = (*(*scan).vertex).coords;
            lmev(scan, scan, next_vtx_id, v.0 + dx, v.1 + dy, v.2 + dz);
            next_vtx_id += 1;

            while scan != h_first {
                v = (*(*(*scan).next).vertex).coords;
                lmev(
                    (*scan).next,
                    (*scan).next,
                    next_vtx_id,
                    v.0 + dx,
                    v.1 + dy,
                    v.2 + dz,
                );
                next_vtx_id += 1;
                lmef((*scan).prev, (*(*scan).next).next, next_face_id);
                next_face_id += 1;
                scan = (*mate((*scan).next)).next;
            }
            lmef((*scan).prev, (*(*scan).next).next, next_face_id);
            next_face_id += 1;
            l = (*l).next;
        }
    }
}

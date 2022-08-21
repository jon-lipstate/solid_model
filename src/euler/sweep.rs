use super::{
    low_level::{lkef, lkfmrh, lmef, lmev},
    mate,
};
use crate::structs::{Face, HalfEdge, Solid};

pub fn sweep(f: *mut Face, dx: f32, dy: f32, dz: f32) {
    unsafe {
        let mut next_face_id = (*(*f).solid).get_max_face_id() + 1;
        let mut next_vtx_id = (*(*f).solid).get_max_vtx_id() + 1;

        let mut l = (*f).loop_list;

        while !l.is_null() {
            let h_first = (*l).half_edge;
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

                if !scan.is_null() {
                    let snext = (*scan).next;
                    if !snext.is_null() {
                        let _sn2 = (*snext).next;
                    }
                }

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

pub fn rsweep(s: *mut Solid, n_faces: usize) -> Result<(), String> {
    println!("rsweep");
    println!("{:?}", s);
    println!("{:?}", n_faces);

    unimplemented!();
    // let mut n_faces_mut = n_faces;
    // unsafe {
    //     let mut is_closed = false;
    //     let he: *mut HalfEdge;
    //     let mut max_vtx_id = (*s).get_max_vtx_id() + 1;
    //     let mut max_face_id = (*s).get_max_face_id() + 1;
    //     let head_face = (*s).faces_start;

    //     if !(*(*s).faces_start).next.is_null() {
    //         // solidls(s, 2);
    //         is_closed = true;
    //         he = (*(*(*s).faces_start).loop_list).half_edge;
    //         let coord = (*(*he).vertex).coords;
    //         lmev(he, mate(he), max_vtx_id, coord.0, coord.1, coord.2);
    //         max_vtx_id += 1;
    //         lkef((*he).prev, mate((*he).prev));
    //         // solidls(s, 1);
    //     }
    //     let mut first = (*(*(*s).faces_start).outer_loop).half_edge;
    //     while (*first).edge != (*(*first).next).edge {
    //         first = (*first).next;
    //     }

    //     let mut last = (*first).next;

    //     while (*last).edge != (*(*last).next).edge {
    //         last = (*last).next;
    //     }
    //     let cfirst = first;
    //     let mut scan;
    //     matident(m);
    //     matrotat(m, (360.0 / n_faces as f32), 0., 0.);
    //     loop {
    //         let cfnxt = (*cfirst).next;
    //         vecmult(v, (*(*cfnxt).vertex).coords, m);
    //         lmev(cfnxt, cfnxt, max_vtx_id, v[0], v[1], v[2]);
    //         max_vtx_id += 1;
    //         scan = cfnxt;
    //         while scan != (*last).next {
    //             lmef((*(*scan).prev).prev, (*scan).next, max_face_id);
    //             max_face_id += 1;
    //             scan = mate((*(*scan).next).next);
    //         }
    //         last = scan;
    //         cfirst = mate((*(*cfirst).next).next);
    //         n_faces_mut -= 1; //idk if this is equal to while(--nfaces); todo verify
    //         if n_faces_mut <= 0 {
    //             break;
    //         }
    //     }
    //     let mut tail_face = lmef((*cfirst).next, mate(first), max_face_id);
    //     max_face_id += 1;
    //     while cfirst != scan {
    //         lmef(cfirst, (*(*(*cfirst).next).next).next, max_face_id);
    //         cfirst = (*mate((*cfirst).prev)).prev;
    //     }
    //     if is_closed {
    //         lkfmrh(head_face, &mut tail_face);
    //         (*head_face).loopglue();
    //     }
    // }
    // Ok(())
}

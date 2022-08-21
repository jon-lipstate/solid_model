use crate::{
    euler::delete_halfedge,
    structs::{delete, Edge, HalfEdge, Loop},
};
use std::ptr;

pub fn lkemr(h1: *mut HalfEdge, h2: *mut HalfEdge) {
    unsafe {
        let old_loop = (*h1).lp;
        let new_loop = Loop::new((*old_loop).face);
        let kill_edge = (*h1).edge;
        let mut h3 = (*h1).next;
        (*h1).next = (*h2).next;
        (*(*h2).next).prev = h1;
        (*h2).next = h3;
        (*h3).prev = h2;

        let mut h4 = h2;
        loop {
            (*h4).lp = new_loop;
            h4 = (*h4).next;
            if h4 == h2 {
                break;
            }
        }
        h3 = delete_halfedge(h1);
        (*old_loop).half_edge = h3;
        h4 = delete_halfedge(h2);
        (*new_loop).half_edge = h4;

        (*(*h3).vertex).half_edge = match (*h3).edge.is_null() {
            true => ptr::null_mut(),
            false => h3,
        };
        (*(*h4).vertex).half_edge = match (*h4).edge.is_null() {
            true => ptr::null_mut(),
            false => h4,
        };
        let s = (*(*old_loop).face).solid;
        delete(&mut kill_edge, s);
    }
}

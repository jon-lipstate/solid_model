use std::ptr;

use add_halfedge::delete_halfedge;

use crate::{
    euler::{add_halfedge, Dir},
    structs::{delete, Edge, Face, HalfEdge, Loop},
};

pub fn lmef(h1: *mut HalfEdge, h2: *mut HalfEdge, face_id: usize) -> *mut Face {
    unsafe {
        let s = (*(*(*h1).lp).face).solid;
        let mut new_face = Face::new(face_id, s);
        let mut new_loop = Loop::new(new_face);
        let new_edge = Edge::new(s);
        (*new_face).outer_loop = new_loop;

        //Set all halfedges to be pointing to this new loop:
        let mut he = h1;
        while he != h2 {
            (*he).lp = new_loop;
            he = (*he).next;
        }
        let new_h1 = add_halfedge(new_edge, (*h2).vertex, h1, Dir::CW);
        let new_h2 = add_halfedge(new_edge, (*h1).vertex, h2, Dir::CCW);

        (*(*new_h1).prev).next = new_h2;
        (*(*new_h2).prev).next = new_h1;
        let temp = (*new_h1).prev;
        (*new_h1).prev = (*new_h2).prev;
        (*new_h2).prev = temp;
        (*new_loop).half_edge = new_h1;
        (*(*h2).lp).half_edge = new_h2;

        new_face
    }
}

pub fn lkef(h1: *mut HalfEdge, h2: *mut HalfEdge) {
    unsafe {
        //stash self-references to enable deletion:
        let mut dh1 = (*(*h1).prev).next;
        let mut dh2 = (*(*h2).prev).next;
        // let s = (*(*(*h1).lp).face).solid;
        let l1 = (*h1).lp;
        let f1 = (*l1).face;
        let mut l2 = (*h2).lp;
        let mut f2 = (*l2).face;
        let mut l = (*f2).loop_list;
        while !l.is_null() {
            // i dont understand this one.. come back with breakpoint
            (*f2).remove_loop(l);
            (*f1).add_loop(l);
            l = (*f2).loop_list;
        }
        let mut he = (*l2).half_edge;
        loop {
            (*he).lp = l1;
            he = (*he).next;
            if he == (*l2).half_edge {
                break;
            }
        }
        //Probable can use .remove_self() ...?
        (*(*h1).prev).next = h2;
        (*(*h2).prev).next = h1;
        he = (*h2).prev;
        (*h2).prev = (*h1).prev;
        (*h1).prev = he;
        delete_halfedge(&mut dh2);
        delete_halfedge(&mut dh1);

        (*(*h2).vertex).half_edge = (*h1).next;

        if !(*(*(*h2).vertex).half_edge).edge.is_null() {
            (*(*h2).vertex).half_edge = ptr::null_mut();
        }
        if !(*(*(*h1).vertex).half_edge).edge.is_null() {
            (*(*h1).vertex).half_edge = ptr::null_mut();
        }
        (*l1).half_edge = (*h1).next;

        delete(&mut f2);
        delete(&mut l2);
        delete(&mut (*h2).edge);
    }
}

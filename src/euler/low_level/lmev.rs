use std::ptr;

use crate::{
    euler::{add_halfedge::*, mate, Dir},
    structs::*,
};
pub fn lmev(h1: *mut HalfEdge, h2: *mut HalfEdge, vtx_id: usize, x: f32, y: f32, z: f32) {
    unsafe {
        //Ref to parent Solid:
        let s = (*(*(*h1).lp).face).solid;
        //new vertex:
        let mut vtx = (*s).new_vertex(vtx_id, x, y, z);
        //new Edge:
        let edge = (*s).new_edge();

        let mut he = h1;
        while he != h2 {
            (*he).vertex = vtx;
            he = (*mate(he)).next;
        }
        add_halfedge(edge, (*h2).vertex, h2, Dir::CCW); //plus
        add_halfedge(edge, vtx, h1, Dir::CW); //minus
        (*vtx).half_edge = (*h2).prev;
        (*(*h2).vertex).half_edge = h2;
    }
}

pub fn lkev(h1: *mut HalfEdge, h2: *mut HalfEdge) {
    unsafe {
        //stash self-references to enable deletion:
        let mut dh1 = (*(*h1).prev).next;
        let mut dh2 = (*(*h2).prev).next;

        let mut he = (*h2).next;
        while he != h1 {
            (*he).vertex = (*h2).vertex;
            he = (*mate(he)).next;
        }
        (*(*h1).lp).half_edge = delete_halfedge(&mut dh1);
        (*(*h2).lp).half_edge = delete_halfedge(&mut dh2);
        (*(*h2).vertex).half_edge = (*h2).next;

        if !(*(*(*h2).vertex).half_edge).edge.is_null() {
            (*(*h2).vertex).half_edge = ptr::null_mut();
        }
        // let s = (*(*(*h1).lp).face).solid;
        delete(&mut (*h1).edge);
        delete(&mut (*h1).vertex);
    }
}

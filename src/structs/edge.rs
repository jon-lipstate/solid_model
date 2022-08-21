use super::{HalfEdge, Solid};
use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::{self},
};

#[derive(Debug, PartialEq)]
pub struct Edge {
    ///ptr to right halfedge (he1)
    pub he_rt: *mut HalfEdge,
    ///ptr to left halfedge  (he2)
    pub he_lt: *mut HalfEdge,
    // ///ptr to next edge      (nexte)
    pub next: *mut Edge,
    // ///ptr to prev edge      (preve)
    pub prev: *mut Edge,
}

//
impl Edge {
    //globally alloc - ONLY ALLOCATE VIA SOLID::NEW_EDGE
    pub fn new(parent: *mut Solid) -> *mut Edge {
        unsafe {
            let e = alloc(Layout::new::<Edge>()) as *mut Edge;

            let ev: Edge = Edge {
                he_rt: ptr::null_mut(),
                he_lt: ptr::null_mut(),
                next: (*parent).edges_start,
                prev: ptr::null_mut(),
            };
            e.write(ev);

            e
        }
    }
}

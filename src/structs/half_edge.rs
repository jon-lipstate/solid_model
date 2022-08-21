use super::{Edge, Loop, Vertex};
use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::{self},
};
#[derive(Debug, PartialEq)]
pub struct HalfEdge {
    ///ptr to parent edge (edg)
    pub edge: *mut Edge,
    ///ptr to start vtx   (vtx)
    pub vertex: *mut Vertex,
    ///back ptr to loop   (wloop)
    pub lp: *mut Loop,
    ///ptr to next he     (nxt)
    pub next: *mut HalfEdge,
    // ///ptr to prev he     (prv)
    pub prev: *mut HalfEdge,
}
//
impl HalfEdge {
    /// Allocates a new halfedge
    pub fn new() -> *mut HalfEdge {
        unsafe {
            let h = alloc(Layout::new::<HalfEdge>()) as *mut HalfEdge;

            let hv = HalfEdge {
                edge: ptr::null_mut(),
                vertex: ptr::null_mut(),
                lp: ptr::null_mut(),
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            };
            h.write(hv);

            h
        }
    }
    pub fn set_next(&mut self, item: *mut HalfEdge) -> Result<(), String> {
        if item.is_null() {
            return Err(format!("Null Ptr"));
        }
        self.next = item;

        Ok(())
    }
    pub fn set_prev(&mut self, item: *mut HalfEdge) -> Result<(), String> {
        if item.is_null() {
            return Err(format!("Null Ptr"));
        }
        self.prev = item;

        Ok(())
    }
    pub fn remove_self(&mut self) -> Result<(), String> {
        if self.prev.is_null() || self.next.is_null() {
            return Err(format!("Null Ptr"));
        }
        let prev = self.prev;
        let next = self.next;
        unsafe {
            (*prev).next = next;
            (*next).prev = prev;
        }
        println!("should i exec deallocate on self??");
        Ok(())
    }
    pub fn insert_after(new: *mut HalfEdge, existing: *mut HalfEdge) {
        unsafe {
            (*new).prev = existing;
            (*new).next = (*existing).next;
            (*(*existing).next).prev = new;
            (*existing).next = new;
        }
    }
    pub fn insert_before(new: *mut HalfEdge, existing: *mut HalfEdge) {
        unsafe {
            let mut old_prev = (*existing).prev;
            if !old_prev.is_null() {
                (*old_prev).next = new;
            }
            (*new).prev = (*existing).prev;
            (*existing).prev = new;
            (*new).next = existing;
        }
    }
    pub fn is_matched(a: *mut HalfEdge, b: *mut HalfEdge) -> bool {
        unsafe {
            let av = (*a).vertex;
            let bv = (*b).vertex;
            Vertex::is_coincident(av, bv)
        }
    }
}

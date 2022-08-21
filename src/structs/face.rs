use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::{self},
};

use super::{delete, HalfEdge, Loop, Solid};

#[derive(Debug, PartialEq)]
pub struct Face {
    ///face identifier (faceno)
    pub id: usize,
    ///back ptr to solid (fsolid)
    pub solid: *mut Solid,
    ///ptr to outer loop (flout)
    pub outer_loop: *mut Loop,
    ///ptr to list of loops (floops)
    pub loop_list: *mut Loop,
    ///face equation (feq)
    pub face_eq: Option<(f32, f32, f32, f32)>,
    // ///ptr to next face (nextf)
    pub next: *mut Face,
    // ///ptr to prev face (prevf)
    pub prev: *mut Face,
}
//
impl Face {
    /// Globally Allocated- ONLY ALLOCATE VIA SOLID::NEW_FACE
    pub fn new(id: usize, parent: *mut Solid) -> *mut Face {
        unsafe {
            let f = alloc(Layout::new::<Face>()) as *mut Face;
            let fv = Face {
                id,
                solid: parent,
                outer_loop: ptr::null_mut(),
                loop_list: ptr::null_mut(),
                face_eq: None,
                next: (*parent).faces_start,
                prev: ptr::null_mut(),
            };
            f.write(fv);

            f
        }
    }
    /// Allocates new loop & adds to self
    pub fn new_loop(&mut self) -> *mut Loop {
        let lp = Loop::new(self);
        self.add_loop(lp);

        lp
    }
    /// registers a pre-existing loop (eg. transfer ownership)
    pub fn add_loop(&mut self, lp: *mut Loop) {
        unsafe {
            (*lp).next = self.loop_list;
            (*lp).prev = ptr::null_mut();
            if !self.loop_list.is_null() {
                (*self.loop_list).prev = lp;
            }
            self.loop_list = lp;
            (*lp).face = self;
        }
    }
    /// removes loop from face ptrs, does not deallocate
    pub fn remove_loop(&mut self, lp: *mut Loop) {
        unsafe {
            if !(*lp).prev.is_null() {
                (*(*lp).prev).next = (*lp).next;
            }
            if !(*lp).next.is_null() {
                (*(*lp).next).prev = (*lp).prev;
            }
            if lp == self.loop_list {
                self.loop_list = (*lp).next;
            }
        }
    }

    pub fn find_halfedge_1(&self, vtx_id: usize) -> Result<Option<*mut HalfEdge>, String> {
        if self.loop_list.is_null() {
            return Err(format!("[{}] Face.loop_list nullptr", self.id));
        }
        unsafe {
            let mut l = self.loop_list;
            while !l.is_null() {
                let mut h = (*l).half_edge;
                loop {
                    let vid = (*(*h).vertex).id;
                    if vid == vtx_id {
                        return Ok(Some(h));
                    }
                    h = (*h).next;
                    if h == (*l).half_edge {
                        break;
                    }
                }
                l = (*l).next;
            }
        }
        Ok(None)
    }
    //
    pub fn find_halfedge_2(
        &self,
        vtx_id1: usize,
        vtx_id2: usize,
    ) -> Result<Option<*mut HalfEdge>, String> {
        if self.loop_list.is_null() {
            return Err(format!("[{}] Face.loop_list nullptr", self.id));
        }
        unsafe {
            let mut l = self.loop_list;
            while !l.is_null() {
                let mut h = (*l).half_edge;
                loop {
                    if (*h).next.is_null() {
                        break;
                    }
                    let v1 = (*(*h).vertex).id;
                    let v2 = (*(*(*h).next).vertex).id;
                    if v1 == vtx_id1 && v2 == vtx_id2 {
                        return Ok(Some(h));
                    }
                    h = (*h).next;
                    if h == (*l).half_edge {
                        break;
                    }
                }
                l = (*l).next;
            }
        }
        Ok(None)
    }

    //
}

use super::{delete, HalfEdge, Loop, Solid};
use crate::euler::{
    lkef, lmef, lmekr,
    low_level::{lkemr, lkev},
    mate,
};
use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::{self},
};

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
    pub fn loopglue(&mut self) {
        unsafe {
            let mut h1 = (*self.loop_list).half_edge;
            let mut h2 = (*(*self.loop_list).next).half_edge;
            while !HalfEdge::is_matched(h1, h2) {
                h2 = (*h2).next;
            }
            lmekr(h1, h2);
            lkev((*h1).prev, (*h2).prev);
            while (*h1).next != h2 {
                let h1_next = (*h1).next;
                lmef((*h1).next, (*h1).prev, 0);
                lkev((*h1).next, mate((*h1).next));
                lkef(mate(h1), h1);
                h1 = h1_next;
            }
            lkef(mate(h1), h1);
        }
    }
    //Program 13.1; P219
    fn equation(&self, lp: *mut Loop) -> Result<(f64, f64, f64, f64), String> {
        unsafe {
            let mut eqn = (0., 0., 0., 0.);
            let mut a = 0.;
            let mut b = 0.;
            let mut c = 0.;
            let mut xc = 0.;
            let mut yc = 0.;
            let mut zc = 0.;
            let mut len = 0;
            let he = (*lp).half_edge;
            loop {
                let vc = (*(*he).vertex).coords;
                let nvc = (*(*(*he).next).vertex).coords;
                let xi = vc.0 as f64;
                let yi = vc.1 as f64;
                let zi = vc.2 as f64;
                let xj = nvc.0 as f64;
                let yj = nvc.1 as f64;
                let zj = nvc.2 as f64;

                a += (yi - yj) * (zi + zj);
                b += (zi - zj) * (xi + xj);
                c += (xi - xj) * (yi + yj);
                xc += xi;
                yc += yi;
                zc += zi;
                len += 1;
                if (*he).next == (*lp).half_edge {
                    break;
                }
            }
            let norm = f64::sqrt(a * a + b * b + c * c);
            if norm != 0. {
                eqn.0 = a / norm;
                eqn.1 = b / norm;
                eqn.2 = c / norm;
                eqn.3 = (eqn.0 * xc + eqn.1 * yc + eqn.2 * zc) / -len as f64;
                return Ok(eqn);
            } else {
                return Err(format!("faceeq: null face {}", self.id));
            }
        }
    }
    //
}

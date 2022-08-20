use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::{self},
};

use super::{HalfEdge, Loop, Solid};

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
    /// Globally Allocated Ptr to a Face
    // pub fn new(id: usize, parent: *mut Solid) -> Face {
    //     unsafe {
    //         // let mut f = alloc(Layout::new::<Face>()) as *mut Face;
    //         let mut f = Face {
    //             id,
    //             solid: parent,
    //             outer_loop: ptr::null_mut(),
    //             loop_list: ptr::null_mut(),
    //             face_eq: None,
    //             next: (*parent).faces_start,
    //             prev: ptr::null_mut(),
    //         };
    //         // f.write(fv);

    //         if !(*parent).faces_start.is_null() {
    //             (*(*parent).faces_start).prev = &mut f;
    //         }
    //         (*parent).faces_start = &mut f;
    //         f
    //     }
    // }

    /// Globally Allocated Ptr to a Face
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

            if !(*parent).faces_start.is_null() {
                (*(*parent).faces_start).prev = f;
            }
            (*parent).faces_start = f;
            // case FACE:
            //     which->f.nextf = where->s.sfaces;
            //     which->f.prevf = (Face *)NIL;
            //     if (where->s.sfaces)
            //             where->s.sfaces->prevf = (Face *)which;
            //     where->s.sfaces = (Face *)which;
            //     which->f.fsolid = (Solid *)where;
            //     break;
            f
        }
    }

    pub fn delete(f: &mut *mut Face) {
        println!("TODO: Destroy children Loops, remove self from ll");
        unsafe {
            dealloc(*f as *mut u8, Layout::new::<Face>());
            *f = ptr::null_mut();
        }
    }

    pub fn find_halfedge_1(&self, vtx_id: usize) -> Result<Option<*mut HalfEdge>, String> {
        if self.loop_list.is_null() {
            return Err(format!("[{}] Face.loop_list nullptr", self.id));
        }
        unsafe {
            let mut l = self.loop_list;
            while !l.is_null() {
                let mut h = (*l).ledg;
                loop {
                    let vid = (*(*h).vertex).id;
                    if vid == vtx_id {
                        return Ok(Some(h));
                    }
                    h = (*h).next;
                    if h == (*l).ledg {
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
                let mut h = (*l).ledg;
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
                    if h == (*l).ledg {
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

// #[derive(Debug, PartialEq)]
// pub struct Face {
//     ///face identifier (faceno)
//     pub id: usize,
//     ///back ptr to solid (fsolid)
//     pub solid: Rc<RefCell<Solid>>,
//     ///ptr to outer loop (flout)
//     pub outer_loop: Option<Link<Loop>>,
//     ///ptr to list of loops (floops)
//     pub loop_list: LinkedList<Link<Loop>>,
//     ///face equation (feq)
//     pub face_eq: (f32, f32, f32, f32),
//     // ///ptr to next face (nextf)
//     // pub next: Link<Face>,
//     // ///ptr to prev face (prevf)
//     // pub prev: Link<Face>,
// }
// //
// impl Face {
//     pub fn new(id: usize, parent: Rc<RefCell<Solid>>) -> Rc<RefCell<Face>> {
//         Rc::new(RefCell::new(Face {
//             id,
//             solid: Rc::clone(&parent),
//             outer_loop: None,
//             loop_list: LinkedList::new(),
//             face_eq: (0., 0., 0., 0.),
//             // next: None,
//             // prev: None,
//         }))
//     }

//     pub fn find_halfedge_1(&self, vtx_id: usize) -> Option<Link<HalfEdge>> {
//         let mut lol = self.loop_list.iter();
//         let mut he;
//         loop {
//             let lp = match lol.next() {
//                 None => break,
//                 Some(lp) => lp,
//             };
//             let lpll = &lp.borrow().ledg;
//             let first = lpll.front().unwrap();
//             let mut li = lpll.iter();

//             loop {
//                 he = match li.next() {
//                     None => break,
//                     Some(he) => he,
//                 };
//                 let v_id = match &he.borrow().vertex {
//                     None => break,
//                     Some(v) => v.borrow().id,
//                 };
//                 if v_id == vtx_id {
//                     return Some(Rc::clone(&he));
//                 }
//                 if he == first {
//                     break;
//                 }
//             } // end HE loop
//         } //End LP loop
//     }
//     //
//     pub fn find_halfedge_2(&self, vtx_id1: usize, vtx_id2: usize) -> Option<Link<HalfEdge>> {
//         let mut lol = self.loop_list.iter();
//         let mut he;
//         loop {
//             let lp = match lol.next() {
//                 None => break,
//                 Some(lp) => lp,
//             };
//             let lpll = &lp.borrow().ledg;
//             let first = lpll.front().unwrap();
//             let mut li = lpll.iter().peekable();

//             loop {
//                 he = match li.next() {
//                     None => break,
//                     Some(he) => he,
//                 };
//                 let v_id = match &he.borrow().vertex {
//                     None => break,
//                     Some(v) => v.borrow().id,
//                 };
//                 if v_id == vtx_id1 {
//                     //NOW DO STEP #2
//                     let hnext = match li.peek() {
//                         None => break,
//                         Some(he) => *he,
//                     };
//                     let v_id_next = match hnext.borrow().vertex {
//                         None => break,
//                         Some(v) => v.borrow().id,
//                     };
//                     if vtx_id2 == v_id_next {
//                         return Some(Rc::clone(&he));
//                     }
//                 }
//                 if he == first {
//                     break;
//                 }
//             } // end HE loop
//         } //End LP loop
//     }

//     //
// }

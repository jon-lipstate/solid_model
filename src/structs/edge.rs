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
    //globally alloc
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
            if !(*parent).edges_start.is_null() {
                (*(*parent).edges_start).prev = e;
            }
            (*parent).edges_start = e;

            // case EDGE:
            // which->e.nexte = where->s.sedges;
            // which->e.preve = (Edge *)NIL;
            // if (where->s.sedges)
            //         where->s.sedges->preve = (Edge *)which;
            // where->s.sedges = (Edge *)which;
            // break;

            // Case ES.is_null():
            // ES->{P:null,N:(null)}

            // Case !ES.is_null():
            // ES->{P:null,N:->}{P:<-,N:null}

            e
        }
    }
}

// use std::{cell::RefCell, rc::Rc};
// pub type Link<T> = Rc<RefCell<T>>;
// use super::{HalfEdge, Solid};

// #[derive(Debug, PartialEq)]
// pub struct Edge {
//     ///ptr to right halfedge (he1)
//     pub he_rt: Option<Link<HalfEdge>>,
//     ///ptr to left halfedge  (he2)
//     pub he_lt: Option<Link<HalfEdge>>,
//     // ///ptr to next edge      (nexte)
//     // pub next: Link<Edge>,
//     // ///ptr to prev edge      (preve)
//     // pub prev: Link<Edge>,
// }

// //
// impl Edge {
//     pub fn new(parent: Rc<RefCell<Solid>>) -> Rc<RefCell<Edge>> {
//         let mut se = parent.borrow_mut();

//         let rc_e: Rc<RefCell<Edge>> = Rc::new(RefCell::new(Edge {
//             he_rt: None,
//             he_lt: None,
//             // next: None,
//             // prev: None,
//         }));

//         se.edges_start.push_back(Rc::clone(&rc_e));
//         // match &se.edges_start {
//         //     None => se.edges_start = Some(Rc::clone(&rc_e)),
//         //     Some(es) => rc_e.borrow_mut().next = Some(Rc::clone(&es)),
//         // }
//         rc_e
//     }
// }

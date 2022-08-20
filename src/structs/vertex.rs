use super::{HalfEdge, Solid};
use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::{self},
};

#[derive(Debug, PartialEq)]
pub struct Vertex {
    pub id: usize,                    /* ident */
    pub half_edge: *mut HalfEdge,     /* ptr to halfedge */
    pub coords: (f32, f32, f32, f32), /* vtx coords */
    pub next: *mut Vertex,            /* ptr to next vtx */
    pub prev: *mut Vertex,            /* ptr to prev vtx */
}
//
impl Vertex {
    //globally allocated
    pub fn new(parent: *mut Solid, id: usize, x: f32, y: f32, z: f32) -> *mut Vertex {
        unsafe {
            let v = alloc(Layout::new::<Vertex>()) as *mut Vertex;
            let vv = Vertex {
                id,
                half_edge: ptr::null_mut(),
                coords: (x, y, z, 1.),
                next: (*parent).vertices_start,
                prev: ptr::null_mut(),
            };
            v.write(vv);

            if !(*parent).vertices_start.is_null() {
                (*(*parent).vertices_start).prev = v;
            }
            (*parent).vertices_start = v;
            // case VERTEX:
            // which->v.nextv = where->s.sverts;
            // which->v.prevv = (Vertex *)NIL;
            // if (where->s.sverts)
            //         where->s.sverts->prevv = (Vertex *)which;
            // where->s.sverts = (Vertex *)which;
            // break;
            v
        }
    }
    pub fn delete(f: &mut *mut Vertex) {
        println!("TODO: Destroy children Loops, remove self from ll");
        unsafe {
            dealloc(*f as *mut u8, Layout::new::<Vertex>());
            *f = ptr::null_mut();
        }
    }

    // pub fn new(parent: *mut Solid, id: usize, x: f32, y: f32, z: f32) -> Vertex {
    //     unsafe {
    //         let mut v = Vertex {
    //             id,
    //             half_edge: ptr::null_mut(),
    //             coords: (x, y, z, 1.),
    //             next: (*parent).vertices_start,
    //             prev: ptr::null_mut(),
    //         };
    //         if !(*parent).vertices_start.is_null() {
    //             (*(*parent).vertices_start).prev = &mut v;
    //         }
    //         (*parent).vertices_start = &mut v;
    //         // case VERTEX:
    //         // which->v.nextv = where->s.sverts;
    //         // which->v.prevv = (Vertex *)NIL;
    //         // if (where->s.sverts)
    //         //         where->s.sverts->prevv = (Vertex *)which;
    //         // where->s.sverts = (Vertex *)which;
    //         // break;
    //         v
    //     }
    // }
}

// use std::{cell::RefCell, rc::Rc};
// type Link<T> = Option<Rc<RefCell<T>>>;
// use super::HalfEdge;

// #[derive(Debug, PartialEq)]
// pub struct Vertex {
//     pub id: usize,                    /* ident */
//     pub half_edge: Link<HalfEdge>,    /* ptr to halfedge */
//     pub coords: (f32, f32, f32, f32), /* vtx coords */
//     pub next: Link<Vertex>,           /* ptr to next vtx */
//     pub prev: Link<Vertex>,           /* ptr to prev vtx */
// }
// //
// impl Vertex {
//     pub fn new(id: usize, x: f32, y: f32, z: f32) -> Rc<RefCell<Vertex>> {
//         Rc::new(RefCell::new(Vertex {
//             id,
//             half_edge: None,
//             coords: (x, y, z, 1.),
//             next: None,
//             prev: None,
//         }))
//     }
// }

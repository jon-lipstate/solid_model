use super::{Face, HalfEdge};
use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::{self},
};
#[derive(Debug, PartialEq)]
pub struct Loop {
    ///ptr to ring of halfedges (ledg)
    pub half_edge: *mut HalfEdge,
    ///back ptr to face (lface) MAYBE CONST*???
    pub face: *mut Face,
    // ptr to next loop (nextl)
    pub next: *mut Loop,
    // ///ptr to prev loop (prevl)
    pub prev: *mut Loop,
}

impl Loop {
    pub fn new(parent: *mut Face) -> *mut Loop {
        unsafe {
            let lp = alloc(Layout::new::<Loop>()) as *mut Loop;

            let lv = Loop {
                face: parent,
                next: (*parent).loop_list,
                prev: ptr::null_mut(),
                half_edge: ptr::null_mut(),
            };
            lp.write(lv);
            if !(*parent).loop_list.is_null() {
                (*(*parent).loop_list).prev = lp;
            }
            (*parent).loop_list = lp;
            // case LOOP:
            // which->l.nextl = where->f.floops;
            // which->l.prevl = (Loop *)NIL;
            // if (where->f.floops)
            //         where->f.floops->prevl = (Loop *)which;
            // where->f.floops = (Loop *)which;
            // which->l.lface = (Face *)where;
            // break;

            lp
        }
    }
}

// #[derive(Debug, PartialEq)]
// pub struct Loop {
//     ///ptr to ring of halfedges (ledg)
//     pub ledg: LinkedList<Link<HalfEdge>>,
//     ///back ptr to face (lface)
//     pub face: Rc<RefCell<Face>>,
//     // ptr to next loop (nextl)
//     // pub next: Link<Loop>,
//     // ///ptr to prev loop (prevl)
//     // pub prev: Link<Loop>,
// }

// impl Loop {
//     pub fn new(parent: Rc<RefCell<Face>>) -> Rc<RefCell<Loop>> {
//         Rc::new(RefCell::new(Loop {
//             face: Rc::clone(&parent),
//             // next: None,
//             // prev: None,
//             ledg: LinkedList::new(),
//         }))
//     }
// }

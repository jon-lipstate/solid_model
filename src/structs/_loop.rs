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

            lp
        }
    }
}

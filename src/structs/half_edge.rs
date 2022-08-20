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
    pub fn set_next(&mut self, item: *mut HalfEdge) -> Result<(), &str> {
        if item.is_null() {
            return Err("Null Ptr");
        }
        self.next = item;

        Ok(())
    }
    pub fn set_prev(&mut self, item: *mut HalfEdge) -> Result<(), &str> {
        if item.is_null() {
            return Err("Null Ptr");
        }
        self.prev = item;

        Ok(())
    }
    pub fn remove_self(&mut self) -> Result<(), &str> {
        if self.prev.is_null() || self.next.is_null() {
            return Err("Null Ptr");
        }
        let prev = self.prev;
        let next = self.next;
        unsafe {
            (*prev).next = next;
            (*next).prev = prev;
        }
        println!("should i exec drop on self??");
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
            (*((*existing).prev)).next = new;
            (*new).prev = (*existing).prev;
            (*existing).prev = new;
            (*new).next = existing;
        }
    }
}

// use std::{cell::RefCell, rc::Rc};
// pub type Link<T> = Option<Rc<RefCell<T>>>;
// use super::{Edge, Loop, Vertex};

// #[derive(Debug, PartialEq)]
// pub struct HalfEdge {
//     ///ptr to parent edge (edg)
//     pub edge: Link<Edge>,
//     ///ptr to start vtx   (vtx)
//     pub vertex: Link<Vertex>,
//     ///back ptr to loop   (wloop)
//     pub lp: Link<Loop>,
//     ///ptr to next he     (nxt)
//     // pub next: Link<HalfEdge>,
//     // ///ptr to prev he     (prv)
//     // pub prev: Link<HalfEdge>,
// }
// //
// impl HalfEdge {
//     pub fn new() -> Rc<RefCell<HalfEdge>> {
//         let h = HalfEdge {
//             edge: None,
//             vertex: None,
//             lp: None,
//             // next: None,
//             // prev: None,
//         };
//         Rc::new(RefCell::new(h))
//     }
//     pub fn set_next(&mut self, item: Link<HalfEdge>) {
//         match item {
//             None => self.next = None,
//             Some(i) => {
//                 self.next = Some(Rc::clone(&i));
//             }
//         }
//     }
//     pub fn set_prev(&mut self, item: Link<HalfEdge>) {
//         match item {
//             None => self.prev = None,
//             Some(i) => {
//                 self.prev = Some(Rc::clone(&i));
//             }
//         }
//     }
//     // pub fn remove_self(&mut self) {
//     //     match self.prev {
//     //         None => {}
//     //         Some(p) => p.borrow_mut().next = self.next,
//     //     }
//     //     match self.next {
//     //         None => {}
//     //         Some(p) => p.borrow_mut().prev = self.prev,
//     //     }
//     // }
//     // pub fn insert_after(new: Rc<RefCell<HalfEdge>>, item: Rc<RefCell<HalfEdge>>) {
//     //     //new's prev is -1:
//     //     new.borrow_mut().prev = Some(item);
//     //     //new's next is -1's next:
//     //     new.borrow_mut().next = item.borrow().next;
//     //     //set +1's prev to new:
//     //     item.borrow().next.unwrap().borrow_mut().prev = Some(new);
//     //     //set -1's next to new:
//     //     item.borrow_mut().next = Some(new);
//     // }
//     pub fn insert_before(new: Rc<RefCell<HalfEdge>>, existing: Rc<RefCell<HalfEdge>>) {
//         let prev = Rc::clone(&(existing.borrow().prev.as_ref().unwrap()));
//         prev.borrow_mut().next = Some(Rc::clone(&new));
//         new.borrow_mut().prev = Some(Rc::clone(&(prev)));
//         existing.borrow_mut().prev = Some(Rc::clone(&new));
//         new.borrow_mut().next = Some(Rc::clone(&existing));
//     }
// }

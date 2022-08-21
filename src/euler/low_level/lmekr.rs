use crate::{
    euler::{add_halfedge, Dir},
    structs::{Edge, HalfEdge},
};
pub fn lmekr(h1: *mut HalfEdge, h2: *mut HalfEdge) {
    unsafe {
        let l1 = (*h1).lp;
        let l2 = (*h2).lp;
        let f = (*l1).face;
        let n_h1 = (*l2).half_edge;
        loop {
            (*n_h1).lp = l1;
            n_h1 = (*n_h1).next;
            if n_h1 == (*l2).half_edge {
                break;
            }
        }
        let new_edge = Edge::new((*f).solid);
        n_h1 = add_halfedge(new_edge, (*h1).vertex, h1, Dir::CCW);
        let n_h2 = add_halfedge(new_edge, (*h2).vertex, h2, Dir::CW);

        (*n_h1).next = h2;
        (*n_h2).next = h1;
        (*h2).prev = n_h1;
        (*h1).prev = n_h2;
        if (*f).outer_loop == l2 {
            (*f).outer_loop = l1;
        }
        delete(l2, (*l2).face);
    }
}

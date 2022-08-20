use crate::{
    euler::{add_halfedge, Dir},
    structs::{Edge, Face, HalfEdge, Loop},
};

pub fn lmef(h1: *mut HalfEdge, h2: *mut HalfEdge, face_id: usize) -> *mut Face {
    unsafe {
        let s = (*(*(*h1).lp).face).solid;
        let mut new_face = Face::new(face_id, s);
        let mut new_loop = Loop::new(new_face);
        let new_edge = Edge::new(s);
        (*new_face).outer_loop = new_loop;

        //Set all halfedges to be pointing to this new loop:
        let mut he = h1;
        while he != h2 {
            (*he).lp = new_loop;
            he = (*he).next;
        }
        let new_h1 = add_halfedge(new_edge, (*h2).vertex, h1, Dir::CW);
        let new_h2 = add_halfedge(new_edge, (*h1).vertex, h2, Dir::CCW);

        (*(*new_h1).prev).next = new_h2;
        (*(*new_h2).prev).next = new_h1;
        let temp = (*new_h1).prev;
        (*new_h1).prev = (*new_h2).prev;
        (*new_h2).prev = temp;
        (*new_loop).ledg = new_h1;
        (*(*h2).lp).ledg = new_h2;

        new_face
    }
}

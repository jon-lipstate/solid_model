use std::ptr;

use super::Dir;
use crate::structs::*;

pub fn add_halfedge(
    edge: *mut Edge,
    vertex: *mut Vertex,
    to: *mut HalfEdge,
    sign: Dir,
) -> *mut HalfEdge {
    unsafe {
        //if there is no edge on the destination, clone existing he
        // if there is an edge on `to`, insert the new he there
        let he: *mut HalfEdge;
        if (*to).edge.is_null() {
            he = to;
        } else {
            he = HalfEdge::new();
            HalfEdge::insert_before(he, to);
        }
        //Copy Edge & Loop to new HE
        (*he).edge = edge;
        (*he).lp = (*to).lp;
        //set the HE to the fn's vertex:
        (*he).vertex = vertex;

        match sign {
            Dir::CCW => {
                //PLUS / HE1
                (*edge).he_rt = he;
            }
            Dir::CW => {
                (*edge).he_lt = he;
            }
        }

        return he;
    }
}

pub fn delete_halfedge(he: &mut *mut HalfEdge) -> *mut HalfEdge {
    unsafe {
        if (*(*he)).edge.is_null() {
            delete(he);
            return ptr::null_mut();
        } else if (*(*he)).next == *he {
            (*(*he)).edge = ptr::null_mut();
            return *he;
        } else {
            (*(*(*he)).prev).next = (*(*he)).next;
            (*(*(*he)).next).prev = (*(*he)).prev;
            let rv = (*(*he)).prev;
            delete(he);
            return rv;
        }
    }
}

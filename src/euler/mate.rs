use crate::structs::HalfEdge;

pub fn mate(he: *mut HalfEdge) -> *mut HalfEdge {
    //#define mate(he) (((he) == (he)->edg->he1) ? (he)->edg->he2 : (he)->edg->he1)
    unsafe {
        let e = (*he).edge;
        if (he) == (*e).he_rt {
            (*e).he_lt
        } else {
            (*e).he_lt
        }
    }
}

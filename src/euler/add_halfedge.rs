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

pub fn delete_halfedge(he: *mut HalfEdge) -> *mut HalfEdge {
    unsafe {
        unimplemented!();
        // return (*he).prev;
    }
}
// HalfEdge        *delhe(he)
// HalfEdge        *he;
// {
//         if(he->edg == NIL)
//         {
//                 del(HALFEDGE, he, NIL);
//                 return(NIL);
//         }
//         else if(he->nxt == he)
//         {
//                 he->edg = NIL;
//                 return(he);
//         }
//         else
//         {
//                 he->prv->nxt = he->nxt;
//                 he->nxt->prv = he->prv;
//                 del(HALFEDGE, he, NIL);
//                 return(he->prv);
//         }
// }

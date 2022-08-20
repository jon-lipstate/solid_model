use crate::{
    euler::{add_halfedge, mate, Dir},
    structs::*,
};
pub fn lmev(h1: *mut HalfEdge, h2: *mut HalfEdge, vtx_id: usize, x: f32, y: f32, z: f32) {
    unsafe {
        //Ref to parent Solid:
        let s = (*(*(*h1).lp).face).solid;
        //new vertex:
        let mut vtx = Vertex::new(s, vtx_id, x, y, z);
        //new Edge:
        let edge = Edge::new(s);

        let mut he = h1;
        while he != h2 {
            (*he).vertex = vtx;
            he = (*mate(he)).next;
        }
        add_halfedge(edge, (*h2).vertex, h2, Dir::CCW); //plus
        add_halfedge(edge, vtx, h1, Dir::CW); //minus
        (*vtx).half_edge = (*h2).prev;
        (*(*h2).vertex).half_edge = h2;
    }
}

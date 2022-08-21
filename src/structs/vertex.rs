use super::{HalfEdge, Solid};
use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::{self},
};

#[derive(Debug, PartialEq)]
pub struct Vertex {
    /// (vertexno)
    pub id: usize,
    /// (vedge) ptr to halfedge
    pub half_edge: *mut HalfEdge,
    /// (vcoord) vtx coords
    pub coords: (f32, f32, f32, f32),
    /// (nextv)
    pub next: *mut Vertex,
    /// (prevv)
    pub prev: *mut Vertex,
}

//
impl Vertex {
    ///globally allocated - ONLY ALLOCATE VIA SOLID::NEW_VERTEX
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

            v
        }
    }
}

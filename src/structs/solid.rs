use super::{edge, Edge, Face, Vertex};
use std::ptr;

#[derive(Debug, PartialEq)]
pub struct Solid {
    ///solid identifier
    pub id: usize,
    ///ptr to list of faces    (sfaces)
    pub faces_start: *mut Face,
    ///ptr to list of edges    (sedges)
    pub edges_start: *mut Edge,
    ///ptr to list of vertices (sverts)
    pub vertices_start: *mut Vertex,
}
//
impl Solid {
    pub fn new(id: usize) -> Solid {
        Solid {
            id,
            faces_start: ptr::null_mut(),
            edges_start: ptr::null_mut(),
            vertices_start: ptr::null_mut(),
        }
    }
    pub fn find_face(&self, face_id: usize) -> Result<Option<*mut Face>, String> {
        if self.faces_start.is_null() {
            return Err(format!("[{}] Solid.faces_start nullptr", self.id));
        }
        let mut face = self.faces_start;
        loop {
            unsafe {
                if face.is_null() {
                    break;
                }
                if (*face).id == face_id {
                    return Ok(Some(face));
                }
                face = (*face).next;
            }
        }
        return Ok(None);
    }
    pub fn get_max_face_id(&self) -> usize {
        let mut max: usize = 1;
        let mut f = self.faces_start;
        while !f.is_null() {
            unsafe {
                if (*f).id > max {
                    max = (*f).id;
                }
                f = (*f).next;
            }
        }

        max
    }
    pub fn get_max_vtx_id(&self) -> usize {
        let mut max: usize = 1;
        let mut v = self.vertices_start;
        while !v.is_null() {
            unsafe {
                if (*v).id > max {
                    max = (*v).id;
                }
                v = (*v).next;
            }
        }

        max
    }
    ///Allocates new edge & registers it within the solid's edge-list
    pub fn new_edge(&mut self) -> *mut Edge {
        unsafe {
            let new_edge = Edge::new(self);
            (*new_edge).next = self.edges_start;
            (*new_edge).prev = ptr::null_mut();
            if !self.edges_start.is_null() {
                (*self.edges_start).prev = new_edge;
            }
            self.edges_start = new_edge;

            new_edge
        }
    }
    // pub fn add_edge(&mut self, edge: *mut Edge) {
    //     unsafe {
    //         (*edge).next = self.edges_start;
    //         (*edge).prev = ptr::null_mut();
    //         if !self.edges_start.is_null() {
    //             (*self.edges_start).prev = edge;
    //         }
    //         self.edges_start = edge;
    //     }
    // }
    ///Removes edge from Solid's Edge list, does not deallocate
    pub fn remove_edge(&mut self, edge: *mut Edge) {
        unsafe {
            if !(*edge).prev.is_null() {
                (*(*edge).prev).next = (*edge).next;
            }
            if !(*edge).next.is_null() {
                (*(*edge).next).prev = (*edge).prev;
            }
            if edge == self.edges_start {
                self.edges_start = (*edge).next;
            }
        }
    }
    ///removes face from solids facelist, does not deallocate
    pub fn remove_face(&mut self, face: *mut Face) {
        unsafe {
            if !(*face).prev.is_null() {
                (*(*face).prev).next = (*face).next;
            }
            if !(*face).next.is_null() {
                (*(*face).next).prev = (*face).prev;
            }
            if face == self.faces_start {
                self.faces_start = (*face).next;
            }
        }
    }
    ///allocates new face & registers to self.faces_start
    pub fn new_face(&mut self, face_id: usize) -> *mut Face {
        unsafe {
            let mut new_face = Face::new(face_id, self);
            (*new_face).next = self.faces_start;
            (*new_face).prev = ptr::null_mut();
            if !self.faces_start.is_null() {
                (*self.faces_start).prev = new_face;
            }
            self.faces_start = new_face;
            (*new_face).solid = self;

            new_face
        }
    }
    ///allocates new vertex & registers to self.faces_start
    pub fn new_vertex(&mut self, vtx_id: usize, x: f32, y: f32, z: f32) -> *mut Vertex {
        unsafe {
            let mut new_vertex = Vertex::new(self, vtx_id, x, y, z);
            (*new_vertex).next = self.vertices_start;
            (*new_vertex).prev = ptr::null_mut();
            if !self.vertices_start.is_null() {
                (*self.vertices_start).prev = new_vertex;
            }
            if !(*self).vertices_start.is_null() {
                (*(*self).vertices_start).prev = new_vertex;
            }
            (*self).vertices_start = new_vertex;

            new_vertex
        }
    }
    ///removes face from solids facelist, does not deallocate
    pub fn remove_vertex(&mut self, vertex: *mut Vertex) {
        unsafe {
            if !(*vertex).prev.is_null() {
                (*(*vertex).prev).next = (*vertex).next;
            }
            if !(*vertex).next.is_null() {
                (*(*vertex).next).prev = (*vertex).prev;
            }
            if vertex == self.vertices_start {
                self.vertices_start = (*vertex).next;
            }
        }
    }
}

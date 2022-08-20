use super::{Edge, Face, Vertex};
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
    // ///ptr to next solid
    // pub next: *mut Solid,
    // ///ptr to prev solid
    // pub prev: *mut Solid,
}
//
impl Solid {
    pub fn new(id: usize) -> Solid {
        Solid {
            id,
            faces_start: ptr::null_mut(),
            edges_start: ptr::null_mut(),
            vertices_start: ptr::null_mut(),
            // next: ptr::null_mut(),
            // prev: ptr::null_mut(),
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
}

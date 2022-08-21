use crate::structs::*;

/// Creates a Solid containing a vertex, face, loop & halfedge
pub fn mvfs(face_id: usize, vtx_id: usize, x: f32, y: f32, z: f32) -> Solid {
    let mut s: Solid = Solid::new(1);
    unsafe {
        let mut f = s.new_face(face_id);
        let mut he = HalfEdge::new();
        let mut lp = (*f).new_loop();
        let mut v = s.new_vertex(vtx_id, x, y, z);

        (*v).half_edge = he;
        (*f).outer_loop = lp;
        (*lp).half_edge = he;
        (*he).lp = lp;
        (*he).next = he;
        (*he).prev = he;
        (*he).vertex = v;
        s.vertices_start = v;
    }
    s
}

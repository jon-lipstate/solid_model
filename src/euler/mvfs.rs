use crate::structs::*;

/// Creates a Solid containing a vertex, face, loop & halfedge
pub fn mvfs(fid: usize, vid: usize, x: f32, y: f32, z: f32) -> Solid {
    let mut s: Solid = Solid::new(1);
    unsafe {
        let mut f = Face::new(fid, &mut s);
        let mut he = HalfEdge::new();
        let mut lp = Loop::new(f);
        // let mut lp: Loop = Loop::new(&mut f);
        let mut v = Vertex::new(&mut s, vid, x, y, z);

        (*v).half_edge = he;
        (*f).outer_loop = lp;
        // (f).outer_loop = &mut lp;
        (*lp).ledg = he;
        (*he).lp = lp;
        (*he).next = he;
        (*he).prev = he;
        (*he).vertex = v;
        s.faces_start = f;
        // s.faces_start = &mut f;
        s.vertices_start = v;
    }

    s
}

use super::{delete, HalfEdge, Loop, Solid};
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
    /// contvv (P220)
    pub fn is_coincident(v1: *mut Vertex, v2: *mut Vertex) -> bool {
        unsafe {
            let vc1 = (*v1).coords;
            let vc2 = (*v2).coords;
            let diff = (vc1.0 - vc2.0, vc1.1 - vc2.1, vc1.2 - vc2.2);
            let sq = diff.0 * diff.0 + diff.1 * diff.1 + diff.2 * diff.2;
            sq < std::f32::EPSILON
        }
    }
    /// intrev (P222)
    fn intrev(v1: *mut Vertex, v2: *mut Vertex, v3: *mut Vertex, t: &mut Box<f32>) -> bool {
        let mut r1 = (0., 0., 0.);
        let mut r2 = (0., 0., 0.);
        let r1r1: f32;
        let mut tvc = (0., 0., 0.);
        unsafe {
            let v1c = (*v1).coords;
            let v2c = (*v2).coords;
            let v3c = (*v3).coords;
            r1.0 = v2c.0 - v1c.0;
            r1.1 = v2c.1 - v1c.1;
            r1.2 = v2c.2 - v1c.2;
            r1r1 = r1.0 * r1.0 + r1.1 * r1.1 + r1.2 * r1.2; //dot to self
            if r1r1 < std::f32::EPSILON * std::f32::EPSILON {
                *t = Box::new(0.);
                return Vertex::is_coincident(v1, v3);
            } else {
                r2.0 = v3c.0 - v1c.0;
                r2.1 = v3c.1 - v1c.1;
                r2.2 = v3c.2 - v1c.2;
                let mut tprime = r1.0 * r2.0 + r1.1 * r2.1 + r1.2 * r2.2; //dot r1,r2
                tprime /= r1r1;
                tvc.0 = v1c.0 + tprime * r1.0;
                tvc.1 = v1c.1 + tprime * r1.1;
                tvc.2 = v1c.2 + tprime * r1.2;
                *t = Box::new(tprime);
                let mut test_vertex = Vertex::new(ptr::null_mut(), 0, tvc.0, tvc.2, tvc.2);
                let ic = Vertex::is_coincident(test_vertex, v3);
                delete::<Vertex>(&mut test_vertex);
                return ic;
            }
        }
    }
    ///contev (p222)
    pub fn vertex_intersects_edge(
        v1: *mut Vertex,
        v2: *mut Vertex,
        v3: *mut Vertex,
    ) -> Result<bool, String> {
        let mut t: Box<f32> = Box::new(0.);
        if Vertex::intrev(v1, v2, v3, &mut t) {
            println!("t was: {:?}", t);
            Ok(true)
        } else {
            println!("t was: {:?}", t);
            Err(format!("???"))
        }
    }
    ///P223: Boundary Cases
    pub fn bndrlv(
        vtx: *mut Vertex,
        lp: *mut Loop,
    ) -> Result<(Option<*mut Vertex>, Option<*mut HalfEdge>), String> {
        //These two variables are declared as globals refactor as tuple return...?
        let hit_vtx: *mut Vertex;
        let hit_he: *mut HalfEdge;
        unsafe {
            let mut he = (*lp).half_edge;
            loop {
                if Vertex::is_coincident((*he).vertex, vtx) {
                    hit_vtx = (*he).vertex;
                    return Ok((Some(hit_vtx), None));
                }
                he = (*he).next;
                if he == (*lp).half_edge {
                    break;
                }
            }
            he = (*lp).half_edge;
            loop {
                let vie = Vertex::vertex_intersects_edge((*he).vertex, (*(*he).next).vertex, vtx)?;
                if vie {
                    hit_he = he;
                    return Ok((None, Some(hit_he)));
                }
                he = (*he).next;
                if he == (*lp).half_edge {
                    break;
                }
            }
            Err(format!("?? didnt find"))
        }
    }

    ///P225; Vertex Loop Containment
    pub fn contlv(lp: *mut Loop, vtx: *mut Vertex, drop: i32) -> Result<usize, String> {
        unsafe {
            let intr = Vertex::bndrlv(vtx, lp)?;
            if intr.0.is_some() {
                return Ok(2); // from bndrlv
            } else if intr.1.is_some() {
                return Ok(3);
            }
            let mut h2 = (*lp).half_edge;
            //retry:
            let v1 = (*h2).vertex;
            let v2 = (*(*h2).next).vertex;
            let v1c = (*v1).coords;
            let v2c = (*v2).coords;

            let x = (v1c.0 + v2c.0) / 2.;
            let y = (v1c.1 + v2c.1) / 2.;
            let z = (v1c.2 + v2c.2) / 2.;
            let mut h1 = (*lp).half_edge;
            let mut count = 0;
            let mut t1: Box<f32> = Box::new(0.);
            let mut t2: Box<f32> = Box::new(0.);
            let mut c1;
            let mut c2;
            let mut tmp_vtx = Vertex::new(ptr::null_mut(), 0, x, y, z);
            loop {
                let intr = Vertex::line_intersection(vtx, tmp_vtx, v1, v2, drop, &mut t1, &mut t2)?;
                if intr == 1 {
                    c1 = crate::utils::compare(*t2, 0., std::f32::EPSILON);
                    c2 = crate::utils::compare(*t2, 1., std::f32::EPSILON);
                    if c1 == 0 || c2 == 0 {
                        h2 = (*h2).next;
                        if h2 == (*lp).half_edge {
                            delete::<Vertex>(&mut tmp_vtx);
                            return Err(format!("didnt find"));
                        }
                        // goto retry
                    }
                    if c1 == 1 && c2 == -1 {
                        if *t1 > 0. {
                            count += 1;
                        }
                    }
                }
                h1 = (*h1).next;
                if h1 == (*lp).half_edge {
                    break;
                }
            }
            count = count % 2;

            delete::<Vertex>(&mut tmp_vtx);

            Ok(count)
        }
    }

    ///Line Intersection - formula P226. Seems confusing compared to my recollection..?
    /// drop:0,1,2->x,y,z
    fn line_intersection(
        v1: *mut Vertex,
        v2: *mut Vertex,
        v3: *mut Vertex,
        v4: *mut Vertex,
        drop: i32,
        t1: &mut Box<f32>,
        t2: &mut Box<f32>,
    ) -> Result<i32, String> {
        unsafe {
            let v1c = (*v1).coords;
            let v2c = (*v2).coords;
            let v3c = (*v3).coords;
            let v4c = (*v4).coords;

            let a1: f32;
            let a2: f32;
            let b1: f32;
            let b2: f32;

            match drop {
                1 => {
                    a1 = v2c.1 - v1c.1;
                    a2 = v2c.2 - v1c.2;
                    b1 = v3c.1 - v4c.1;
                    b2 = v3c.2 - v4c.2;
                }
                2 => {
                    a1 = v2c.0 - v1c.0;
                    a2 = v2c.2 - v1c.2;
                    b1 = v3c.0 - v4c.0;
                    b2 = v3c.2 - v4c.2;
                }
                3 => {
                    a1 = v2c.0 - v1c.0;
                    a2 = v2c.1 - v1c.1;
                    b1 = v3c.0 - v4c.0;
                    b2 = v3c.1 - v4c.1;
                }
                _ => return Err(format!("invalid Drop, 0,1,2 only for x,y,z")),
            };
            let d = a1 * b1 - a2 * b2;
            if crate::utils::compare(d, 0., f32::EPSILON) == 0 {
                return Ok(0);
            } else {
                let c1: f32;
                let c2: f32;
                match drop {
                    1 => {
                        c1 = v1c.1 - v3c.1;
                        c2 = v1c.2 - v3c.2;
                    }
                    2 => {
                        c1 = v1c.0 - v3c.0;
                        c2 = v1c.2 - v3c.2;
                    }
                    3 => {
                        c1 = v1c.0 - v3c.0;
                        c2 = v1c.1 - v3c.1;
                    }
                    _ => return Err(format!("invalid Drop, 0,1,2 only for x,y,z")),
                };
                let d1 = c2 * b1 - c1 * b2;
                let d2 = a2 * c1 - a1 * c2;
                *t1 = Box::new(d1 / d);
                *t2 = Box::new(d2 / d);

                return Ok(1);
            }
        }
    }
}

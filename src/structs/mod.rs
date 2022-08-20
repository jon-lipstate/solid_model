use std::{cell::RefCell, rc::Rc};
pub type Link<T> = Option<Rc<RefCell<T>>>;
//
mod solid;
pub use solid::Solid;
//
mod face;
pub use face::Face;
//
mod _loop;
pub use _loop::Loop;
//
mod edge;
pub use edge::Edge;
//
mod half_edge;
pub use half_edge::HalfEdge;
//
mod vertex;
pub use vertex::Vertex;
//
mod nodes;
pub use nodes::Nodes;
//

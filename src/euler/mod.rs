mod cuboid;
pub use cuboid::cuboid;
mod mvfs;
pub use mvfs::mvfs;
mod mate;
pub use mate::mate;
mod low_level;
use low_level::*;

mod add_halfedge;
pub use add_halfedge::{add_halfedge, delete_halfedge};

mod mev;
pub use mev::mev;

mod mef;
pub use mef::{kef, mef};

mod sweep;
pub use sweep::sweep;

pub enum Dir {
    CCW,
    CW,
}
// Node *new (int what, Node *where)
// {
//     Node *node;
//     char *malloc();
//     node = (Node *)malloc(nodesize[what]);
//     switch (what)
//     {
//     case SOLID:
//         addlist(SOLID, node, NIL);
//         node->s.sfaces = (Face *)NIL;
//         node->s.sedges = (Edge *)NIL;
//         node->s.sverts = (Vertex *)NIL;
//         break;
//     case FACE:
//         addlist(FACE, node, where);
//         node->f.floops = (Loop *)NIL;
//         node->f.flout = (Loop *)NIL;
//         break;
//     case LOOP:
//         addlist(LOOP, node, where);
//         break;
//     case EDGE:
//         addlist(EDGE, node, where);
//         break;
//     case VERTEX:
//         addlist(VERTEX, node, where);
//         node->v.vedge = (Vertex *)NIL;
//         break;
//     default:
//         break;
//     }
//     return (node);
// }

// void addlist(int what, Node *which, Node *where)
// {
//     switch (what)
//     {
//     case SOLID:
//         which->s.nexts = firsts;
//         which->s.prevs = (Solid *)NIL;
//         if (firsts)
//             firsts->prevs = (Solid *)which;
//         firsts = (Solid *)which;
//         break;
//     case FACE:
//         which->f.nextf = where->s.sfaces;
//         which->f.prevf = (Face *)NIL;
//         if (where->s.sfaces)
//             where->s.sfaces->prevf = (Face *)which;
//         where->s.sfaces = (Face *)which;
//         where->f.fsolid = (Solid *)where;
//         break;
//     case LOOP:
//         which->l.nextl = where->f.floops;
//         which->l.prevl = (Loop *)NIL;
//         if (where->f.floops)
//             where->f.floops->prevl = (Loop *)which;
//         where->f.floops = (Loop *)which;
//         where->l.lface = (Face *)where;
//         break;
//     case EDGE:
//         which->e.nexte = where->s.sedges;
//         which->e.preve = (Edge *)NIL;
//         if (where->s.sedges)
//             where->s.sedges = (Edge *)which;
//         break;
//     case VERTEX:
//         which->v.nextv = where->s.sverts;
//         which->v.prevv = (Vertex *)NIL;
//         if (where->s.sverts)
//             where->s.sverts->prevv = (Vertex *)which;
//         where->s.sverts = (Vertex *)which;
//         break;
//     default:
//         break;
//     }
// }

#![allow(dead_code, unused_imports)]
use std::ptr;
mod euler;
mod structs;
pub use euler::{mef, mev, mvfs};
pub use structs::*;

use crate::euler::sweep;

fn main() -> Result<(), String> {
    let s = mvfs(1, 1, 0., 0., 0.);
    println!("{:?}", s);
    mev(&s, 1, 1, 2, 1., 0., 0.)?;

    mev(&s, 1, 2, 3, 1., 1., 0.)?;
    mev(&s, 1, 3, 4, 0., 1., 0.)?;
    mef(&s, 1, 4, 1, 2)?;
    let f = s.find_face(1)?.unwrap();
    sweep(f, 0.0, 0.0, 1.);

    println!("done");

    Ok(())
}

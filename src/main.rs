#![allow(dead_code, unused_imports)]
mod euler;
mod structs;
// use std::ptr;
// pub use euler::{mef, mev, mvfs};
// pub use structs::*;

use crate::euler::cuboid;

fn main() -> Result<(), String> {
    let c = cuboid(3., 4., 5.)?;
    println!("{:?}", c);

    Ok(())
}

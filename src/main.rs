#![allow(dead_code, unused_imports)]
mod compound;
mod euler;
mod structs;
mod utils;
pub use utils::*;
// use std::ptr;
// pub use euler::{mef, mev, mvfs};
// pub use structs::*;

use crate::compound::{block, cyl};

fn main() -> Result<(), String> {
    let c = block(3., 4., 5.)?;
    println!("{:?}", c);
    //not working lmef h2 is nullptr for some reason during sweep(): ln33
    let c = cyl(1, 1., 1., 16)?;
    println!("{:?}", c);

    Ok(())
}

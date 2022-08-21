use std::{
    alloc::{dealloc, Layout},
    ptr,
};

use crate::structs::Solid;
pub fn delete<T>(item: &mut *mut T, parent: *mut Solid) {
    println!("todo: cleanup logic (removing self from linked-lists etc");
    unsafe {
        dealloc(*item as *mut u8, Layout::new::<T>());
        *item = ptr::null_mut();
    }
}

///TEMP FILE - TEST IDEAS
use std::{
    alloc::{alloc, dealloc, Layout},
    ptr,
};
#[derive(Debug)]
pub struct Foo {
    a: u32,
}
impl Foo {
    pub fn new() -> *mut Foo {
        unsafe {
            let f = alloc(Layout::new::<Foo>()) as *mut Foo;
            let fv = Foo { a: 3 };
            f.write(fv);
            f
        }
    }

    pub fn delete(item: &mut *mut Foo) {
        unsafe {
            dealloc(*item as *mut u8, Layout::new::<Foo>());
            *item = ptr::null_mut();
        }
    }
}

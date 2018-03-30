use std::mem;
use std::ptr;

const SIZE: usize = 10;

fn main() {
    let mut x : [Box<u32>; SIZE];
    unsafe {
        x = mem::uninitialized();
        for i in 0..SIZE {
            ptr::write(&mut x[i], Box::new(i as u32));
        }
    }
}

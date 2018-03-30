use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/*
 * A helper struct to make Variance of types possible
 * for Vector types.
 * Also by making use of PhantomData helps rust to figure
 * out about the ownership of the data.
 */
struct Unique<T: Debug>
{
    /// *const T is variant over T
    /// *mut T is invariant over T
    ptr : *const T,
    _marker : PhantomData<T>,
}

impl <T> Unique<T>
    where T : Debug
{
    ///
    fn new(ptr : *mut T) -> Self {
        Unique{
            ptr: ptr,
            _marker: PhantomData,
        }
    }
    ///
    fn as_ptr(&self) -> *mut T
    {
        self.ptr as *mut T
    }
    ///
    fn empty() -> Self
    {
        // This is a dangling pointer but still
        // pointing to aligned data.
        // Use of null is thus avoided which could have
        // otherwise made rust do bad things.(Like what ?)
        let p = std::mem::align_of::<T>() as *mut T;
        Unique{
            ptr : p,
            _marker : PhantomData
        }
    }
    ///
    fn offset(&self, ix: isize) -> *mut T
    {
        unsafe {
            self.as_ptr().offset(ix)
        }
    }
}

pub struct IntoIter<T : Debug>
{
    buf : Unique<T>,
    cap : usize,
    start : *const T,
    end   : *const T,
}

impl<T> Iterator for IntoIter<T> where T : Debug
{
    type Item = T;
    ///
    fn next(&mut self) -> Option<T>
    {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T>
    where T : Debug
{
    ///
    fn next_back(&mut self) -> Option<T>
    {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

pub struct Vector<T : Debug>
{
    /// pointer to the array
    ptr : Unique<T>,
    /// Capacity of the vector
    cap : usize,
    /// Number of elements in the vector
    len : usize,
}

impl<T> Vector<T>
    where T : Debug
{
    ///
    pub fn new() -> Vector<T>
    {
        assert!(mem::size_of::<T>() != 0, "Not ready for ZSTs");
        Vector{
            ptr : Unique::empty(),
            cap : 0,
            len : 0,
        }
    }
    ///
    pub fn size(&self) -> usize
    {
        self.len
    }
    ///
    pub fn capacity(&self) -> usize
    {
        self.cap
    }
    ///
    pub fn push(&mut self, elem: T)
    {
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            ptr::write(self.ptr.offset(self.len as isize), elem);
        }
        self.len += 1;
    }
    ///
    pub fn pop(&mut self) -> Option<T>
    {
        if self.len == 0{
            return None;
        }
        self.len -= 1;
        unsafe {
            return Some(ptr::read(self.ptr.offset(self.len as isize)));
        }
    }
    ///
    pub fn insert(&mut self, at: usize, elem: T)
    {
        assert!(at <= self.len, "index out of bounds");
        if self.cap == self.len {
            self.grow();
        }

        //copy the contents from `at` to the end
        unsafe {
            if at < self.len {
                ptr::copy(
                    self.ptr.offset(at as isize), //<< The source memory location
                    self.ptr.offset((at + 1) as isize), //<< The destination memory location
                    self.len - at // << Number of elements to be copied
                );
            }
            //Write the lement at the location
            ptr::write(self.ptr.offset(at as isize), elem);
            self.len += 1;
        }
    }
    ///
    pub fn remove(&mut self, from: usize) -> T
    {
        assert!(from < self.len, "index out of bounds");
        unsafe {
            let result = ptr::read(self.ptr.offset(from as isize));
            self.len -= 1;

            ptr::copy(
                self.ptr.offset((from + 1) as isize),
                self.ptr.offset(from as isize),
                self.len - from
            );

            return result;
        }
    }
    ///
    pub fn into_iter(self) -> IntoIter<T>
    {
        let ret;
        unsafe {
            ret = IntoIter{
                buf: Unique::new(self.ptr.as_ptr()),
                cap: self.cap,
                start: self.ptr.as_ptr(),
                end: if self.cap == 0{
                    self.ptr.as_ptr()
                } else {
                    self.ptr.offset(self.len as isize)
                }
            };
        }
        //Do not call drop for this vector
        mem::forget(self);
        return ret;
    }
    ///
    fn allocate(count : usize) -> *mut T
    {
        let mut v: Vec<T> = Vec::with_capacity(count);
        let ptr = v.as_mut_ptr();
        mem::forget(v);
        return ptr;
    }
    ///
    fn deallocate(ptr: *mut T, len: usize, cap: usize)
    {
        unsafe {
            mem::drop(Vec::from_raw_parts(ptr, len, cap));
        }
    }
    ///
    fn reallocate(old_ptr: *mut T, old_size: usize, new_size: usize) -> *mut T
    {
        unsafe {
            // allocate new block of memory
            let new_ptr = Vector::allocate(new_size);
            //copy from old memory to new memory
            for i in 0..old_size {
                ptr::write(new_ptr.offset(i as isize), old_ptr.offset(i as isize).read());
            }
            return new_ptr as *mut T;
        }
    }
    ///
    fn grow(&mut self)
    {
        let elem_size = mem::size_of::<T>();

        let (new_cap, ptr) = if self.cap == 0 {
            let ptr = Vector::allocate(elem_size);
            (1, ptr)
        } else {
            // Double the memory
            let new_cap = self.cap * 2;
            let old_num_bytes = self.len * elem_size;

            assert!(old_num_bytes <= ::std::isize::MAX as usize / 2,
                    "capacity overflow!");

            let new_num_bytes = old_num_bytes * 2;
            let ptr = Vector::reallocate(self.ptr.as_ptr() as *mut _,
                                         old_num_bytes,
                                         new_num_bytes);
            (new_cap, ptr)
        };

        self.ptr = Unique::new(ptr as *mut _);
        self.cap = new_cap;
    }
}

impl<T> Drop for Vector<T> where T : Debug {
    ///
    fn drop(&mut self)
    {
        // No capacity, no deallocation
        if self.cap == 0 {
            return ();
        }
        Vector::deallocate(self.ptr.as_ptr() as *mut T, self.len, self.cap);
    }
}

impl<T> Deref for Vector<T> where T : Debug {
    type Target = [T];
    ///
    fn deref(&self) -> &[T]
    {
        unsafe {
            ::std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T> DerefMut for Vector<T> where T : Debug {
    ///
    fn deref_mut(&mut self) -> &mut [T]
    {
        unsafe {
            ::std::slice::from_raw_parts_mut(self.as_ptr() as *mut T, self.len)
        }
    }
}

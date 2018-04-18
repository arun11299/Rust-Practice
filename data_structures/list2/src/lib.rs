use std::{ptr, mem};

type Link<T> = Option<Box<Node<T>>>;

struct Raw<T>
{
    ptr: *const Node<T>,
}

struct Node<T>
{
    elem: T,
    prev: Raw<T>,
    next: Link<T>,
}

// Implementation for `Node`
impl<T> Node<T> {
    // Makes a node with the givin element
    fn new(elem : T) -> Self {
        Node{
            elem : elem,
            prev : Raw::none(),
            next : None,
        }
    }

    // Join two lists
    fn link(&mut self, mut next: Box<Self>) {
        next.prev = Raw::some(self);
        self.next = Some(next);
    }

    // Make this node come after this one
    fn splice_next(&mut self, mut next : Box<Self>) {
        let mut old_next = self.next.take();
        old_next.as_mut().map(|node| node.prev = Raw::some(&mut *next));
        next.next = old_next;
        next.prev = Raw::some(self);
        self.next = Some(next)
    }

    // Takes the next node from this one, breaking the list into two
    fn take_next(&mut self) -> Option<Box<Self>> {
        let mut next = self.next.take();
        next.as_mut().map(|node| node.prev = Raw::none());
        next
    }
}

// Implementation for `Raw`
impl<T> Raw<T> {
    // Makes a null reference
    fn none() -> Self {
        Raw { ptr : ptr::null_mut() }
    }

    // Makes a reference to the given node
    fn some(ptr: &mut Node<T>) -> Self {
        Raw { ptr : ptr }
    }

    // Converts the reference to an Option containing the reference
    fn as_ref(&self) -> Option<&Node<T>> {
        unsafe {
            if self.ptr.is_null() {
                None
            } else {
                Some(&*self.ptr)
            }
        }
    }

    // Converts the reference to an Option contaning mutable reference
    fn as_mut(&mut self) -> Option<&mut Node<T>>
    {
        unsafe {
            if self.ptr.is_null() {
                None
            } else {
                Some(&mut *(self.ptr as *mut Node<T>))
            }
        }
    }

    // Takes the reference out and nulls out this one.
    fn take(&mut self) -> Self {
        mem::replace(self, Raw::none())
    }

    // Clones the reference
    fn clone(&mut self) -> Self {
        Raw {ptr : self.ptr}
    }
}

pub struct LinkedList<T>
{
    len : usize,
    head: Link<T>,
    tail: Raw<T>,
}

impl<T> LinkedList<T> {
    // Returns an empty linked list
    pub fn new() -> Self {
        LinkedList {
            len  : 0,
            head : None,
            tail : Raw::none(),
        }
    }

    // Appends the given element to the back of the linked list
    pub fn push_back(&mut self, elem : T) {
        self.len += 1;
        let mut new_node = Box::new(Node::new(elem));
        let mut old_tail = mem::replace(&mut self.tail, Raw::some(&mut *new_node));
        match old_tail.as_mut() {
            None => self.head = Some(new_node),
            Some(tail) => tail.link(new_node),
        }
    }

    // Appends the given element to the front of the list
    pub fn push_front(&mut self, elem : T) {
        self.len += 1;
        let mut new_node = Box::new(Node::new(elem));

        match self.head.take() {
            None => self.tail = Raw::some(&mut *new_node),
            Some(head) => new_node.link(head),
        }
        self.head = Some(new_node);
    }

    // Removes the element from the back of the list and returns it
    pub fn pop_back(&mut self) -> Option<T> {
        let mut old_tail = self.tail.take();
        old_tail.as_mut().and_then(
            |tail| {
                self.len -= 1;
                match tail.prev.take().as_mut() {
                    None => self.head.take().map(|node| node.elem),
                    Some(prev) => {
                        self.tail = Raw::some(prev);
                        prev.next.take().map(|node| node.elem)
                    }
                }
            }
        )// end and_then
    }

    // Removes the element from the front of the list and returns it
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(
            |mut head| {
                self.len -= 1;
                match head.take_next() {
                    None => self.tail = Raw::none(),
                    Some(next) => self.head = Some(next),
                }
                head.elem
            }
        )
    }

    // Returns a reference to the element in the front of the list
    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    // Returns a mutable reference to the first element in the list
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    // Returns a reference to the last element of the list
    pub fn back(&self) -> Option<&T> {
        self.tail.as_ref().map(|node| &node.elem)
    }

    // Returns a mutable reference to the last element in the list
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail.as_mut().map(|node| &mut node.elem)
    }
}


/// A cursor over `LinkedList`.
///
/// A `Cursor` is like an iterator, except that it can freely seek back n forth
/// and can safely mutate the list during iteration. This is because the lifetime of
/// the yielded references is tied to its own lifetime, instead of the underlying list.
///
/// Cursors always rest between two elements in the list, and index in a logical circular
/// way. To accomodate this there is a "ghost" non-element that yields None between the
/// head and the tail of the list.
///
/// When created, the cursor starts between the ghost and the front of the list. That is,
/// `next` will yield front of the list, and `prev` will yield `None`. Calling `prev` again
/// will yield the tail.
pub struct Cursor<'a, T: 'a>
{
    list: &'a mut LinkedList<T>,
    prev: Raw<T>,
    index: usize,
}

impl<'a, T> Cursor<'a, T> {
    /// Resets the cursor to lie between the first and the last element
    pub fn reset(&mut self) {
        self.prev = Raw::none();
        self.index = 0;
    }

    /// Gets the next element in the list
    pub fn next(&mut self) -> Option<&mut T> {
        self.index += 1;
        match self.prev.take().as_mut() {
            // We had no previous elements, the cursor was sitting at the start
            // Next element should be the head of the list.
            None => {
                match self.list.head {
                    None => {
                        self.index = 0;
                        None
                    }
                    Some(ref mut head) => {
                        self.prev = Raw::some(&mut **head);
                        Some(&mut head.elem)
                    }
                }
            }
            // We had some previous elements, lets go to its next
            Some(ref mut prev) => {
                match prev.next {
                    Some(ref mut next) => {
                        self.prev = Raw::some(&mut **next);
                        unsafe {
                            //Dont kno0w what this is
                            Some(mem::transmute(&mut next.elem))
                        }
                    }
                    None => {
                        self.index = 0;
                        self.prev = Raw::none();
                        None
                    }
                }
            }
        }
    }

    // Gets the previous element in the list
    pub fn prev(&mut self) -> Option<&mut T> {
        match self.prev.take().as_mut() {
            Some(ref mut prev) => {
                self.index -= 1;
                self.prev = prev.prev.clone();
                unsafe {
                    //upgrade the lifetime, because its coming
                    //from the temporary value self.prev.take
                    Some(mem::transmute(&mut prev.elem))
                }
            }
            None => {
                self.prev = self.list.tail.clone();
                self.index = self.list.len;
                None
            }
        }
    }

    //Calls `next` specified number of times
    pub fn seek_forward(&mut self, by : usize) {
        for _ in 0..by {
            self.next();
        }
    }

    //Calls `prev` specified number of times
    pub fn seek_backward(&mut self, by : usize) {
        for _ in 0..by {
            self.prev();
        }
    }

    // Removes the next element in the list, without moving the cursor
    // Return None if the list is empty or if `next` is a ghost element
    pub fn remove(&mut self) -> Option<T> {
        let Cursor{ ref mut list, ref mut prev, .. } = *self;
        match prev.as_mut() {
            //No prev, we are at the start of the list
            None => list.pop_front(),
            Some(prev) => {
                match prev.take_next() {
                    None => None,
                    Some(mut next) => {
                        list.len -= 1;
                        match next.next.take() {
                            None => list.tail = Raw::some(prev);
                            Some(next_next) => prev.link(next_next),
                        }
                        Some(next.elem)
                    }
                }
            }
        }
        None
    }
}

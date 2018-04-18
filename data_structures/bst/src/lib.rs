use std::{ptr, mem};
use std::cmp::PartialOrd;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>
{
    elem  : T,
    left  : Link<T>,
    right : Link<T>,
}

struct Raw<T>
{
    ptr : *const Node<T>,
}

impl<T> Raw<T> {
    pub fn new() -> Self {
        Raw {
            ptr : ptr::null_mut(),
        }
    }

    pub fn some(ptr : &mut Node<T>) -> Self {
        Raw {
            ptr : ptr,
        }
    }

    pub fn none() -> Self {
        Raw { ptr : ptr::null_mut() }
    }

    pub fn as_ref(&self) -> Option<&Node<T>> {
        unsafe {
            if  self.ptr.is_null() {
                None
            } else {
                Some(&*self.ptr)
            }
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe {
            if self.ptr.is_null() {
                None
            } else {
                Some (&mut *(self.ptr as *mut _))
            }
        }
    }

    pub fn take(&mut self) -> Self {
        mem::replace(self, Raw::none())
    }
}

impl<T> Node<T> {
    /// Create a new Node with value.
    /// Both left and right links are set to None
    #[inline]
    pub fn new(elem : T) -> Self {
        Node {
            elem : elem,
            left : None,
            right : None,
        }
    }

    pub fn print<F : Fn(&T)>(&self, cb : &F) {
        match self.left {
            None => {},
            Some(ref lnode) => {
                lnode.print(cb);
            }
        }
        cb(&self.elem);

        match self.right {
            None => {},
            Some(ref rnode) => {
                rnode.print(cb);
            }
        }
    }

    /// Get the const value ref of this node.
    #[inline]
    pub fn value(&self) -> &T {
        &self.elem
    }

    /// Get the mutable value ref of this node.
    #[inline]
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.elem
    }

    /// Get reference to the left value inside an Option.
    /// Returns None if left is empty.
    #[inline]
    pub fn value_left(&self) -> Option<&T> {
        self.left.as_ref().map(|node| &node.elem)
    }

    /// Returns mutable reference to the left value inside an Option.
    /// Returns None if left is empty.
    #[inline]
    pub fn value_left_mut(&mut self) -> Option<&mut T> {
        self.left.as_mut().map(|node| &mut node.elem)
    }

    /// Get reference to the right value inside an Option.
    /// Returns None if right is empty.
    #[inline]
    pub fn value_right(&self) -> Option<&T> {
        self.right.as_ref().map(|node| &node.elem)
    }

    /// Returns mutable reference to the right value inside an Option.
    /// Returns None if right is empty.
    #[inline]
    pub fn value_right_mut(&mut self) -> Option<&mut T> {
        self.right.as_mut().map(|node| &mut node.elem)
    }

    /// Get the value in the left node.
    /// Original value replaced by None.
    /// Basically splits the BST from the left.
    #[inline]
    pub fn take_left(&mut self) -> Option<Box<Node<T>>> {
        mem::replace(&mut self.left, None)
    }

    /// Get the value in the right node.
    /// Original value replaced by None.
    /// Basically splits the BST from the right.
    #[inline]
    pub fn take_right(&mut self) -> Option<Box<Node<T>>> {
        mem::replace(&mut self.right, None)
    }

    /// Add node to the left of this node.
    /// Private function. Assumes some preconditions.
    #[inline]
    fn add_final_left(&mut self, elem : T) {
        assert! (self.left.is_none());
        self.left = Some(Box::new(Node::new(elem)));
    }

    /// Add node to the right of this node.
    /// Private function. Assumes some preconditions.
    #[inline]
    fn add_final_right(&mut self, elem : T) {
        assert! (self.right.is_none());
        self.right = Some(Box::new(Node::new(elem)));
    }

    #[inline]
    pub fn as_raw(&mut self) -> Raw<T> {
        Raw {
            ptr : self,
        }
    }
}

pub struct BST<T : PartialOrd>
{
    len : usize,
    head : Link<T>,
}

impl<T> BST<T> where T : PartialOrd {
    /// Create new BST
    pub fn new() -> Self {
        BST {
            len : 0,
            head : None,
        }
    }

    /// Number of elements in BST
    pub fn len(&self) -> usize {
        self.len
    }

    /// Add a new node to BST
    pub fn add(&mut self, elem : T) {
        match self.head {
            None => {
                let node = Box::new(Node::new(elem));
                self.head = Some(node);
            }
            Some(ref mut boxed_node) => {
                let mut rnode = boxed_node.as_raw();
                BST::add_node_int(rnode, elem)
            }
        }

        // Update the number of items in BST
        self.len += 1;
    }

    // Recursive inorder traversal
    pub fn print<F : Fn(&T)>(&self, cb : F) {
        match self.head {
            None => {
            },
            Some(ref node) => {
                node.print(&cb);
            }
        }
    }

    /// Internal add node impl.
    fn add_node_int(mut raw_node : Raw<T>, elem : T) {
        loop{
            match raw_node.take().as_mut() {
                None => { break; },
                Some(ref mut node) => {
                    if elem >= *node.value() {
                        match node.right {
                            None => {
                                node.add_final_right(elem);
                                break;
                            },
                            Some(ref mut rnode) => {
                                raw_node = Raw::some(&mut *rnode);
                            }
                        }
                    } else {
                        match node.left {
                            None => {
                                node.add_final_left(elem);
                                break;
                            },
                            Some(ref mut lnode) => {
                                raw_node = Raw::some(&mut *lnode);
                            }
                        }
                    }
                }
            }
        }
    }
}

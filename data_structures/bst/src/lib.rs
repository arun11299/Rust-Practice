use std::{ptr, mem};
use std::cmp::PartialOrd;

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T>
{
    elem  : T,
    left  : Link<T>,
    right : Link<T>,
}

pub struct Raw<T>
{
    ptr : *const Node<T>,
}

impl<T> Raw<T> {
    #[inline]
    pub fn new() -> Self {
        Raw {
            ptr : ptr::null_mut(),
        }
    }

    #[inline]
    pub fn some(ptr : &mut Node<T>) -> Self {
        Raw {
            ptr : ptr,
        }
    }

    #[inline]
    pub fn none() -> Self {
        Raw { 
            ptr : ptr::null_mut()
        }
    }

    #[inline]
    pub fn as_ref(&self) -> Option<&Node<T>> {
        unsafe {
            if  self.ptr.is_null() {
                None
            } else {
                Some(&*self.ptr)
            }
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe {
            if self.ptr.is_null() {
                None
            } else {
                Some (&mut *(self.ptr as *mut _))
            }
        }
    }

    #[inline]
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

    /// Returns `true` if the node is leaf.
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    /// Print the tree in inorder traversal
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
    pub len : usize,
    pub head : Link<T>,
}

impl<T> BST<T> where T : PartialOrd {
    /// Create new BST
    #[inline]
    pub fn new() -> Self {
        BST {
            len : 0,
            head : None,
        }
    }

    /// Number of elements in BST
    #[inline]
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

    /// Remove a node from the BST
    pub fn remove(&mut self, elem : T) {
        // Update the number of items in BST
        // TODO: Only if the element is found
        self.len -= 1;
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

    // Move `the_node` to the leftmost node of `of_node`
    pub fn move_to_leftmost(mut of_node : Raw<T>, the_node : Box<Node<T>>) {
        let mut tmp_node = of_node.as_mut().take();
        loop {
            if let Some(mut lnode) = tmp_node {
                match lnode.left {
                    None => { 
                        lnode.left = Some(the_node); 
                        break;
                    },
                    Some(ref mut boxed_node) => {
                        tmp_node = Some(&mut *boxed_node);
                    }
                }
            } else {
                break;
            }
        } // end loop
    }

    // Move `the_node` to the rightmost node of `of_node`
    fn move_to_rightmost(mut of_node : Raw<T>, the_node : Box<Node<T>>) {
        let mut tmp_node = of_node.as_mut().take();
        loop {
            if let Some(mut rnode) = tmp_node {
                match rnode.right {
                    None => {
                        rnode.right = Some(the_node);
                        break
                    },
                    Some(ref mut boxed_node) => {
                        tmp_node = Some(&mut *boxed_node);
                    }
                }
            } else {
                break;
            }
        }
    }

    /// Internal remove node impl.
    fn remove_node_int(mut cur_node : Raw<T>, mut prev_node : Raw<T>, elem : T) {
        /// This should not be called on the head of the tree.
        /// So prev_node should never be null.
        loop {
           match cur_node.take().as_mut() {
               None => { break; },
               Some(mut node) => {
                   if node.elem == elem {
                       if node.is_leaf() {
                           // Check which leaf to remove from the previous
                           prev_node.as_mut().map(|int_node| {
                               if int_node.elem > elem {
                                   int_node.left.take();
                               } else {
                                   int_node.right.take();
                               }
                           });
                           break;
                       } else {
                           prev_node.as_mut().map(|int_node| {
                               // Check where does this node lie wrt the previous node
                               if int_node.elem > elem {
                                   // this node is on the left side.
                                   if node.left.is_none() {
                                       int_node.left.take();
                                       int_node.left = node.right.take();
                                   }
                                   else if (node.right.is_none()) {
                                       int_node.left.take();
                                       int_node.left = node.left.take();
                                   } else {
                                       // make the right node take this nodes place
                                       let mut rrnode = node.right.take().map(|mut node| node.as_raw());
                                       //iterate through to the left most node of this node
                                       let mut tmp = Raw::some(node);
                                       loop {
                                           match tmp.take().as_mut() {
                                               None => { break; },
                                               Some(ref mut n) => {
                                                   if n.is_leaf() {
                                                       n.left = node.left.take();
                                                       break;
                                                   }
                                                   match node.left {
                                                       None => { break; },
                                                       Some(ref mut n) => {
                                                           tmp = n.as_raw();
                                                       }
                                                   }
                                               }
                                           }
                                       }
                                       // prev_node.left = rrnode
                                       unsafe {
                                        match rrnode {
                                            None => {},
                                            Some(ref mut r) => {
                                                int_node.left = Some(Box::from_raw(r.ptr as *mut _));
                                            }
                                        }
                                       }
                                   }
                               } else {
                                   // this node is on the right side
                                   if node.left.is_none() {
                                       int_node.right.take();
                                       int_node.right = node.right.take();
                                   } else if (node.right.is_none()) {
                                       int_node.right.take();
                                       int_node.right = node.left.take();
                                   } else {
                                       // make the right node take this nodes place
                                       let mut rrnode = node.right.take().map(|mut node| node.as_raw());
                                       //iterate through to the left most of this node
                                       let mut tmp = Raw::some(node);
                                       loop {
                                           match tmp.take().as_mut() {
                                               None => { break; },
                                               Some(ref mut n) => {
                                                   if n.is_leaf() {
                                                       n.left = node.left.take();
                                                       break;
                                                   }
                                                   match node.left {
                                                       None => { break; },
                                                       Some(ref mut n) => {
                                                           tmp = n.as_raw();
                                                       }
                                                   }
                                               }
                                           }
                                       }
                                       // prev_node.right = rrnode
                                       unsafe {
                                        match rrnode {
                                            None => {},
                                            Some(ref mut r) => {
                                                int_node.right = Some(Box::from_raw(r.ptr as *mut _));
                                            }
                                        }
                                       }
                                   }
                               }
                           });
                       }
                       break;
                   }
               }
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

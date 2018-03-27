use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node
{
    data: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<[Option<Rc<Node>>; 2]>,
}

#[derive(Debug)]
struct BinarySearchTree
{
    root: RefCell<Rc<Node>>,
}

impl BinarySearchTree {
    fn new(data: i32) -> BinarySearchTree
    {
        BinarySearchTree{
            root: RefCell::new(Rc::new(Node::new(data))),
        }
    }

    fn root(&self) -> Rc<Node>
    {
        Rc::clone(&self.root.borrow())
    }

    fn is_present(&self, value: i32) -> bool
    {
        self.root.borrow().find(value)
    }
}

impl Node {
    fn new(data: i32) -> Node
    {
        Node{
            data,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new([None, None]),
        }
    }

    fn set_left(&self, node: Rc<Node>)
    {
        self.children.borrow_mut()[0] = Some(Rc::clone(&node));
    }

    fn set_right(&self, node: Rc<Node>)
    {
        self.children.borrow_mut()[1] = Some(Rc::clone(&node));
    }

    fn add_child(&self, data: i32)
    {
        if data > self.data {
            if let Some(ref item) = self.children.borrow()[1] {
                return item.add_child(data);
            }
            self.set_right(Rc::new(Node::new(data)));
        } else {
            if let Some(ref item) = self.children.borrow()[0] {
                return item.add_child(data);
            }
            self.set_left(Rc::new(Node::new(data)));
        }
    }

    fn find(&self, value: i32) -> bool
    {
        if value == self.data {
            return true;
        }
        if value > self.data {
            if let Some(ref item) = self.children.borrow()[1] {
                return item.find(value);
            } else {
                return false;
            }
        } else {
            if let Some(ref item) = self.children.borrow()[0] {
                return item.find(value);
            } else {
                return false;
            }
        }
    }
}

fn main() {
    let bst = BinarySearchTree::new(10);
    bst.root().add_child(11);
    bst.root().add_child(5);
    //println!("BST is: {:?}", bst);

    assert_eq!(bst.is_present(10), true);
    assert_eq!(bst.is_present(11), true);
    assert_eq!(bst.is_present(5), true);

    assert_eq!(bst.is_present(6), false);
}

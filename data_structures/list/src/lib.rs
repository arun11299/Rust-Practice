use std::boxed::Box;
use std::mem;
use std::cell::RefCell;
use std::cmp::PartialEq;

pub struct Node<T: PartialEq>
{
    pub value : T,
    pub next  : Option<Box<Node<T>>>,
}

impl<T> Node<T> where T : PartialEq {
    ///
    pub fn new(value : T) -> Node<T>
    {
        Node {
            value,
            next : None,
        }
    }
    ///
    fn append(&mut self, value: T)
    {
        let mut anchor = &mut self.next;
        loop {
            let tmp = anchor;
            if let Some(ref mut n) = *tmp {
                anchor = &mut n.next;
            } else {
                anchor = tmp;
                break;
            }
        }
        *anchor = Some(Box::new(Node::new(value)));
    }
    ///
    fn append_rec(&mut self, value : T)
    {
        loop {
            if self.next.is_none() {
                self.next = Some(Box::new(Node::new(value)));
                break;
            } else {
                for n in self.next.iter_mut() {
                    n.append(value);
                    break;
                }
                break;
            }
        }
    }
    ///
    fn remove(&mut self, prev: &mut Node<T>, value : T) -> bool
    {
        if self.value == value {
            prev.next = self.next.take();
            return true;
        }
        let mut next = &mut self.next;
        prev.next = prev.next.take();

        let mut prev = match prev.next {
            Some(ref mut n) => n,
            None => return false
        };
        loop {
            let tmp = next;
            if let Some(ref mut node) = *tmp {
                if node.value == value {
                    prev.next = node.next.take();
                    return true;
                } else {
                    next = &mut node.next;
                }
            } else {
                return false;
            }
        }
        return false;
    }
}

pub struct List<T>
    where T : PartialEq
{
    pub len  : usize,
    pub head : Option<Box<Node<T>>>,
}

impl<T> List<T> where T: PartialEq {
    ///
    pub fn new() -> List<T>
    {
        List{
            len : 0,
            head : None,
        }
    }
    ///
    pub fn append(&mut self, value : T)
    {
        if let Some(ref mut node) = self.head {
            node.append(value);
        } else {
            self.head = Some(Box::new(Node::new(value)));
        }
        self.len += 1;
    }
}

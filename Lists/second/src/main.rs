use std::mem;

struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {head : None}
    }

    fn push(&mut self, value: T) {
        let new_node = Node {
            elem : value,
            next : self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(boxed_node) => {
                let result = boxed_node.elem;
                self.head = boxed_node.next;
                Some(result)
            }
        }
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

////// Iterator implementation ////////
struct IntoIter<T>(List<T>);

impl<T> List<T> {
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T> {
    next : Option<&'a Node<T>>
}

impl<T> List<T> {
    fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next : self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}


struct IterMut<'a, T> {
    next : Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next : self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(
            |node| {
                self.next = node.next.as_mut().map(|node| &mut **node);
                &mut node.elem
            }
        )
    }
}

fn main() {
    let mut lists = List::new();
    lists.push(1);
    lists.push(2);
    lists.push(3);

    let mut iter = lists.into_iter();

    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
}

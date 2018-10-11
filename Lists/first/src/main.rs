use std::mem;

struct List {
    head : Link,
}

struct Node {
    elem : i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    fn new() -> Self {
        List {head : Link::Empty}
    }

    fn push(&mut self, value : i32) {
        let new_node = Node {
            elem : value,
            next : mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                let result = Some(node.elem);
                self.head = node.next;
                result
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }

    }
}

fn main() {
    let mut lists = List::new();
    lists.push(1);
    lists.push(2);

    let popped = lists.pop().unwrap();
    println!("popped value is {}", popped);
}
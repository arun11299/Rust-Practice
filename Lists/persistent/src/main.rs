use std::rc::Rc;

struct List<T> {
    head : Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem : T,
    next : Link<T>,
}


impl<T> List<T> {
    fn new() -> Self {
        List {
            head : None
        }    
    }

    fn append(&self, elem : T) -> List<T> {
        List {
            head : Some(Rc::new(Node {
                elem : elem,
                next : self.head.clone()
            }))
        }
    }

    fn tail(&self) -> List<T> {
        List {
            head : self.head.as_ref().and_then(
                |node| node.next.clone()
            )
        }
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

fn main() {
    println!("Hello, world!");
}

extern crate list;
use list::List;

fn print(l : &List<i32>)
{
    let mut n = &l.head;
    loop {
        let tmp = n;
        if let Some(ref node) = *tmp {
            print!("{:?} ", node.value);
            n = &node.next;
        } else {
            break;
        }
    }
}

fn main() {
    let mut l = List::new();
    l.append(1);
    l.append(2);
    l.append(3);

    print(&l);
}

extern crate bst;

use bst::BST;
use bst::Node;

fn main() {
    let mut tree : BST<i32> = BST::new();
    tree.add(50); // the head
    /*
    if let Some(ref mut root) = tree.head {
        BST::move_to_leftmost(root.as_raw(), Box::new(Node::new(5)));
        BST::move_to_leftmost(root.as_raw(), Box::new(Node::new(6)));
        BST::move_to_leftmost(root.as_raw(), Box::new(Node::new(7)));
    }
    */
    tree.add(20);
    tree.add(10);
    tree.add(30);

    tree.print(|elem| println!("{}", elem));

    tree.remove(20);

    tree.print(|elem| println!("{}", elem));
}

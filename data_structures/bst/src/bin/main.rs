extern crate bst;

use bst::BST;
use bst::Node;

fn main() {
    let mut tree : BST<i32> = BST::new();
    tree.add(50); // the head
    tree.add(20);
    tree.add(10);
    tree.add(30);
    tree.add(60);
    tree.add(70);
    tree.add(55);

    tree.print(|elem| println!("{}", elem));

    tree.remove(20);

    println!("------------------");

    tree.print(|elem| println!("{}", elem));

    tree.remove(60);

    println!("------------------");

    tree.print(|elem| println!("{}", elem)); 
}

extern crate bst;

use bst::BST;

fn main() {
    let mut tree : BST<i32> = BST::new();
    tree.add(20);
    tree.add(10);
    tree.add(30);

    tree.print(|elem| println!("{}", elem));
}

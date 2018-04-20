extern crate bst;

use bst::BST;

fn main() {
    let mut tree : BST<i32> = BST::new();
    tree.add(20);
    tree.add(10);
    tree.add(30);
    tree.add(5);
    tree.add(12);

    tree.print(|elem| println!("{}", elem));

    tree.remove(30);
    tree.print(|elem| println!("{}", elem));
}

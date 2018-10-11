

fn binary_search<T : std::cmp::PartialOrd>(collection : &[T], needle: T) -> bool {
    let mut slice = collection;

    while slice.len() > 0 {
        println!("slice len: {}", slice.len());
        let len = slice.len();
        let mid = (len >> 1) - 1;
        let mid_elem = &slice[mid];

        if *mid_elem > needle {
            slice = &slice[0 .. mid];
        } else if *mid_elem < needle {
            slice = &slice[(mid + 1) ..];
        } else {
            return true;
        }
    }
    false
}

fn main() {
    let mut vec = vec![2, 3, 1, 4, 5, 7, 9, 0];
    vec.sort();
    println!("{:?}", vec);
    let found = binary_search(&vec, 10);
    println!("Found: {}", found);
}

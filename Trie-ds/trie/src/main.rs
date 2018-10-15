
#[derive(Clone)]
struct TrieNode {
    array : Vec<Option<Box<TrieNode>>>,
    marker : bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            array : vec![None; 26],
            marker : false,
        }
    }
}

struct Trie {
    root : TrieNode
}

impl Trie {
    /// Create a new Trie instance
    fn new() -> Self {
        Trie {
            root : TrieNode::new(),
        }
    }

    /// Add a string to the trie
    fn add(&mut self, data : String) {
        let mut current_node = &mut self.root;

        for ch in data.chars() {
            let idx = (ch as u8 - 'a' as u8) as usize;

            if  current_node.array[idx].is_none() {
                current_node.array[idx] = Some(Box::new(TrieNode::new()));
            }
            current_node = moving(current_node)
                           .array[idx]
                           .as_mut()
                           .map(|node| &mut *node)
                           .take()
                           .unwrap();
        }
        current_node.marker = true;
    }

    fn is_present(&self, data : String) -> bool {
        let mut current_node = &self.root;

        for c in data.chars() {
            let idx = (c as u8 - 'a' as u8) as usize;
            if current_node.array[idx].is_none() {
                return false;
            }
            current_node = moving(current_node)
                           .array[idx]
                           .as_ref()
                           .map(|node| &*node)
                           .take()
                           .unwrap();
        }

        return current_node.marker;
    }
}

/// The Rust trick to overwrite a mutable variable
/// in a for loop without any borrowing issues.
/// Creates pure rvalue reference if you will.
fn moving<T>(t : T) -> T { t }

fn main() {
    let mut t = Trie::new();
    t.add("test".to_owned());
    println!("{}", t.is_present("test".to_owned()));
}

macro_rules! char_at {
    ($str: ident, $index: ident) => {
        &$str[$index..$index + 1]
    }
}

fn basic_string_match(text : String, pattern : String) -> Option<usize>
{
    if text.len() == 0 {
        return None;
    }
    if pattern.len() == 0 {
        return None;
    }
    if text.len() < pattern.len() {
        return None;
    }

    let tlen = text.len();
    let plen = pattern.len();

    for mut i in 0..tlen {
        for mut j in 0..plen {
            let t =  char_at!(text, i);
            let p = char_at!(pattern, j);

            if t == p {
                if j == plen - 1 {
                    return Some(i - j);
                }
                i = i + 1;
            } else {
                i = i - j;
                break;
            }
        }
    }
    return None;
}

fn preconditions_match(text : &str, pattern : &str) -> bool {
    return (text.len() == 0 ) 
    || (pattern.len() == 0) 
    || (text.len() < pattern.len());
}

fn basic_string_match_2(text : &str, pattern : &str) -> Option<usize> {
    if preconditions_match(text, pattern) {
        return None;
    }
    let tlen = text.len();
    let plen = pattern.len();
    let text_b = text.as_bytes();
    let pattern_b = pattern.as_bytes();

    for mut i in 0 .. tlen {
        for mut j in 0 .. plen {
            if text_b[i] == pattern_b[j] {
                if j == plen - 1 {
                    return Some(i - j);
                }
                i += 1;
            } else {
                i -= j;
                break;
            }
        }
    }
    return None;
}

fn main() {
    match basic_string_match_2("this is a long string", "long")
    {
        None => {
            println!("Not found!");
        },
        Some(pos) => {
            println!("Found at position: {}", pos);
        }
    }
}

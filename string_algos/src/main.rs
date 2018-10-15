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

//////////////////////////////////////////////////////////////////////////
/// KMP Search Algorithm
//////////////////////////////////////////////////////////////////////////

fn compute_kmp_array(pattern : &str) -> Vec<usize> {
    let plen = pattern.len();
    let pbytes = pattern.as_bytes();
    let mut res = vec![0; plen];
    // The first element is always zero
    res[0] = 0;
    let mut len = 0;
    let mut i = 1;

    while i < plen {
        if pbytes[i] == pbytes[len] {
            len += 1;
            res[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = res[len - 1];
            } else {
                res[i] = 0;
                i += 1;
            }
        }
    }
    return res;
}

fn kmp_search(text: &str, pattern: &str) -> Option<usize> {
    if preconditions_match(text, pattern) {
        return None;
    }

    let kmp_arr = compute_kmp_array(pattern);
    let tlen = text.len();
    let plen = pattern.len();
    let tbytes = text.as_bytes();
    let pbytes = pattern.as_bytes();

    let mut i = 0;
    let mut j = 0;

    while i < tlen {
        if tbytes[i] == pbytes[j] {
            i += 1;
            j += 1;
            // Check if pattern found
            if j == plen {
                return Some(i - plen);
             } 
        } else {
            if j != 0 {
                j = kmp_arr[j - 1];
            } else {
                i += 1;
            }
        }
    }
    return None;
}

///////////////////////////////////////////////////////////
/// Using traits as the standard library does
///////////////////////////////////////////////////////////

#[derive(Debug)]
enum SearchStep {
    /// If the pattern matches the haystack[a..b]
    Match(usize, usize),
    /// If the pattern is rejected from haystack[a..b]
    Reject(usize, usize),
    /// Done scanning through all the bytes in haystack
    Done,
}

trait Searcher<'a> {
    /// Get the underlying haystack
    fn haystack(&self) -> &'a str;

    /// Returns a stream of Match Reject and finally Done.
    fn next(&mut self) -> SearchStep;

    /// Only return the match
    #[inline]
    fn next_match(&mut self) -> Option<(usize, usize)> {
        loop {
            match self.next() {
                SearchStep::Match(a, b) => return Some((a, b)),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }
}

///////////////////////////////////////////////////////
/// A searcher for a character
///////////////////////////////////////////////////////

struct CharSearcher<'a> {
    haystack : &'a str,
    needle : char,
    /// basically begin() in C++
    /// The first index to search...
    finger : usize,
    /// basically end() in C++
    /// The last index + 1
    finger_back : usize,

    /// extra utf-8 things
    utf8_size : usize,
    utf8_copy : [u8; 4],
}

impl<'a> Searcher for CharSearcher<'a> {

    fn haystack(&self) -> &'a str {
        self.haystack
    }

    fn next(&mut self) -> SearchStep {
        let old_finger = self.finger;
        let slice = unsafe {
            self.haystack.get_unchecked(self.finger..self.finger_back)
        };
        let mut iter = slice.chars();
        let old_len = iter.iter.len();

        if Some(ch) = iter.next() {
            self.finger += old_len - iter.iter.len();
            if ch == self.needle {
                return SearchStep::Match(old_finger, self.finger);
            } else {
                return SearchStep::Reject(old_finger, self.finger);
            }
        } else {
            return SearchStep::Done;
        }
    }

    #[inline]
    fn next(&mut self) -> SearchStep {

    }
}

////////////////////////////////////////////////////////
fn main() {
    match kmp_search("this is a long string", "long")
    {
        None => {
            println!("Not found!");
        },
        Some(pos) => {
            println!("Found at position: {}", pos);
        }
    }
}

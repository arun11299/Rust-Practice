
fn main() {
    println! ("Days {}", 31);

    println! ("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println! ("{subject} {verb} {object}", 
              object="the lazy dog",
              verb="jumps over",
              subject="the quick brown fox");

    #[allow(dead_code)]
    struct Structure(i32);
}

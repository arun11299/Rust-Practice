
fn longest<'a>(a : &'a str, b : &'a str) -> &'a str
{
    if a.len() > b.len() {
        return a;
    }
    return b;
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "abc";

    let mut result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    /*
    let string3 = String::from("xyz");
    {
        let string4 = String::from("xy");
        result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is: {}", result);
    }
    */
}

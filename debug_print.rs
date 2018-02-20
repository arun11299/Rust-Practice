#[derive(Debug)]
struct UnPrintable
{
    id : i32
}

#[derive(Debug)]
struct Printable
{
    id : i32,
    unp : UnPrintable
}

fn main() {
    println! ("Printable struct: {:?}", Printable{ id : 42, unp : UnPrintable { id : 43 } });
}

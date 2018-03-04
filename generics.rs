
fn identity<T>(arg : T) -> T
{
    arg
}

struct GenericVal<T>(T,);

impl<T> GenericVal<T> {

    fn what(self) -> (){
        println!("Implementation for generic types");
    }

}

/// Implement DoubleDrop for 
/// any generic type parameter `T`
trait DoubleDrop<T> {
    fn double_drop(self, _ : T);
}

/// Implement DoubleDrop<T> for any
/// generic type `U`
impl<T, U> DoubleDrop<T> for U {

    /// This method takes ownership of both
    /// passed arguments, deallocating both.
    fn double_drop(self, _ : T) {}

}

struct Empty;
struct Null;

fn main() {
    identity('a');

    let a1 = GenericVal(42u32);
    let a2 = GenericVal(String::from("Arun"));
    let a3 = GenericVal(1.0f32);

    a1.what();
    a2.what();
    a3.what();

    let e = Empty;
    let n = Null;

    e.double_drop(n);

    // ERROR: value used here after move
    //e;
}

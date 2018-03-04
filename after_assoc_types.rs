// A container for two integers
struct Container(i32, i32);

trait Contains {
    /// Define generic types which methods would be able to reuse
    type A;
    type B;

    ///
    fn contains(&self, &Self::A, &Self::B) -> bool;
    ///
    fn first(&self) -> i32;
    ///
    fn last(&self) -> i32;
}

impl Contains for Container {
    /// Specify what types `A` and `B` are for Container.
    type A = i32;
    type B = i32;

    ///
    fn contains(&self, num1 : &i32, num2 : &i32) -> bool
    {
        return (&self.0 == num1) && (&self.1 == num2);
    }
    ///
    fn first(&self) -> i32
    {
        self.0
    }
    ///
    fn last(&self) -> i32
    {
        self.1
    }
}

// `C` contains `A and B`. In light of that, having to 
// express `A` anf `B` again is a nuisance.
fn difference<C>(cont : C) -> i32
    where C : Contains
{
    cont.last() - cont.first()
}


fn main() {
    let num1 : i32 = 1;
    let num2 : i32 = 2;
    let c = Container(num1, num2);

    c.contains(&num1, &num2);

    difference(c);
}

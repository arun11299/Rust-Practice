// A container for two integers
struct Container(i32, i32);

trait Contains<A, B> {
    ///
    fn contains(&self, &A, &B) -> bool;
    ///
    fn first(&self) -> i32;
    ///
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
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
fn difference<A, B, C>(cont : C) -> i32
    where C : Contains<A, B>
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

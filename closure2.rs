
/*
 * Apply is a function which takes a closure as argument.
 */
fn apply<F>(func : F)
    /*
     * Constraining the type of closure to FnOnce which
     * takes the ownership of the data.
     */
    where F : FnOnce()
{
    //call the function
    func();
}

fn wrap() -> Box<Fn()>
{
    let text = "wrapped_text";
    // If not moved its a compilation error
    // otherwise it would be passing reference to a local
    // variable!!
    return Box::new(move || println!("{}", text));
}

fn main() {

    let msg = "greetings";
    let mut omsg = msg.to_owned();

    let c1 = || {
        println!("Fn: {}", msg);
    };

    apply(c1);

    let c2 = move || {
        println!("FnOnce: {}", msg);
    };
    apply(c2);
    //omsg is still accessible
    println!("outside closure(1): {}", omsg);

    let c3 = move || {
        println!("FnOnce(2): {}", omsg);
    };
    apply(c3);

    //access to omsg is not allowed
    //println!("outside closure(2): {}", omsg);
    
    let wrapped = wrap();
    //call
    wrapped();
}


fn main() {
    #[allow(dead_code)]
    let logical : bool = true;
    #[allow(dead_code)]
    let a_float : f64 = 1.0;
    #[allow(dead_code)]
    let an_int = 42u64;
    
    // Defaults
    #[allow(dead_code)]
    let def_float = 1.0; //f64
    #[allow(dead_code)]
    let def_int = 42; // i32

    //Mutable variables
    #[allow(dead_code)]
    let mut mutable = 12; // mutable i32
    mutable = 21;

    // variables can be overwritten with shadowing
    #[allow(dead_code)]
    let mutable = true;
}

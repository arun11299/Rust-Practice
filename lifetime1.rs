
struct OfReferences<'a, 'b> {
    x : &'a i32,
    y : &'b i32,
}

/*
fn get_a_value<'a, 'b>(v : &OfReferences<'a, 'b>, cond : bool) -> &'a i32
{
    if (cond) {
        return v.x;
    }
    return v.y;
}
*/

fn get_x<'a>(v : &'a OfReferences) -> &'a i32
{
    v.x
}

fn main() {
    let a : i32 = 1;
    let r: &i32;
    {
        let b : i32 = 2;
        let val = OfReferences {
            x : &a,
            y : &b,
        };

        //let res = get_a_value(&val, true);
        //println!("Res is {}", res);

        let r = get_x(&val);
    }
}

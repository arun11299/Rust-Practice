
struct Thing { count : u32, }

struct Combined<'a>(Thing, &'a u32);

fn make_combined<'b>() -> Combined<'b>
{
    let thing = Thing{ count : 42 };
    let c = &thing.count;
    return Combined(thing, c);
}

fn main() {
    let c = make_combined();
}

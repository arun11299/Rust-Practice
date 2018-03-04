
struct ToDrop;

impl Drop for ToDrop {
    ///
    fn drop(&mut self) {
        println!("cleaning up ToDrop");
    }
}

struct Resource
{
    res_ : Box<u32>,
}

impl Resource {
    ///
    fn new(num : u32) -> Resource
    {
        Resource{
            res_ : Box::new(num),
        }
    }
}

fn ret_pointer() -> Box<i32>
{
    let ptr = Box::new(42i32);
    return ptr;
}

fn main() {
    let a = ToDrop;
    let r = Resource::new(5u32);
}

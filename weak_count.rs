use std::rc::Rc;

fn main() {
    let wa;
    {
        let a = Rc::new(5);
        wa = Rc::downgrade(&a);
        println!("Strong ref count of a {}", Rc::strong_count(&a));
        println!("Weak ref count of a {}", Rc::weak_count(&a));
    }
    let sa = wa.upgrade();
    assert!(sa.is_none());
}

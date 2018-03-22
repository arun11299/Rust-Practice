
trait Messenger {
    fn send(&self, msg: &str);
}

/// 'a on LimitTracker specifies the lifetime
/// associated with th einstance of LimitTracker.
/// Using 'a plainly on `messenger` will not work
/// since `T` wont be having any lifetime parameter.
/// Therefore its necessary to provide a lifetime parameter
/// for T as well.
struct LimitTracker<'a, T>
    where T : 'a + Messenger
{
    messenger : &'a T,
    value : usize,
    max : usize,
}

impl<'a, T> LimitTracker<'a, T> 
    where T : Messenger {
    ///
    fn new(messenger: &T, max: usize) -> LimitTracker<T>
    {
        LimitTracker{
            messenger,
            value : 0,
            max,
        }
    }

    ///
    fn set_value(&mut self, value: usize)
    {
        self.value = value;
        let percent = self.value as f64 / self.max as f64;

        if percent >= 0.7 && percent < 0.9 {
            self.messenger.send("Wartning: Used over 75% of allocated quota");
        } else if percent >= 0.9 && percent < 1.0 {
            self.messenger.send("Urgent Warning: You have used over 90% of your allocated quota");
        } else if percent >= 1.0 {
            self.messenger.send("Error: your quota is over");
        }
    }
}

use std::cell::RefCell;

struct MockMessenger
{
    sent_messages : RefCell<Vec<String>>,
}

impl MockMessenger {
    ///
    fn new() -> MockMessenger
    {
        MockMessenger{ sent_messages : RefCell::new(vec![]) }
    }
}

impl Messenger for MockMessenger {
    ///
    fn send(&self, msg: &str)
    {
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}

fn main() {
    let msgn = MockMessenger::new();
    let mut lt = LimitTracker::new(&msgn, 10);
    lt.set_value(5);
}

#![allow(dead_code)]

enum WebEvent
{
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x : i32, y : i32},
}

fn inspect(wevent : WebEvent)
{
    match wevent {
        WebEvent::PageLoad    => println!("page loaded"),
        WebEvent::PageUnload  => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed key '{}'", c),
        WebEvent::Paste(s)    => println!("copied\"{}\"", s),
        WebEvent::Click{x, y} => {
            println!("clicked at x={}, y={}", x, y);
        },
    }
}


fn main()
{
    let pressed = WebEvent::KeyPress('c');
    let pasted = WebEvent::Paste("Arun".to_owned());
    let clicked = WebEvent::Click{x : 0, y : 1};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(clicked);
    inspect(load);
    inspect(unload);
}

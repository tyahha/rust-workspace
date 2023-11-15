#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn smart_pointer_main() {
    let b = Box::new(5);
    println!("Box: {}", b);

    let list = Cons(
        1,
        Box::new(Cons(2,
            Box::new(Nil)
        ))
    );

    println!("list: {:?}", list);
}

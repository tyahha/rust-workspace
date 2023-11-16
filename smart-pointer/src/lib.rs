#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;
use List::{Cons, Nil};

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("hello, {}!", name);
}

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

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let y2 = Box::new(x);
    assert_eq!(5, *y2);

    let y3 = MyBox::new(x);
    assert_eq!(5, *y3);

    let s1 = "aaa";
    hello(s1);
    let s2 = Box::new("bbb");
    hello(&s2);
    let s3 = MyBox::new("ccc");
    hello(&s3);
    let s4 = MyBox::new(String::from("ddd"));
    hello(&s4);
}

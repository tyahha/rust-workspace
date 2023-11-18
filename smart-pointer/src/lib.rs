#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::ops::Deref;
use std::rc::Rc;
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

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("drop!!! {}", self.data);
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
        Rc::new(Cons(2,
            Rc::new(Nil)
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

    let f = CustomSmartPointer { data: String::from("smart pointer first")};
    drop(f);
    // println!("f: {}", f.data); // compile error
    let c = CustomSmartPointer { data: String::from("smart pointer c")};
    let d = CustomSmartPointer { data: String::from("smart pointer d")};
    println!("created CustomSmartPointer");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("counter after creating a: {}", Rc::strong_count(&a));
    println!("a: {:?}", a);

    let b = Cons(3, Rc::clone(&a));
    println!("counter after creating b: {}", Rc::strong_count(&a));
    println!("b: {:?}", b);

    {
        let c = Cons(4, Rc::clone(&a));
        println!("counter after creating c: {}", Rc::strong_count(&a));
        println!("c: {:?}", c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

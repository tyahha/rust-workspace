mod list_with_rc_and_refcell;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::ops::Deref;
use std::rc::Rc;
use List::{Cons, Nil};
use crate::list_with_rc_and_refcell::list_with_rc_and_refcell_main;

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

trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
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

    list_with_rc_and_refcell_main();
}

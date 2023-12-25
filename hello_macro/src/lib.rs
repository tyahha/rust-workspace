pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Test;

pub fn hello_macro_main() {
    println!("hello macro");
}
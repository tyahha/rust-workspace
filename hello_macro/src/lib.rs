extern crate hello_macro_derive;

use hello_macro_derive::HelloMacro;

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Test;

pub fn hello_macro_main() {
    Test::hello_macro();
}
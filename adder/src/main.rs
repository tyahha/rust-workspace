extern crate add_one;
extern crate add_two;
extern crate smart_pointer;

fn main() {
    println!("4 + 1 = {}", add_one::add_one(4));
    println!("6 + 2 = {}", add_two::add_two(6));

    smart_pointer::smart_pointer_main();
}

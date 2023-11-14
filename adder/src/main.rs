extern crate add_one;
extern crate add_two;

fn main() {
    println!("4 + 1 = {}", add_one::add_one(4));
    println!("6 + 2 = {}", add_two::add_two(6));
}

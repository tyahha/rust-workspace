extern crate add_one;
extern crate add_two;
extern crate smart_pointer;
extern crate fearless_concurrency;
extern crate oop_in_rust;
extern crate blog;
use blog::Post;

extern crate pattern_matching;
extern crate advanced_features;
extern crate hello_macro;

fn main() {
    println!("4 + 1 = {}", add_one::add_one(4));
    println!("6 + 2 = {}", add_two::add_two(6));

    smart_pointer::smart_pointer_main();
    fearless_concurrency::fearless_concurrency_main();
    oop_in_rust::oop_in_rust_main();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    pattern_matching::pattern_matching_main();

    advanced_features::advanced_features_main();

    hello_macro::hello_macro_main();
}

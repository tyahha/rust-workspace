pub fn pattern_matching_main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(v) = stack.pop() {
        println!("Value in stack: {}", v);
    }
}
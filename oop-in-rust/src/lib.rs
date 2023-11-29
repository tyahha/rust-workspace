trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

pub fn oop_in_rust_main() {
    println!("oop in rust");
}
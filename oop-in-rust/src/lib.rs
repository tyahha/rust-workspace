trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{:?}", self)
    }
}

#[derive(Debug)]
struct Select {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for Select {
    fn draw(&self) {
        println!("{:?}", self)
    }
}

pub fn oop_in_rust_main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 32,
                height: 11,
                label: String::from("button"),
            }),
            Box::new(Select {
                width: 12,
                height: 100,
                options: vec![
                    String::from("option1"),
                    String::from("option2"),
                ],
            })
        ]
    };

    screen.run();
}
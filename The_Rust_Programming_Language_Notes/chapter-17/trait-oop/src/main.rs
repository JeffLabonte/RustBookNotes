extern crate trait_oop;
use trait_oop::Draw;
use trait_oop::{Screen, Button};

struct SelectBox {
    width: i32,
    height: i32,
    options: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self){
        // TODO Implement actual code to draw SelectBox
    }
}


fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height:100,
                options: vec![
                    String::from("Hey"),
                    String::from("What's"),
                    String::from("up!"),
                ]
            }),
            Box::new(Button{
                width: 100,
                height: 100,
                label: String::from("My new button")
            })
        ]
    };

    screen.run();
}

extern crate trait_oop;
use trait_oop::Draw;

struct SelectBox{
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

}

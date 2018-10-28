pub trait Draw{
    pub draw(&self);
}

/// First iteration of code
//pub struct Screen{
//    pub components: Vec<Box<dyn Draw>>,
//}
//
//impl Screen{
//    pub fn run(&self){
//        for component in self.components.iter(){
//            component.draw();
//        }
//    }
//}

pub struct Screen<T: Draw>{
    pub components: Vec<T>.
}

impl<T> Screen <T>
    where T: Draw{
        pub fn run(&self){
            for component in self.components.iter(){
                component.draw();
            }
        }
}

pub struct Button{
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self){
        // TODO Code to implement drawing this component
    }
}

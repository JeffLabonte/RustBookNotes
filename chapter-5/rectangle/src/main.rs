#[derive(Debug)] // To make the println! in rectangle_rect_way work (last print)
struct Rectangle{ // This is a traits derived to our struct
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other:& Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle { // Associated function
        Rectangle{ width: size, height: size }
    }
}

fn main() {
    rectangle_variable_way();
    rectangle_tuple_way();
    rectangle_rect_way();
    create_square(); //Associate functions used
}

fn rectangle_variable_way(){
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width, height)
    );
}

fn area(width: u32, height: u32) -> u32{
    width * height
}

fn rectangle_tuple_way(){
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rect1)
    );
}

fn area_tuple(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}

fn rectangle_rect_way(){
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    let rect2 = Rectangle{
        width: 40,
        height: 50
    };


    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rect1) // We borrow to keep ownership here!
    );

    println!("This is our rectangle: {:?}", rect1);

    println!("Calling Rectangle method area {}", rect1.area());

    println!("Rect1 can hold rect2? {}", rect1.can_hold(&rect2));
}

fn area_struct(rect: &Rectangle) -> u32{ // &Rectangle is borrowed ( Referenced )
    rect.width * rect.height 
}

fn create_square(){
    let sq = Rectangle::square(15); // :: is used for namspace and to call associated function

    println!("This is our square {}", sq.area());

    println!("This is the object {:?}", sq);
}

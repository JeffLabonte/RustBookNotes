struct Rectangle{
    width: u32,
    height: u32
}

fn main() {
    rectangle_variable_way();
    rectangle_tuple_way();
    rectangle_rect_way();
}

fn rectangle_variable_way(){
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width, height)
    )
}

fn area(width: u32, height: u32) -> u32{
    width * height
}

fn rectangle_tuple_way(){
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rect1)
    )
}

fn area_tuple(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}

fn rectangle_rect_way(){
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };


    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rect1) // We borrow to keep ownership here!
    )
}

fn area_struct(rect: &Rectangle) -> u32{ // &Rectangle is borrowed ( Referenced )
    rect.width * rect.height 
}

fn main() {
    rectangle_variable_way();
    rectangle_tuple_way();
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

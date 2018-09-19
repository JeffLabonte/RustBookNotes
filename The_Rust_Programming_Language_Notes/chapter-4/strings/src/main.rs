fn main() {
    let mut s = String::from("Hello"); // Create string from Litteral using String namespace

    s.push_str(", world"); // push_str() appends a litteral to a String

    println!("{}", s); // This will print 'Hello, world
}

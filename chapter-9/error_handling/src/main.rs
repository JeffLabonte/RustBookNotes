use std::io;
use std::io::Read;
use std::fs::File;

fn main(){
    let result = read_username_from_file();
    println!("{:?}", result);
    let result = read_username_concise();
    println!("{:?}", result);
    let result = read_username_chaining();
    println!("{:?}", result);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_username_concise() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // Can only be used within a function that returns a result
    let mut s = String::new();
    f.read_to_string(&mut s)?; // Can only be used within a function that returns a result
    Ok(s)
}

fn read_username_chaining() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; // Can only be used within a function that returns a result
    Ok(s)
}

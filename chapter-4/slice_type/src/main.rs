fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // This empties the string, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with.word is now totally invalid!

    let mut s_1 = String::from("Hello World");

    let slice = first_word_rev2(&s_1);
    println!("{}", slice);

    s_1.clear(); // Should make it all fail!

     let my_string = String::from("hello world");

     let word_1 = first_word_rev3(&my_string[..]);

     let my_string_literal = "hello world";

     let word_2 = first_word_rev3(&my_string_literal[..]);

     let word_3 = first_word_rev3(my_string_literal); // Since it is &str
}

fn first_word(s:&String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }

    s.len()
}

fn first_word_rev2(s:&String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_rev3(s: &str) -> &str { // Signature is the only difference!
    let bytes = s.as_bytes();       //Able to use string literal and String

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}

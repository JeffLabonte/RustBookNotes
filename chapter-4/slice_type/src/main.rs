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

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];

    let initial_scores = vec![10, 50];

    let scores_second:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect(); // <_,_> is required here since rust
    // has no idea what data structure you might want to bring in!
    
    let field_name = String::from("My favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value); // At this point the two strings belongs to the hashmap
    // You can't reuse the variables field_name and field_value
    //
    // However we could pass the reference of the strings into the map
    // They would still be valid to them here then! But they have to stay "alive" as long as the
    // hashmap hasn't been cleaned up
}

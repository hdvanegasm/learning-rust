use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).unwrap_or(&0);
    println!("The score for {} is {}", team_name, score);

    for (key, value) in &scores {
        println!("Key: {} - Value: {}", key, value);
    }

    let key = String::from("Hello");
    let value = String::from("World!");
    let mut map = HashMap::new();
    map.insert(key, value);

    // Key and value are not valid here because map has taken ownership.
    // println!("Key: {}", key);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Black")).or_insert(500);

    println!("Current state: {:?}", scores);

    // Updating a value according to old value.
    let blue_points = scores.entry(String::from("White")).or_insert(0);
    *blue_points += 1;

    println!("Current state: {:?}", scores);

    // Counting the words in a String
    let text = String::from("The world is the most beautifull world");
    let mut counter_map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert returns a mutable reference to the value of the entry.
        let count = counter_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counter: {:?}", counter_map);
}

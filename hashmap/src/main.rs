use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("blue"), 40);
    scores.insert(String::from("yellow"), 40);
    println!("{:?}", scores);

    let blue_score = scores.get("blue");
    match blue_score {
        Some(score) => println!("The score of blue team is: {}", score),
        None => println!("There is no score for blue team"),
    }

    let text: &str = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // if word is not present in the map, insert it with a value of 0 and return a mutable reference to the value
        *count += 1;
    }

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

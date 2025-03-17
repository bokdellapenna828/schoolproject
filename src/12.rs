use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", "fruit");
    map.insert("carrot", "vegetable");

    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }
}

use std::cmp;

fn main() {
    let mut array: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    if let [min, max] = array.iter().minmax() {
        println!("Minimum value: {}", min);
        println!("Maximum value: {}", max);
    } else {
        eprintln!("Array is empty or contains only one element.");
    }
}

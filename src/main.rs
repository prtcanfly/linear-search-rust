fn linear_search(array: &[i32], target: i32) -> Option<usize> {
    for (i, &item) in array.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let array = [1, 3, 5, 7, 9];
    let target = 5;

    println!("{:?}", array);
    match linear_search(&array, target) {
        Some(i) => println!("Found {} at index {}", target, i),
        None => println!("{} not found in array", target),
    }
}

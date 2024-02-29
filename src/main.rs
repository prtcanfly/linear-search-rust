// create the search function that takes an array and the target as input and returns an option
fn linear_search(array: &[i32], target: i32) -> Option<usize> {
    // insert the array into the iterator
    for (i, &item) in array.iter().enumerate() {
        // check if the value equals the target
        if item == target {
            return Some(i);
        }
    }
    
    // return none if there is no value equal to the target
    None
}

fn main() {
    // declare the array and the target
    let array = [1, 3, 5, 7, 9];
    let target = 5;

    // print the array for reference
    println!("{:?}", array);

    // match the option types that get returned by the search
    match linear_search(&array, target) {
        // print the results
        Some(i) => println!("Found {} at index {}", target, i),
        None => println!("{} not found in array", target),
    }
}

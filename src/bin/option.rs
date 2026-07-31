fn find_number(num: i32) -> Option<i32> {
    if num > 0 {
        Some(num)
    } else {
        None
    }
}

fn main() {
    let result = find_number(10);

    match result {
        Some(value) => println!("Number is: {}", value),
        None => println!("No valid number found"),
    }
}
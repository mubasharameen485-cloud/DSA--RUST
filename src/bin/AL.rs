fn main() {
    let num = 8;

    if num > 0 && num % 2 == 0 {
        println!("Positive Even Number");
    } else if num > 0 && num % 2 != 0 {
        println!("Positive Odd Number");
    } else {
        println!("Negative or Zero");
    }
}
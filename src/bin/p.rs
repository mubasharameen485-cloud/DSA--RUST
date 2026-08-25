fn main() {
    let a = 10;
    let b = 20;

    let p1 = &a;
    let p2 = &b;

    let sum = *p1 + *p2;

    println!("First number = {}", *p1);
    println!("Second number = {}", *p2);
    println!("Sum = {}", sum);
}

use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter first angle:");
    io::stdin().read_line(&mut a).unwrap();

    println!("Enter second angle:");
    io::stdin().read_line(&mut b).unwrap();

    println!("Enter third angle:");
    io::stdin().read_line(&mut c).unwrap();

    let a: i32 = a.trim().parse().unwrap();
    let b: i32 = b.trim().parse().unwrap();
    let c: i32 = c.trim().parse().unwrap();

    if a + b + c != 180 {
        println!("Invalid Triangle");
    } else if a == 60 && b == 60 && c == 60 {
        println!("Equilateral Triangle");
    } else if a == b || b == c || a == c {
        println!("Isosceles Triangle");
    } else {
        println!("Scalene Triangle");
    }
}
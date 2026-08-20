fn main() {
    let mut x = Some(10);

    if let Some(value) = x.as_mut() {
        *value = 20;
    }

    println!("{:?}", x);
}

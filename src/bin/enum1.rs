enum Message {
    Write(String),
    Quit,
    Number(i32),
}

fn main() {

    let message = Message::Write(String::from("Hello"));

    // 1. MATCHES ARE EXHAUSTIVE
    match message {
        Message::Write(text) => {
            println!("Write: {}", text);
        }

        Message::Quit => {
            println!("Quit");
        }

        Message::Number(number) => {
            println!("Number: {}", number);
        }
    }
}

struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&i32> {
        self.data.last()
    }

    fn display(&self) {
        println!("{:?}", self.data);
    }
}

fn main() {
    let mut s = Stack::new();

    s.push(10);
    s.push(20);
    s.push(30);

    println!("Stack:");
    s.display();

    println!("Top Element: {:?}", s.peek());

    println!("Popped: {:?}", s.pop());

    println!("After Pop:");
    s.display();
}
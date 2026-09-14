struct Stack {
    top: Option<Box<Node>>,
}

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Stack {
    fn new() -> Self {
        Stack { top: None }
    }

    fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.top.take(),
        });

        self.top = Some(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        match self.top.take() {
            Some(node) => {
                self.top = node.next;
                Some(node.value)
            }
            None => None,
        }
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("{:?}", stack.pop()); // Some(30)
    println!("{:?}", stack.pop()); // Some(20)
    println!("{:?}", stack.pop()); // Some(10)
    println!("{:?}", stack.pop()); // None
}
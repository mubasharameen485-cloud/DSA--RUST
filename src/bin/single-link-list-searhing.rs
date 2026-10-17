struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn search(&self, target: i32) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.value == target {
                return true;
            }
            current = &node.next;
        }
        false
    }

    pub fn print_list(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.insert(10);
    list.insert(20);
    list.insert(30);
    list.insert(40);

    list.print_list();

    let target1 = 20;
    if list.search(target1) {
        println!("Value {} found", target1);
    } else {
        println!("Value {} not found", target1);
    }

    let target2 = 99;
    if list.search(target2) {
        println!("Value {} found", target2);
    } else {
        println!("Value {} not found", target2);
    }
}

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

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn search_and_delete(&mut self, target: i32) -> bool {
        let mut current = &mut self.head;

        loop {
            match current {
                None => return false,
                Some(node) if node.value == target => {
                    let next_node = node.next.take();
                    *current = next_node;
                    return true;
                }
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }

    pub fn print_list(&self) {
        let mut current = &self.head;
        print!("List: ");
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push(50);
    list.push(40);
    list.push(30);
    list.push(20);
    list.push(10);

    println!("--- Initial List ---");
    list.print_list();

    let target1 = 30;
    println!("\nSearching and deleting: {}", target1);
    if list.search_and_delete(target1) {
        println!("Success: {} found and deleted.", target1);
    } else {
        println!("Failed: {} not found.", target1);
    }
    list.print_list();

    let target2 = 99;
    println!("\nSearching and deleting: {}", target2);
    if list.search_and_delete(target2) {
        println!("Success: {} found and deleted.", target2);
    } else {
        println!("Failed: {} not found.", target2);
    }
    list.print_list();
}


struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node {
            data: value,
            next: None,
        });

        let mut current = &mut self.head;

        while let Some(node) = current {
            current = &mut node.next;
        }

        *current = Some(new_node);
    }

    fn display(&self) {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = node.next.as_ref();
        }

        println!("None");
    }

    fn reverse(&mut self) {
        let mut prev: Option<Box<Node>> = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            current = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        self.head = prev;
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.insert(10);
    list.insert(20);
    list.insert(30);
    list.insert(40);
    list.insert(50);

    println!("Original Linked List:");
    list.display();

    list.reverse();

    println!("Reversed Linked List:");
    list.display();
}

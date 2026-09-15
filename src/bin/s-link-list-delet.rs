#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

pub struct MyList {
    head: Option<Box<Node>>,
}

impl MyList {
    pub fn new() -> Self {
        MyList { head: None }
    }

    // Insert data at the end
    pub fn insert_end(&mut self, val: i32) {
        let new_node = Box::new(Node {
            data: val,
            next: None,
        });

        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(new_node);
                break;
            }

            current = node.next.as_mut();
        }
    }

    // Delete node by value
    pub fn delete_by_value(&mut self, val: i32) {
        // Case 1: List is empty
        if self.head.is_none() {
            return;
        }

        // Case 2: Delete Head node
        if self.head.as_ref().unwrap().data == val {
            let old_head = self.head.take();

            self.head = old_head.unwrap().next;

            return;
        }

        // Case 3: Delete middle/end node
        let mut current = self.head.as_mut();

        while let Some(node) = current {
            // Check the next node
            if let Some(ref next_node) = node.next {
                if next_node.data == val {
                    // Remove target node
                    let target_node = node.next.take();

                    // Connect current node to target's next
                    node.next = target_node.unwrap().next;

                    return;
                }
            }

            // Move to next node
            current = node.next.as_mut();
        }
    }

    // Display the linked list
    pub fn display(&self) {
        let mut current = &self.head;

        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }

        println!("None");
    }
}

fn main() {
    let mut list = MyList::new();

    // Insert values
    list.insert_end(10);
    list.insert_end(20);
    list.insert_end(30);
    list.insert_end(40);

    println!("Original List:");
    list.display();

    println!("\nDeleting 20 (The Middle Node)...");
    list.delete_by_value(20);
    list.display();

    println!("\nDeleting 10 (The Head Node)...");
    list.delete_by_value(10);
    list.display();
}

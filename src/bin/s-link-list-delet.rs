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

pub fn insert_end(&mut self, val: i32) {
let new_node = Box::new(Node { data: val, next: None });
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
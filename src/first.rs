pub struct List {
    head: Link,
}
struct Node {
    elem: i32,
    next: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        // TODO
        let new_node = Box::new(Node {
            elem,
            // Replace the current head with Empty and get the old head
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = current_link {
            current_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

mod test {
    #[test]
    fn basics() {
        let mut list = crate::first::List::default();
        assert_eq!(list.pop(), None);
        list.push(2);
        list.push(4);
        list.push(10);
        assert_eq!(list.pop(), Some(10));
    }
}

use std::ptr::NonNull;
use std::option::Option::Some;
use std::fmt::Display;

struct OrderedLinkedList<T> {
    pub header: Option<NonNull<Node<T>>>,
}

impl<T> OrderedLinkedList<T> where T: PartialOrd + Display {
    pub fn new() -> Self {
        OrderedLinkedList {
            header: None,
        }
    }

    pub fn push(&mut self, value: T) {
        let node = Box::new(Node::new(value));
        unsafe {
            if let None = self.header {
                self.header = Some(Box::leak(node).into());
                return;
            }

            let mut current: NonNull<Node<T>> = Box::leak(node).into();
            let mut header_ref = self.header.unwrap();
            if header_ref.as_ref().value < current.as_ref().value {
                current.as_mut().next = self.header;
                self.header = Some(current);
                return;
            }

            let mut ptr = self.header.unwrap().as_mut().next;
            let mut prev_ptr = self.header;
            loop {
                if let Some(h) = ptr {
                    if h.as_ref().value < current.as_ref().value {
                        current.as_mut().next = ptr;
                        prev_ptr.unwrap().as_mut().next = Some(current);
                        break;
                    }
                } else {
                    prev_ptr.unwrap().as_mut().next = Some(current);
                    break;
                }
                prev_ptr = ptr;
                ptr = (*ptr.unwrap().as_ptr()).next;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let mut ptr = self.header;
            if let Some(mut t) = ptr {
                self.header = t.as_mut().next;
                let v = Box::from_raw(t.as_ptr());
                return Some(v.value);
            }
            None
        }
    }


    pub fn peek(&self) -> Option<&T> {
        unsafe {
            let mut ptr = self.header;
            if let Some(t) = ptr {
                return Some(&t.as_ref().value);
            }
            None
        }
    }


    pub fn debug(&self) {
        unsafe {
            let mut ptr = self.header;
            loop {
                if let Some(h) = ptr {
                    println!("{}", h.as_ref().value);
                    ptr = ptr.unwrap().as_ref().next;
                } else {
                    break;
                }
            }
        }
    }
}


struct Node<T> {
    pub prev: Option<NonNull<Node<T>>>,
    pub next: Option<NonNull<Node<T>>>,
    pub value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            prev: None,
            next: None,
            value,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::LinkedList;
    use std::fmt::Formatter;
    use std::cmp::Ordering;

    #[test]
    fn test_linkedlist() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.pop_back();
    }

    #[test]
    fn test_orderedlist() {
        let mut list: OrderedLinkedList<i32> = OrderedLinkedList::new();
        list.push(2);
        list.push(60);
        list.push(4);
        list.push(8);
        list.push(243);
        list.push(123);
        list.debug();

        let v = list.pop();
        assert_eq!(v.unwrap(), 243);

        let v = list.pop();
        assert_eq!(v.unwrap(), 123);

        let v = list.peek();
        assert_eq!(v.unwrap(), &60);

        list.debug();
    }


    #[test]
    fn test_orderlinkedlist_user() {
        let mut list = OrderedLinkedList::new();
        let user = User::new(2);
        list.push(user);
        let user = User::new(60);
        list.push(user);
        let user = User::new(4);
        list.push(user);
        let user = User::new(8);
        list.push(user);
        let user = User::new(243);
        list.push(user);
        let user = User::new(123);
        list.push(user);


        let u = list.pop();
        if let Some(u) = u {
            println!("user id : {}", u.id);
        }
        let u = list.pop();
        if let Some(u) = u {
            println!("user id : {}", u.id);
        }
        let u = list.pop();
        if let Some(u) = u {
            println!("user id : {}", u.id);
        }
    }

    struct User {
        pub id: i32,
    }


    impl User {
        pub fn new(id: i32) -> Self {
            User {
                id,
            }
        }
    }

    impl Display for User {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "user id :{}", self.id)
        }
    }

    impl Drop for User {
        fn drop(&mut self) {
            println!("user:{} object destruct", self.id);
        }
    }

    impl PartialOrd for User {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.id.partial_cmp(&other.id)
        }
    }

    impl PartialEq for User {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }
}

pub mod array {

    pub fn run() {
        let mut a: Vec<u16> = vec![0; 3];
        a[0] = 1;
        a[1] = 2;
        a[2] = 3;
        println!("{:?}", a);
    }
}

pub mod linkedlist {
    use std::collections::{HashSet, LinkedList};

    pub fn run() {
        let mut list: LinkedList<i16> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_front(6);
        println!("{:?}", list);
    }
}

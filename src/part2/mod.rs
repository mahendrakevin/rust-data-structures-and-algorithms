#[allow(dead_code, unused_imports)]
pub mod tree {
    use serde::Serialize;
    use std::cmp::{max, min};
    use std::fmt;
    use std::path::Component::ParentDir;

    #[derive(Debug, Clone, Serialize)]
    struct Node {
        value: i32,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    }

    impl Node {
        fn new(value: i32) -> Node {
            Node {
                value,
                left: None,
                right: None,
            }
        }
    }

    #[derive(Debug, Clone, Serialize)]
    struct Tree {
        root: Option<Box<Node>>,
    }

    impl Tree {
        fn new() -> Tree {
            Tree { root: None }
        }

        fn insert(&mut self, value: i32) {
            if self.root.is_none() {
                self.root = Some(Box::new(Node::new(value)));
                return;
            }
            let mut current = &mut self.root;
            loop {
                if let Some(ref mut cur) = current {
                    if value < cur.value {
                        if cur.left.is_none() {
                            cur.left = Some(Box::new(Node::new(value)));
                            break;
                        }
                        current = &mut cur.left;
                    } else {
                        if cur.right.is_none() {
                            cur.right = Some(Box::new(Node::new(value)));
                            break;
                        }
                        current = &mut cur.right;
                    }
                }
            }
        }

        fn find(&mut self, value: i32) -> bool {
            let mut current = &mut self.root;
            while !current.is_none() {
                if let Some(ref mut cur) = current {
                    if value < cur.value {
                        current = &mut cur.left;
                    } else if value > cur.value {
                        current = &mut cur.right;
                    } else {
                        return true;
                    }
                }
            }
            false
        }

        fn traverse_pre_order(&self) {
            if let Some(ref root) = self.root {
                self.pre_order(root);
            }
        }

        fn pre_order(&self, root: &Box<Node>) {
            println!("{}", root.value);
            if let Some(ref left) = root.left {
                self.pre_order(left);
            }
            if let Some(ref right) = root.right {
                self.pre_order(right);
            }
        }

        fn traverse_in_order(&self) {
            if let Some(ref root) = self.root {
                self.in_order(root);
            }
        }

        fn in_order(&self, root: &Box<Node>) {
            if let Some(ref right) = root.right {
                self.pre_order(right);
            }
            println!("{}", root.value);
            if let Some(ref left) = root.left {
                self.pre_order(left);
            }
        }

        fn count_height(&self) -> i32 {
            self.height(&self.root)
        }

        fn height(&self, root: &Option<Box<Node>>) -> i32 {
            if let Some(ref node) = root {
                1 + max(self.height(&node.left), self.height(&node.right))
            } else {
                -1
            }
        }

        fn find_min_value(&self) -> i32 {
            let mut current = self.root.to_owned();
            let mut last = current.clone();
            while !current.is_none() {
                last = current.clone();
                current = current.clone().and_then(|node| node.left);
            }
            last.clone()
                .and_then(|node| Some(node.value))
                .unwrap_or(i32::MAX)
        }

        fn min_value(&self, root: Option<Box<Node>>) -> i32 {
            if let Some(ref node) = root {
                let left = self.min_value(node.left.clone());
                let right = self.min_value(node.right.clone());
                min(min(left, right), node.value)
            } else {
                root.and_then(|node| Some(node.value)).unwrap_or(i32::MAX)
            }
        }

        fn equals(&self, other: &Tree) -> bool {
            self.equals_helper(self.root.clone(), other.root.clone())
        }

        fn equals_helper(&self, first: Option<Box<Node>>, second: Option<Box<Node>>) -> bool {
            match first {
                Some(ref node) => match second {
                    Some(ref other) => {
                        node.value == other.value
                            && self.equals_helper(node.left.clone(), other.left.clone())
                            && self.equals_helper(node.right.clone(), other.right.clone())
                    }
                    None => false,
                },
                None => match second {
                    None => true,
                    _ => false,
                },
            }
        }

        fn is_binary_search_tree(&self) -> bool {
            self.is_binary_search_tree_helper(self.root.clone(), i32::MIN, i32::MAX)
        }

        fn is_binary_search_tree_helper(
            &self,
            root: Option<Box<Node>>,
            min: i32,
            max: i32,
        ) -> bool {
            match root {
                Some(ref node) => {
                    if node.value < min || node.value > max {
                        return false;
                    }
                    return self.is_binary_search_tree_helper(
                        node.left.clone(),
                        min,
                        node.value - 1,
                    ) && self.is_binary_search_tree_helper(
                        node.right.clone(),
                        node.value + 1,
                        max,
                    );
                }
                None => return true,
            }
        }

        fn swap_root(&mut self) {
            match self.root {
                Some(ref mut node) => {
                    let temp = node.right.clone();
                    node.right = node.left.clone();
                    node.left = temp
                }
                _ => {}
            }
        }

        fn print_node_at_distance(&self, distance: i32) -> Vec<i32> {
            let mut list: Vec<i32> = vec![];
            self._print_node_at_distance(self.root.clone(), distance, &mut list);
            list
        }

        fn _print_node_at_distance(
            &self,
            root: Option<Box<Node>>,
            distance: i32,
            list: &mut Vec<i32>,
        ) {
            match root {
                Some(node) => {
                    if distance == 0 {
                        list.push(node.value);
                        // println!("{}", node.value);
                    }

                    self._print_node_at_distance(node.left, distance - 1, list);
                    self._print_node_at_distance(node.right, distance - 1, list);
                }
                None => return,
            }
        }

        fn traverse_level_order(&self) {
            for i in 0..=self.count_height() {
                for node in self.print_node_at_distance(i) {
                    println!("{}", node);
                }
            }
        }

        fn size(&self) -> i32 {
            self._size(self.root.clone())
        }

        fn _size(&self, root: Option<Box<Node>>) -> i32 {
            match root {
                Some(node) => 1 + self._size(node.left) + self._size(node.right),
                None => 0,
            }
        }

        fn count_leaves(&self) -> i32 {
            self._count_leaves(self.root.clone())
        }

        fn _count_leaves(&self, root: Option<Box<Node>>) -> i32 {
            match root {
                Some(node) => {
                    if node.left.is_none() && node.right.is_none() {
                        return 1;
                    }
                    self._count_leaves(node.left.clone()) + self._count_leaves(node.right.clone())
                }
                None => 0,
            }
        }

        fn find_max(&self) -> i32 {
            self._find_max(self.root.clone())
        }

        fn _find_max(&self, root: Option<Box<Node>>) -> i32 {
            match root {
                Some(root) => {
                    let left = self._find_max(root.left);
                    let right = self._find_max(root.right);
                    max(max(left, right), root.value)
                },
                None => i32::MIN
            }
        }

        fn to_json(&self) -> String {
            serde_json::to_string_pretty(&self).unwrap()
        }
    }

    fn factorial(number: i32) -> i32 {
        if number == 0 {
            return 1;
        }
        number * factorial(number - 1)
    }

    pub fn run() {
        let mut tree = Tree::new();
        tree.insert(7);
        tree.insert(4);
        tree.insert(6);
        // tree.insert(1);
        tree.insert(9);
        tree.insert(10);
        tree.insert(8);

        let mut tree2 = Tree::new();
        tree2.insert(7);
        tree2.insert(4);
        tree2.insert(6);
        // tree2.insert(1);
        tree2.insert(9);
        tree2.insert(10);
        tree2.insert(8);
        // println!("{}", tree.to_json());
        // println!("{:?}", factorial(4));
        // println!("{:?}", tree.traverse_in_order());
        // println!("{:?}", tree.count_height());
        // println!("{:?}", tree.find_min_value());
        // println!("{:?}", tree.equals(&tree2));
        // tree.swap_root();
        // println!("{}", tree.to_json());
        // println!("{:?}", tree.is_binary_search_tree());
        // let result = tree.print_node_at_distance(2);
        // println!("{:?}", result);
        // println!("{:?}", tree.traverse_level_order());
        // println!("{:?}", tree.size());
        // println!("{:?}", tree.count_leaves());
        println!("{:?}", tree.find_max());
    }
}

pub mod avl {
    use std::cmp::max;
    use serde::Serialize;

    #[derive(Debug, Clone, Serialize)]
    struct AvlNode {
        height: i32,
        value: i32,
        left: Option<Box<AvlNode>>,
        right: Option<Box<AvlNode>>
    }

    #[derive(Debug, Serialize)]
    struct AvlTree {
        root: Option<Box<AvlNode>>
    }

    impl AvlNode {
        pub fn new(value: i32) -> Self {
            Self {
                height: 0,
                value,
                left: None,
                right: None
            }
        }
    }

    impl AvlTree {
        pub fn new() -> Self {
            Self {
                root: None
            }
        }

        fn insert(&mut self, value: i32) {
            let root = self.root.take();
            self.root = self._insert(root, value)
        }

        fn _insert(&mut self, mut root: Option<Box<AvlNode>>, value: i32) -> Option<Box<AvlNode>> {
            match root.clone() {
                Some(mut root) => {
                    if value < root.value {
                        root.left = self._insert(root.left, value);
                    } else {
                        root.right = self._insert(root.right, value);
                    }

                    root.height = max(self.height(&root.left), self.height(&root.right)) + 1;

                    root = self.balance(&mut Some(root.clone())).unwrap_or(root.clone());
                    return Some(root);
                }
                None => {
                    Some(Box::new(AvlNode::new(value)))
                }
            }
        }

        fn height(&self, node: &Option<Box<AvlNode>>) -> i32 {
            match node {
                Some(n) => n.height,
                None => -1
            }
        }


        fn balance_factor(&self, node: &Box<AvlNode>) -> i32 {
            self.height(&node.left) - self.height(&node.right)
        }
        fn is_left_heavy(&self, node: &mut Box<AvlNode>) -> bool {
            self.balance_factor(&node) > 1
        }
        fn is_right_heavy(&self, node: &mut Box<AvlNode>) -> bool {
            self.balance_factor(&node) < -1
        }

        fn balance(&mut self, root: &mut Option<Box<AvlNode>>) -> Option<Box<AvlNode>> {
            if let Some(ref mut node) = root {
                let is_left_heavy = self.is_left_heavy(node);
                let is_right_heavy = self.is_right_heavy(node);

                if is_left_heavy {
                    match node.left {
                        Some(ref mut left) => {
                            if self.balance_factor(left) < 0 {
                                node.left = self.rotate_left(&mut node.left);
                            }
                            return self.rotate_right(root)
                        }
                        None => return None
                    }
                } else if is_right_heavy {
                    match node.right {
                        Some(ref mut right) => {
                            if self.balance_factor(right) > 0 {
                                node.right = self.rotate_right(&mut node.right);
                            }
                            return self.rotate_left(root)
                        }
                        None => return None
                    }
                }
            }
            return None
        }

        fn rotate_left(&mut self, node: &mut Option<Box<AvlNode>>) -> Option<Box<AvlNode>> {
            // new_root = node.right
            // node.right = new_root.left
            // new_root.left = node
            if let Some(ref mut n) = node {
                let mut new_root = n.right.take();
                if let Some(ref mut new_root_node) = new_root {
                    n.right = new_root_node.left.take();
                    new_root_node.left = node.take();
                    self.set_height(node);
                    self.set_height(&mut new_root);
                    return new_root;
                }
            }
            None
        }

        fn rotate_right(&mut self, node: &mut Option<Box<AvlNode>>) -> Option<Box<AvlNode>> {
            // new_root = node.left
            // node.left = new_root.right
            // new_root.right = node
            if let Some(ref mut n) = node {
                let mut new_root = n.left.take();
                if let Some(ref mut new_root_node) = new_root {
                    n.left = new_root_node.right.take();
                    new_root_node.right = node.take();
                    self.set_height(node);
                    self.set_height(&mut new_root);
                    return new_root;
                }
            }
            None
        }

        fn set_height(&self, node: &mut Option<Box<AvlNode>>) {
            if let Some(ref mut n) = node {
                n.height = max(self.height(&n.left), self.height(&n.right)) + 1;
            }
        }

        fn to_json(&self) -> String {
            serde_json::to_string_pretty(&self).unwrap()
        }
    }

    pub fn run() {
        let mut avl = AvlTree::new();
        avl.insert(10);
        avl.insert(30);
        avl.insert(20);
        avl.insert(15);
        println!("{}", avl.to_json())
    }
}

pub mod heaps {
    #[derive(Debug)]
    struct Heap {
        items: Vec<i32>,
        size: usize
    }

    impl Heap {
        pub fn new(length: usize) -> Self {
            Self {
                items: vec![0; length],
                size: 0
            }
        }

        pub fn insert(&mut self, value: i32) {
            if self.is_full() {
                return;
            }

            self.items[self.size] = value;
            self.size += 1;
            self.bubble_up();
        }

        fn is_full(&self) -> bool {
            self.size == self.items.len()
        }

        fn bubble_up(&mut self) {
            let mut index  = self.size - 1;
            while index > 0 && self.items[index] > self.items[self.parent(index)] {
                println!("index of {}: {}, parent index of {}: {}", index, self.items[index], self.parent(index), self.items[self.parent(index)]);
                self.swap(index, self.parent(index));
                index = self.parent(index);
            }
        }

        fn swap(&mut self, first: usize, second: usize) {
            let temp = self.items[first];
            self.items[first] = self.items[second];
            self.items[second] = temp;
        }

        fn parent(&self, index: usize) -> usize {
            (index - 1) / 2
        }

    }

    pub fn run () {
        let mut heaps = Heap::new(10);
        heaps.insert(10);
        heaps.insert(5);
        heaps.insert(17);
        heaps.insert(4);
        heaps.insert(22);
        println!("{:?}", heaps)
    }
}
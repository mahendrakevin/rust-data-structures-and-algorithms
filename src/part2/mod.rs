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
                }
                None => i32::MIN,
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
    use serde::Serialize;
    use std::cmp::max;

    #[derive(Debug, Clone, Serialize)]
    struct AvlNode {
        height: i32,
        value: i32,
        left: Option<Box<AvlNode>>,
        right: Option<Box<AvlNode>>,
    }

    #[derive(Debug, Serialize)]
    struct AvlTree {
        root: Option<Box<AvlNode>>,
    }

    impl AvlNode {
        pub fn new(value: i32) -> Self {
            Self {
                height: 0,
                value,
                left: None,
                right: None,
            }
        }
    }

    impl AvlTree {
        pub fn new() -> Self {
            Self { root: None }
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

                    root = self
                        .balance(&mut Some(root.clone()))
                        .unwrap_or(root.clone());
                    return Some(root);
                }
                None => Some(Box::new(AvlNode::new(value))),
            }
        }

        fn height(&self, node: &Option<Box<AvlNode>>) -> i32 {
            match node {
                Some(n) => n.height,
                None => -1,
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
                            return self.rotate_right(root);
                        }
                        None => return None,
                    }
                } else if is_right_heavy {
                    match node.right {
                        Some(ref mut right) => {
                            if self.balance_factor(right) > 0 {
                                node.right = self.rotate_right(&mut node.right);
                            }
                            return self.rotate_left(root);
                        }
                        None => return None,
                    }
                }
            }
            return None;
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
        size: usize,
    }

    impl Heap {
        pub fn new(length: usize) -> Self {
            Self {
                items: vec![0; length],
                size: 0,
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

        fn remove(&mut self) -> i32 {
            if self.is_empty() {
                return -1;
            }

            let root = self.items[0]; // 2
            self.items[0] = self.items[self.size - 1]; // 5
            self.size -= 1; // 4 -> [5, 17, 10, 4, 5]
            self.bubble_down();
            root
        }

        fn bubble_down(&mut self) {
            let mut index = 0;
            while index <= self.size && !self.is_valid_parent(index) {
                let larger_child_index = self.largest_child_index(index);

                self.swap(index, larger_child_index);
                index = larger_child_index;
            }
        }

        fn largest_child_index(&self, index: usize) -> usize {
            if !self.has_left_child(index) {
                return index;
            }

            if !self.has_right_child(index) {
                return self.left_child_index(index);
            }

            if self.left_child(index) > self.right_child(index) {
                self.left_child_index(index)
            } else {
                self.right_child_index(index)
            }
        }

        fn has_left_child(&self, index: usize) -> bool {
            self.left_child_index(index) <= self.size
        }

        fn has_right_child(&self, index: usize) -> bool {
            self.right_child_index(index) <= self.size
        }

        fn is_valid_parent(&self, index: usize) -> bool {
            if !self.has_left_child(index) {
                return true;
            }

            let is_valid = self.items[index] >= self.left_child(index);
            if self.has_right_child(index) {
                is_valid && self.items[index] >= self.right_child(index)
            } else {
                is_valid
            }
        }

        fn left_child(&self, index: usize) -> i32 {
            self.items[self.left_child_index(index)]
        }

        fn right_child(&self, index: usize) -> i32 {
            self.items[self.right_child_index(index)]
        }

        fn left_child_index(&self, index: usize) -> usize {
            index * 2 + 1
        }

        fn right_child_index(&self, index: usize) -> usize {
            index * 2 + 2
        }

        fn is_full(&self) -> bool {
            self.size == self.items.len()
        }

        fn is_empty(&self) -> bool {
            self.size == 0
        }

        fn bubble_up(&mut self) {
            let mut index = self.size - 1;
            while index > 0 && self.items[index] > self.items[self.parent(index)] {
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

        fn last_parent(&self) -> usize {
            self.size / 2 - 1
        }

        pub fn max(&self) -> i32 {
            if self.is_empty() {
                return -1;
            }
            self.items[0]
        }
    }

    struct PriorityQueueWithHeap {
        heap: Heap,
    }

    impl PriorityQueueWithHeap {
        pub fn new(length: usize) -> Self {
            Self {
                heap: Heap::new(length),
            }
        }

        fn enqueue(&mut self, item: i32) {
            self.heap.insert(item);
        }

        fn dequeue(&mut self) -> i32 {
            self.heap.remove()
        }

        fn is_empty(&self) -> bool {
            self.heap.is_empty()
        }
    }

    fn heapify(numbers: &mut Vec<i32>) {
        let len = numbers.len();
        let last_parent_index = len / 2 - 1;
        for i in (0..last_parent_index).rev() {
            _heapify(numbers, i);
        }
        println!("{:?}", numbers)
    }

    fn _heapify(numbers: &mut Vec<i32>, index: usize) {
        let mut larger_index = index;
        let left_index = index * 2 + 1;
        if left_index < numbers.len() && numbers[left_index] > numbers[larger_index] {
            larger_index = left_index;
        }

        let right_index = index * 2 + 2;
        if right_index < numbers.len() && numbers[right_index] > numbers[larger_index] {
            larger_index = right_index;
        }

        if index == larger_index {
            return;
        }
        numbers.swap(index, larger_index);
        _heapify(numbers, larger_index);
    }

    fn get_kth_largest(numbers: &Vec<i32>, k: i32) -> i32 {
        if k < 1 || k > numbers.len() as i32 {
            return -1;
        }
        let mut heap = Heap::new(numbers.len());
        numbers.iter().for_each(|&number| heap.insert(number));
        for _ in 0..k - 1 {
            heap.remove();
        }
        println!("{:?}", heap);
        heap.max()
    }

    pub fn run() {
        let mut heaps = Heap::new(10);
        heaps.insert(10);
        heaps.insert(5);
        heaps.insert(17);
        heaps.insert(4);
        heaps.insert(22);
        heaps.remove();
        println!("{:?}", heaps);
        // let mut numbers = vec![5, 3, 10, 1, 4, 2];
        // let mut heap2 = Heap::new(numbers.len());
        // numbers.iter().for_each(|&number| heap2.insert(number));

        // while !heap2.is_empty() {
        //     println!("{:?}", heap2.remove());
        // }
        //
        // for i in (0..numbers.len()) {
        //     numbers[i] = heap2.remove();
        // }
        // println!("{:?}", numbers);

        // Heapify
        let mut numbers = vec![5, 3, 8, 4, 1, 2];
        // println!("{:?}", heapify(&mut numbers));

        println!("{:?}", get_kth_largest(&numbers, 5));
    }
}

pub mod tries {
    use serde::Serialize;
    use std::collections::HashMap;

    const ALPHABET_SIZE: usize = 26;
    #[derive(Debug, Serialize)]
    struct TrieNode {
        children: HashMap<char, TrieNode>,
        is_end_of_word: bool,
    }

    impl TrieNode {
        pub fn new() -> Self {
            Self {
                children: HashMap::new(),
                is_end_of_word: false,
            }
        }

        pub fn insert(&mut self, value: &str) {
            let mut current = self;
            for c in value.chars() {
                current = current.children.entry(c).or_insert_with(TrieNode::new);
            }

            current.is_end_of_word = true;
        }

        fn has_children(&self, c: &char) -> bool {
            self.children.contains_key(&c)
        }

        fn get_children(&self, c: &char) -> Option<&TrieNode> {
            self.children.get(c)
        }

        pub fn contains(&self, value: &str) -> bool {
            let mut current = self;
            for c in value.chars() {
                if !current.has_children(&c) {
                    return false;
                }
                current = current.get_children(&c).unwrap();
            }
            current.is_end_of_word
        }

        pub fn traverse(&self) {
            self.traverse_(' ')
        }
        fn traverse_(&self, c: char) {
            if self.is_end_of_word {
                println!("End of word");
            }

            for (key, value) in &self.children {
                println!("{}", key);
                value.traverse_(*key);
            }
        }

        pub fn remove(&mut self, value: String) {
            self.remove_(value, 0);
        }

        fn remove_(&mut self, value: String, index: usize) {
            if index == value.len() {
                self.is_end_of_word = false;
                return;
            }
            let c = value.chars().nth(index).unwrap();
            if let Some(child) = self.children.get_mut(&c) {
                child.remove_(value.clone(), index + 1);
                if child.children.is_empty() && !child.is_end_of_word {
                    self.children.remove(&c);
                }
            }
        }

        pub fn find_words(&mut self, prefix: String) -> Vec<String> {
            let mut words: Vec<String> = Vec::new();
            if let Some(node) = self.find_last_node_of(&prefix) {
                println!("{}", node.to_json());
                node.find_words_(&prefix, &mut words);
            }
            words
        }

        fn find_words_(&self, prefix: &String, words: &mut Vec<String>) {
            if self.is_end_of_word {
                words.push(prefix.clone());
            }

            for (c, child) in &self.children {
                let mut new_prefix = prefix.clone();
                new_prefix.push(c.clone());
                child.find_words_(&new_prefix, words);
            }
        }

        fn find_last_node_of(&self, prefix: &String) -> Option<&TrieNode> {
            let mut current = self;
            for ch in prefix.chars() {
                let child = current.children.get(&ch);
                match child {
                    Some(child) => {
                        current = child;
                    }
                    None => return None,
                }
            }
            Some(current)
        }

        pub fn count_words(&self) -> i32 {
            let mut count = 0;
            self.count_words_(&mut count);
            count
        }

        fn count_words_(&self, count: &mut i32) {
            let mut current = self;
            for (_, child) in &current.children {
                if child.is_end_of_word {
                    *count += 1;
                }
                child.count_words_(count);
                current = child;
            }
        }

        pub fn longest_common_prefix(&mut self, words: Vec<String>) -> String {
            if words.is_empty() {
                return String::new();
            }

            let mut trie = TrieNode::new();
            for word in &words {
                trie.insert(word);
            }

            let mut prefix = String::new();
            let mut current = &trie;

            while current.children.len() == 1 && !current.is_end_of_word {
                let (c, next_node) = current.children.iter().next().unwrap();
                prefix.push(*c);
                current = next_node;
            }

            match self.contains(prefix.as_str()) {
                true => prefix,
                false => String::new(),
            }
        }

        fn to_json(&self) -> String {
            serde_json::to_string_pretty(&self).unwrap()
        }
    }
    pub fn run() {
        // let mut trie = TrieNode::new();
        // trie.insert("car");
        // trie.insert("care");
        // trie.remove("care".to_string());
        // // trie.traverse();
        // println!("{}", trie.to_json());
        let mut trie = TrieNode::new();
        trie.insert("car");
        trie.insert("careful");
        trie.insert("card");
        trie.insert("care");
        trie.insert("egg");
        trie.find_words("egg".to_string());
        // println!("{:?}", trie.find_words("e".to_string()));
        // println!("{:?}", trie.count_words());
        println!(
            "{:?}",
            trie.longest_common_prefix(vec!["god".to_string(), "go".to_string()])
        );
    }
}

pub mod graphs {
    use std::collections::{HashMap, HashSet};

    macro_rules! unwrap_or_return {
        ($option:expr) => {
            match $option {
                Some(value) => value,
                None => return,
            }
        };
    }

    #[derive(Debug)]
    pub struct Stack<T> {
        elements: Vec<T>,
    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Self {
                elements: Vec::new(),
            }
        }

        pub fn push(&mut self, item: T) {
            self.elements.push(item);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.elements.pop()
        }

        pub fn peek(&self) -> Option<&T> {
            self.elements.last()
        }

        pub fn is_empty(&self) -> bool {
            self.elements.is_empty()
        }
    }

    #[derive(Debug, Eq, Hash, PartialEq, Clone)]
    struct GraphNode {
        label: String,
    }
    impl GraphNode {
        pub fn new(label: String) -> Self {
            Self { label }
        }
    }

    #[derive(Debug)]
    struct Graph {
        node: HashMap<String, GraphNode>,
        adjacency_list: HashMap<String, Vec<GraphNode>>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                node: HashMap::new(),
                adjacency_list: HashMap::new(),
            }
        }

        pub fn add_node(&mut self, label: String) {
            let node = GraphNode::new(label.to_owned());
            self.node.entry(label.to_owned()).or_insert(node);
            self.adjacency_list
                .entry(label.to_owned())
                .or_insert(Vec::new());
        }

        pub fn remove_node(&mut self, label: String) {
            let node = unwrap_or_return!(self.node.get(&label).cloned());
            let keys = self.adjacency_list.keys().cloned().collect::<Vec<String>>();
            for key in keys {
                self.adjacency_list.get_mut(&key).and_then(|list| {
                    let index = list.iter().position(|n| n.eq(&node));
                    match index {
                        Some(index) => {
                            list.remove(index);
                            Some(())
                        }
                        None => None,
                    }
                });
                self.adjacency_list.remove(&label);
                self.node.remove(&label);
            }
        }

        pub fn add_edge(&mut self, from: String, to: String) {
            let to_node = unwrap_or_return!(self.node.get(&to).cloned());
            self.adjacency_list.get_mut(&from).and_then(|list| {
                list.push(to_node);
                Some(())
            });
        }

        pub fn remove_edge(&mut self, from: String, to: String) {
            let to_node = unwrap_or_return!(self.node.get(&to).cloned());
            self.adjacency_list.get_mut(&from).and_then(|x| {
                match x.iter().position(|n| n.eq(&to_node)) {
                    Some(index) => {
                        x.remove(index);
                        Some(())
                    }
                    None => None,
                }
            });
        }

        pub fn depth_first_traversal_recursive(&self, root: String) {
            let mut visited: HashSet<GraphNode> = HashSet::new();
            let root = unwrap_or_return!(self.node.get(&root).cloned());
            self.depth_first_traversal_recursive_(root, &mut visited);
        }

        fn depth_first_traversal_recursive_(
            &self,
            root: GraphNode,
            visited: &mut HashSet<GraphNode>,
        ) {
            println!("{:?}", root);
            visited.insert(root.clone());

            for node in self.adjacency_list.get(&root.label).cloned().unwrap() {
                if !visited.contains(&node) {
                    self.depth_first_traversal_recursive_(node, visited);
                }
            }
        }

        pub fn depth_first_traversal_iterative(&mut self, root: String) {
            let node = unwrap_or_return!(self.node.get(&root).cloned());
            let mut visited: HashSet<GraphNode> = HashSet::new();
            let mut stack: Stack<GraphNode> = Stack::new();
            stack.push(node.clone());
            while !stack.is_empty() {
                let current = unwrap_or_return!(stack.pop());
                if visited.contains(&current) {
                    continue;
                }
                println!("{:?}", current);
                visited.insert(current.clone());
                for neighbour in self.get_adjacent_nodes(&current).unwrap() {
                    if !visited.contains(&neighbour) {
                        stack.push(neighbour);
                    }
                }
            }
        }

        pub fn breadth_first_traversal(&mut self, root: String) {
            let node = unwrap_or_return!(self.node.get(&root).cloned());
            let mut visited: HashSet<GraphNode> = HashSet::new();
            let mut queue: Vec<GraphNode> = Vec::new();
            queue.push(node.clone());
            while !queue.is_empty() {
                let current = queue.remove(0);
                if visited.contains(&current) {
                    continue;
                }
                println!("{:?}", current);
                visited.insert(current.clone());
                for neighbour in self.get_adjacent_nodes(&current).unwrap() {
                    if !visited.contains(&neighbour) {
                        queue.push(neighbour);
                    }
                }
            }
        }

        fn get_adjacent_nodes(&self, node: &GraphNode) -> Option<Vec<GraphNode>> {
            self.adjacency_list.get(&node.label).cloned()
        }

        pub fn topological_sort(&mut self) -> Vec<String> {
            let mut visited: HashSet<GraphNode> = HashSet::new();
            let mut stack: Stack<GraphNode> = Stack::new();
            let current = self.node.clone();
            for node in current.values() {
                self.topological_sort_(node.clone(), &mut visited, &mut stack);
            }

            let mut sorted: Vec<String> = Vec::new();
            // println!("{:?}", stack);

            while !stack.is_empty() {
                sorted.push(stack.pop().unwrap().label)
            }

            sorted
        }

        fn topological_sort_(&mut self, node: GraphNode, visited: &mut HashSet<GraphNode>, stack: &mut Stack<GraphNode>) {
            if visited.contains(&node) {
                return;
            }

            // [P, B, X, A]
            visited.insert(node.clone());
            for neighbour in self.get_adjacent_nodes(&node).clone().unwrap() {
                // println!("neighbour {:?}", neighbour);
                // P
                self.topological_sort_(neighbour, visited, stack);
            }

            // [P, B, A, X]
            stack.push(node);
            // println!("stack result {:?}", stack);
        }

        pub fn has_cycle(&mut self) -> bool {
            let mut all: HashSet<GraphNode> = HashSet::new();
            self.node.values().for_each(|node| {
                all.insert(node.clone());
            });

            let mut visiting: HashSet<GraphNode> = HashSet::new();
            let mut visited: HashSet<GraphNode> = HashSet::new();
            println!("{:?}", all);

            while !all.is_empty() {
                let current = all.iter().next().unwrap().clone();
                if self.has_cycle_(current, &mut all, &mut visiting, &mut visited) {
                    return true
                }
            }
            false
        }

        fn has_cycle_(&self, root: GraphNode, all: &mut HashSet<GraphNode>, visiting: &mut HashSet<GraphNode>, visited: &mut HashSet<GraphNode>) -> bool {
            all.remove(&root);
            visiting.insert(root.clone());

            for neighbour in self.get_adjacent_nodes(&root).unwrap() {
                println!("neighbour {:?}", neighbour);
                if visited.contains(&neighbour) {
                    continue
                }

                if visiting.contains(&neighbour) {
                    return true
                }

                if self.has_cycle_(neighbour, all, visiting, visited) {
                    return true
                }
            }

            visiting.remove(&root);
            visited.insert(root.clone());
            return false
        }

        pub fn print(&mut self) {
            for (key, value) in &self.adjacency_list {
                let targets = self.adjacency_list.get(key).unwrap();
                if !targets.is_empty() {
                    let label = targets
                        .iter()
                        .map(|node| node.label.clone())
                        .collect::<Vec<String>>();
                    println!("{} is connected to {:?}", key, label);
                }
            }
        }
    }

    pub fn run() {
        // let mut graph = Graph::new();
        // graph.add_node("kevin".to_string());
        // graph.add_node("vina".to_string());
        // graph.add_node("becca".to_string());
        // graph.add_edge("kevin".to_string(), "vina".to_string());
        // graph.add_edge("kevin".to_string(), "becca".to_string());

        // graph.remove_edge("kevin".to_string(), "d".to_string());
        // graph.remove_node("kevin".to_string());
        // graph.add_edge("vina".to_string(), "becca".to_string());
        // graph.print();
        // println!("{:?}", graph);

        // let mut graph = Graph::new();
        // graph.add_node("A".to_string());
        // graph.add_node("B".to_string());
        // graph.add_node("C".to_string());
        // graph.add_node("D".to_string());
        // graph.add_edge("A".to_string(), "B".to_string());
        // graph.add_edge("B".to_string(), "D".to_string());
        // graph.add_edge("D".to_string(), "C".to_string());
        // graph.add_edge("A".to_string(), "C".to_string());
        // graph.print();
        // graph.breadth_first_traversal("D".to_string());

        // let mut graph = Graph::new();
        // graph.add_node("X".to_string());
        // graph.add_node("P".to_string());
        // graph.add_node("A".to_string());
        // graph.add_node("B".to_string());
        // graph.add_edge("X".to_string(), "A".to_string());
        // graph.add_edge("X".to_string(), "B".to_string());
        // graph.add_edge("A".to_string(), "P".to_string());
        // graph.add_edge("B".to_string(), "P".to_string());
        // println!("{:?}", graph.topological_sort());
        // graph.print();

        let mut graph = Graph::new();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_node("C".to_string());
        graph.add_edge("A".to_string(), "B".to_string());
        graph.add_edge("B".to_string(), "C".to_string());
        graph.add_edge("C".to_string(), "A".to_string());
        println!("{:?}", graph.has_cycle());
    }
}

pub mod ungraphs {
    use std::cmp::{min, Reverse};
    use std::collections::{HashMap, HashSet};
    use crate::part2::graphs;
    use crate::part2::graphs::Stack;

    macro_rules! unwrap_or_return {
        ($option:expr) => {
            match $option {
                Some(value) => value,
                None => return,
            }
        };
    }

    #[derive(Debug)]
    struct Path {
        nodes: Vec<String>,
    }

    impl Path {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
            }
        }

        pub fn add_node(&mut self, node: String) {
            self.nodes.push(node);
        }

        pub fn to_string(&self) -> String {
            self.nodes.join(" -> ")
        }
    }

    #[derive(Debug)]
    struct PriorityQueue<T: Ord> {
        elements: Vec<T>,
    }

    impl<T: Ord> PriorityQueue<T> {
        pub fn new() -> Self {
            Self {
                elements: Vec::new(),
            }
        }

        pub fn enqueue(&mut self, item: T) {
            self.elements.push(item);
            // Sort in ascending order
            self.elements.sort();
        }

        pub fn dequeue(&mut self) -> Option<T> {
            self.elements.pop()
        }

        pub fn is_empty(&self) -> bool {
            self.elements.is_empty()
        }

        pub fn peek(&self) -> Option<&T> {
            self.elements.last()
        }

        pub fn count(&self) -> usize {
            self.elements.len()
        }

        pub fn get_all(&self) -> &Vec<T> {
            &self.elements
        }
    }

    #[derive(Debug, Eq, Hash, PartialEq, Clone, Ord, PartialOrd)]
    struct GraphNode {
        label: String,
        edges: Vec<Edge>
    }
    impl GraphNode {
        pub fn new(label: String) -> Self {
            Self { label, edges: Vec::new() }
        }

        pub fn add_edge(&mut self, to: GraphNode, weight: i32) {
            self.edges.push(Edge::new(self.clone(), to, weight));
        }

        pub fn get_edges(&self) -> Vec<Edge> {
            self.edges.clone()
        }
    }

    #[derive(Debug, Eq, Hash, PartialEq, Clone, Ord, PartialOrd)]
    struct Edge {
        from: GraphNode,
        to: GraphNode,
        weight: i32
    }

    impl Edge {
        pub fn new(from: GraphNode, to: GraphNode, weight: i32) -> Self {
            Self {
                from,
                to,
                weight
            }
        }

        pub fn to_string(&self) -> String {
            format!("{} -> {} ({})", self.from.label, self.to.label, self.weight)
        }
    }

    #[derive(Debug)]
    struct WeightedGraphs {
        nodes: HashMap<String, GraphNode>
    }

    #[derive(Debug, Hash, Ord, Eq, PartialEq, PartialOrd)]
    struct NodeEntry {
        node: GraphNode,
        priority: i32
    }

    impl NodeEntry {
        pub fn new(node: GraphNode, priority: i32) -> Self {
            Self {
                node,
                priority
            }
        }
    }

    impl WeightedGraphs {
        pub fn new() -> Self {
            Self {
                nodes: HashMap::new()
            }
        }

        pub fn to_string(&self) -> String {
            let mut result = String::new();
            self.nodes.values().into_iter().for_each(
                |node| {
                    let edges = node.get_edges().iter().map(|edge| edge.to_string()).collect::<Vec<String>>().join(", ");
                    result.push_str(&format!("{} -> [{}]\n", node.label, edges));
                }
            );
            result
        }

        pub fn add_node(&mut self, label: String) {
            self.nodes.entry(label.to_owned()).or_insert(GraphNode::new(label));
        }

        pub fn add_edge(&mut self, from: String, to: String, weight: i32) {
            let to_node = unwrap_or_return!(self.nodes.get(&to).cloned());
            let from_node = unwrap_or_return!(self.nodes.get(&from).cloned());
            if let Some(from) = self.nodes.get_mut(&from) {
                from.add_edge(to_node, weight);
            }
            if let Some(to) = self.nodes.get_mut(&to) {
                to.add_edge(from_node, weight);
            }
        }



        pub fn get_shortest_path(&self, from: String, to: String) -> Path {
            let from_node = if let Some(from_node) = self.nodes.get(&from).cloned() {
                from_node
            } else {
                return Path::new()
            };
            let to_node = if let Some(to_node) = self.nodes.get(&to).cloned() {
                to_node
            } else {
                return Path::new()
            };
            println!("from {} to {}", from_node.label, to_node.label);

            let mut distance: HashMap<GraphNode, i32> = HashMap::new();
            for node in self.nodes.values() {
                distance.insert(node.clone(), i32::MAX);
            }
            distance.insert(from_node.clone(), 0);
            distance.iter().for_each(|(key, value)| {
                println!("distance {} -> {}", key.label, value);
            });

            let mut previous_nodes: HashMap<GraphNode, GraphNode> = HashMap::new();
            let mut visited: HashSet<GraphNode> = HashSet::new();
            let mut queue = priority_queue::PriorityQueue::new();
            queue.push(NodeEntry::new(from_node.clone(), 0), Reverse(0));
            println!("Added to queue {}", from_node.label);
            println!();

            while let Some((NodeEntry { node: current, .. }, _)) = queue.pop() {
                visited.insert(current.clone());
                println!("Visited [{}]", visited.iter().map(|x|x.label.clone()).collect::<Vec<String>>().join(", "));

                println!("Processing Node {} == {}", current.label, current.get_edges().iter()
                    .map(|x|x.to_string()).collect::<Vec<String>>().join(", "));
                for edge in current.get_edges() {
                    println!("processing Edge {} > {} with weight {}", edge.from.label,
                             edge.to.label, edge.weight);
                    let neighbor = self.nodes.get(&edge.to.label).cloned().unwrap();
                    if visited.contains(&neighbor) {
                        println!("Already visited {}", neighbor.label);
                        continue;
                    }

                    let new_distance = distance[&current] + edge.weight;
                    println!("New distance for {} is {}", neighbor.label, new_distance);
                    if new_distance < distance[&neighbor] {
                        println!("The new distance is less than the current distance {} < {}", new_distance, distance[&neighbor]);
                        distance.insert(neighbor.clone(), new_distance);
                        previous_nodes.insert(neighbor.clone(), current.clone());
                        println!("Add to queue {} with distance {}", neighbor.label, new_distance);
                        queue.push(NodeEntry::new(neighbor.clone(), new_distance), Reverse(new_distance));
                    }
                    println!();
                }
            }

            previous_nodes.iter().for_each(|(key, value)| {
                println!("{} -> {}", key.label, value.label);
            });

            self.build_path(to_node, previous_nodes)
        }

        fn build_path(&self, to_node: GraphNode, previous_nodes: HashMap<GraphNode, GraphNode>) -> Path {
            let mut path = Path::new();
            let mut current_label = to_node.clone();

            while let Some(prev_label) = previous_nodes.get(&current_label) {
                path.add_node(current_label.label.clone());
                current_label = prev_label.clone();
            }
            path.add_node(current_label.label);
            path.nodes.reverse();
            path
        }

        pub fn get_edges(&self, node: String) {
            let result = unwrap_or_return!(self.nodes.get(&node).cloned());
            println!("Edges for node {}", result.label);
            result.get_edges().iter().for_each(|edge| {
                println!("{}", edge.to_string());
            });
        }

        pub fn has_cycle(&self) -> bool {
            let mut visited: HashSet<String> = HashSet::new();
            for node in self.nodes.values() {
                println!("Checking node {}", node.label);
                if !visited.contains(&node.label) &&
                    self.has_cycle_(node.clone(), &mut visited, &GraphNode::new("".to_string())) {
                    return true
                }
            }
            false
        }

        fn has_cycle_(&self, node: GraphNode, visited: &mut HashSet<String>, parent: &GraphNode) -> bool {
            visited.insert(node.label.clone());
            for edge in node.get_edges() {
                println!("Checking edge {} -> {}", edge.from.label, edge.to.label);
                if edge.to == *parent {
                    continue
                }

                if visited.contains(&edge.to.label) ||
                    self.has_cycle_(edge.to.clone(), visited, &node) {
                    return true
                }
            }
            return false
        }

        pub fn get_minimum_spanning_tree(&self) -> Self {
            let mut tree = WeightedGraphs::new();
            if self.nodes.is_empty() {
                return tree
            }
            let start_node = self.nodes.values().next().unwrap().clone();
            let mut edges = priority_queue::PriorityQueue::new();
            start_node.get_edges().iter().for_each(|edge| {
                edges.push(edge.clone(), Reverse(edge.weight));
            });

            if edges.is_empty() {
                return tree
            }


            tree.add_node(start_node.label.clone());

            while tree.nodes.len() < self.nodes.len() {
                let (min_edge, weight) = edges.pop().unwrap();
                println!("Dequeue edge {}", min_edge.to_string());
                let next_node = &min_edge.to;
                if tree.contains_node(&next_node.label) {
                    continue
                }

                tree.add_node(next_node.label.clone());
                tree.add_edge(min_edge.from.label.clone(), next_node.label.clone(), min_edge.weight);
                next_node.get_edges().iter().for_each(|edge| {
                    if !tree.contains_node(&edge.to.label) {
                        edges.push(edge.clone(), Reverse(edge.weight));
                    }
                });
            }

            tree

        }

        pub fn contains_node(&self, label: &String) -> bool {
            self.nodes.contains_key(label)
        }
    }

    pub fn run () {
        // let mut graph = WeightedGraphs::new();
        // graph.add_node("A".to_string());
        // graph.add_node("B".to_string());
        // graph.add_node("C".to_string());
        // graph.add_edge("A".to_string(), "B".to_string(), 3);
        // graph.add_edge("A".to_string(), "C".to_string(), 2);
        // println!("{}", graph.to_string());

        let mut graph = WeightedGraphs::new();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_node("C".to_string());
        graph.add_edge("A".to_string(), "B".to_string(), 1);
        graph.add_edge("B".to_string(), "C".to_string(), 2);
        graph.add_edge("A".to_string(), "C".to_string(), 10);
        println!("{}", graph.to_string());
        graph.get_edges("B".to_string());
        println!();
        println!("shortest path: {}", graph.get_shortest_path("A".to_string(), "C".to_string()).to_string());

        // let mut graph = WeightedGraphs::new();
        // graph.add_node("A".to_string());
        // graph.add_node("B".to_string());
        // graph.add_node("C".to_string());
        // graph.add_edge("A".to_string(), "B".to_string(), 0);
        // graph.add_edge("B".to_string(), "C".to_string(), 0);
        // graph.add_edge("C".to_string(), "A".to_string(), 0);
        // // graph.add_edge("C".to_string(), "A".to_string(), 10);
        // println!("{}", graph.to_string());
        // println!("{}", graph.has_cycle());

        let mut graph = WeightedGraphs::new();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_node("C".to_string());
        graph.add_node("D".to_string());
        graph.add_edge("A".to_string(), "B".to_string(), 3);
        graph.add_edge("B".to_string(), "D".to_string(), 4);
        graph.add_edge("C".to_string(), "D".to_string(), 5);
        graph.add_edge("A".to_string(), "C".to_string(), 1);
        graph.add_edge("B".to_string(), "C".to_string(), 2);
        // println!("{}", graph.to_string());
        let tree = graph.get_minimum_spanning_tree();
        println!("{}", tree.to_string());
    }
}


use hex;
use crate::merkle::leaf;

pub struct merkle-tree {
    head: Option<Box<leaf::Leaf>>,
    leaf_count: usize
}

impl merkle-tree {
    pub fn new() -> merkle-tree {
        return merkle-tree {
            head: None,
            leaf_count: 0
        };
    }

    pub fn get_rhash(&self) -> Option<[u8; 20]> {
        if self.head.is_none() {
            return None;
        }

        return Some(self.head.as_ref().unwrap().hash);
    }

    pub fn print_tree(&self) -> () {
        fn print_node(node: &leaf::Leaf, depth: usize) {
            let indent = "  ".repeat(depth);
            let hash_hex = hex::encode(&node.hash);

            if let Some(data) = &node.data {
                println!("{}[Leaf] Hash: {}, Data: {}", indent, hash_hex, data);
            } 
            else {
                println!("{}[Node] Hash: {}", indent, hash_hex);
            }

            if let Some(ref left) = node.left {
                print_node(left, depth + 1);
            }
            
            if let Some(ref right) = node.right {
                print_node(right, depth + 1);
            }
        }

        if let Some(ref head) = self.head {
            print_node(head, 0);
        } 
        else {
            println!("Tree is empty.");
        }
    }

    fn merge_leaves(left: Box<leaf::Leaf>, right: Box<leaf::Leaf>) -> Box<leaf::Leaf> {
        let parent: leaf::Leaf = leaf::Leaf::create_from_childrens(left, right);
        return Box::new(parent);
    }

    pub fn insert(&mut self, data: &str) -> () {
        let new_leaf = Box::new(leaf::Leaf::create_from_data(data));
        self.leaf_count += 1;

        if self.head.is_none() {
            self.head = Some(new_leaf);
        } 
        else {
            let old_root = self.head.take().unwrap();
            let combined = merkle-tree::merge_leaves(old_root, new_leaf);
            self.head = Some(combined);
        }
    }

    pub fn search(&self, query: &str) -> Option<[u8; 20]> {
        fn _dfs(node: &leaf::Leaf, query: &str) -> Option<[u8; 20]> {
            if node.is_leaf() {
                if let Some(ref data) = node.data {
                    if data == query {
                        return Some(node.hash);
                    }
                }

                return None;
            }

            if let Some(ref left) = node.left {
                if let Some(h) = _dfs(left, query) {
                    return Some(h);
                }
            }

            if let Some(ref right) = node.right {
                if let Some(h) = _dfs(right, query) {
                    return Some(h);
                }
            }

            return None;
        }

        return self.head.as_ref().and_then(|head| _dfs(&head, query));
    }

    pub fn update(&mut self, old: &str, new: &str) -> bool {
        fn _dfs(node: &mut leaf::Leaf, old: &str, new: &str) -> bool {
            if node.is_leaf() {
                if let Some(ref data) = node.data {
                    if data == old {
                        node.data = Some(new.to_string());
                        node.hash = ripemd160::hash(new.as_bytes());
                        return true;
                    }
                }

                return false;
            }

            let mut changed = false;

            if let Some(ref mut left) = node.left {
                changed |= _dfs(left, old, new);
            }

            if let Some(ref mut right) = node.right {
                changed |= _dfs(right, old, new);
            }

            if changed {
                node.rehash();
            }

            return changed;
        }

        return self.head.as_mut().map_or(false, |h| _dfs(h, old, new));
    }

    pub fn delete(&mut self, value: &str) -> bool {
        fn collect_leaves(node: &leaf::Leaf, acc: &mut Vec<String>) {
            if node.is_leaf() {
                if let Some(ref data) = node.data {
                    acc.push(data.clone());
                }

                return;
            }

            if let Some(ref left) = node.left {
                collect_leaves(left, acc);
            }

            if let Some(ref right) = node.right {
                collect_leaves(right, acc);
            }
        }

        if let Some(ref head) = self.head {
            let mut values = Vec::new();
            collect_leaves(&head, &mut values);

            let before = values.len();
            values.retain(|d| d != value);
            let after = values.len();

            if before == after {
                return false;
            }

            self.head = None;
            self.leaf_count = 0;
            for val in values {
                self.insert(&val);
            }
            return true;
        }

        false
    }
}

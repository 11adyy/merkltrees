use std;
use crate::merkle::leaf;

pub struct merkle-tree {
    leaves: Vec<std::rc::Rc<std::cell::RefCell<leaf::Leaf>>>,
    head: Option<std::rc::Rc<std::cell::RefCell<leaf::Leaf>>>
}

impl merkle-tree {
    pub fn new() -> merkle-tree {
        return merkle-tree {
            leaves: vec![],
            head: None
        };
    }

    pub fn equals(&mut self, tree: &mut merkle-tree) -> bool {
        let self_head = match &self.head {
            Some(head) => head,
            None => return false,
        };

        let tree_head = match &tree.head {
            Some(head) => head,
            None => return false,
        };

        return self_head.borrow().equals(&tree_head.borrow());
    }

    pub fn rebuild(&mut self) -> bool {
        let mut current_level: Vec<std::rc::Rc<std::cell::RefCell<leaf::Leaf>>> = self.leaves.clone();
        while current_level.len() > 1 {
            let mut next_level: Vec<std::rc::Rc<std::cell::RefCell<leaf::Leaf>>> = Vec::new();
            let mut i = 0;
            while i < current_level.len() {
                let left = current_level[i].clone();
                let right = if i + 1 < current_level.len() {
                    current_level[i + 1].clone()
                } 
                else {
                    left.clone()
                };

                let parent = leaf::Leaf::create_from_childrens(left, right);
                next_level.push(parent);
                i += 2;
            }

            current_level = next_level;
        }

        self.head = current_level.pop();
        return true;
    }

    pub fn insert(&mut self, data: &str) -> bool {
        let new_leaf = std::rc::Rc::new(std::cell::RefCell::new(leaf::Leaf::create_from_data(data)));
        self.leaves.push(new_leaf);
        return true;
    }

    pub fn update(&mut self, index: usize, data: &str) -> bool {
        let leaf: std::rc::Rc<std::cell::RefCell<leaf::Leaf>> = match self.leaves.get(index) {
            Some(leaf) => leaf.clone(),
            None => return false,
        };
    
        let new_hash = ripemd160::hash(data.as_bytes());
    
        {
            let mut leaf_borrow = leaf.borrow_mut();
            leaf_borrow.hash = new_hash;
        }
    
        if self.head.is_none() {
            self.rebuild();
            return true;
        }

        let mut current: std::rc::Rc<std::cell::RefCell<leaf::Leaf>> = leaf;
        loop {
            let parent_weak = current.borrow().parent.clone();
            let parent_rc = match parent_weak.and_then(|w| w.upgrade()) {
                Some(rc) => rc,
                None => break,
            };
    
            parent_rc.borrow_mut().rehash();
            current = parent_rc;
        }
    
        return true;
    }    
}

use std;
use digest_primitives;
mod leaf;

pub struct merkle-tree<H: digest_primitives::Hasher> {
    hfunc: H,
    leaves: Vec<std::rc::Rc<std::cell::RefCell<leaf::Leaf>>>,
    head: Option<std::rc::Rc<std::cell::RefCell<leaf::Leaf>>>
}

impl<H: digest_primitives::Hasher> merkle-tree<H> {
    pub fn new() -> merkle-tree<H> {
        return merkle-tree {
            hfunc: H::new(),
            leaves: vec![],
            head: None
        };
    }

    pub fn get(&self, index: usize) -> i128 {
        return self.leaves[index].as_ref().borrow().data;
    }

    pub fn equals(&mut self, tree: &mut merkle-tree<H>) -> bool {
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
        if self.leaves.is_empty() {
            return false;
        }
        
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

                let parent = leaf::Leaf::create_from_childrens(&self.hfunc, left, right);
                next_level.push(parent);
                i += 2;
            }

            current_level = next_level;
        }

        self.head = current_level.pop();
        return true;
    }

    pub fn insert(&mut self, data: i128) -> bool {
        let new_leaf = std::rc::Rc::new(std::cell::RefCell::new(leaf::Leaf::create_from_data(data, &self.hfunc)));
        self.leaves.push(new_leaf);
        return true;
    }

    pub fn update(&mut self, index: usize, data: i128) -> bool {
        let leaf: std::rc::Rc<std::cell::RefCell<leaf::Leaf>> = match self.leaves.get(index) {
            Some(leaf) => leaf.clone(),
            None => return false,
        };
    
        {
            let mut leaf_borrow = leaf.borrow_mut();
            leaf_borrow.data = data;
            leaf_borrow.hash = self.hfunc.hash(&data.to_le_bytes());
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
    
            parent_rc.borrow_mut().rehash(&self.hfunc);
            current = parent_rc;
        }
    
        return true;
    }    

    pub fn delete(&mut self, index: usize) -> bool {
        if index >= self.leaves.len() {
            return false;
        }

        self.leaves.remove(index);

        if self.leaves.is_empty() {
            self.head = None;
            return true;
        }

        // Determine starting group for partial rebuild
        let mut start_group = index / 2;
        let mut current_level = self.leaves.clone();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for group in 0..start_group {
                let left_idx = 2 * group;
                let right_idx = 2 * group + 1;
                
                let left = current_level[left_idx].clone();
                let right = if right_idx < current_level.len() {
                    current_level[right_idx].clone()
                } 
                else {
                    left.clone()
                };

                let parent = if let Some(parent_weak) = &left.borrow().parent {
                    if let Some(parent_rc) = parent_weak.upgrade() {
                        parent_rc.borrow_mut().left = Some(left.clone());
                        parent_rc.borrow_mut().right = Some(right.clone());
                        parent_rc.borrow_mut().rehash(&self.hfunc);
                        parent_rc
                    } 
                    else {
                        leaf::Leaf::create_from_childrens(&self.hfunc, left.clone(), right)
                    }
                } else {
                    leaf::Leaf::create_from_childrens(&self.hfunc, left, right)
                };

                next_level.push(parent);
            }

            let mut i = 2 * start_group;
            while i < current_level.len() {
                let left = current_level[i].clone();
                let right = if i + 1 < current_level.len() {
                    current_level[i + 1].clone()
                } 
                else {
                    left.clone()
                };

                let parent = leaf::Leaf::create_from_childrens(&self.hfunc, left, right);
                next_level.push(parent);
                i += 2;
            }

            start_group /= 2;
            current_level = next_level;
        }

        self.head = current_level.pop();
        return true;
    }

    pub fn clear(&mut self) -> bool {
        self.head = None;
        self.leaves.clear();
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree<H: digest_primitives::Hasher>() -> merkle-tree<H> {
        let mut tree: merkle-tree<H> = merkle-tree::new();
        tree.rebuild();
        return tree;
    }

    fn create_nempty_tree<H: digest_primitives::Hasher>(offset: i32) -> merkle-tree<H> {
        let base_values = [
            -109340, -12934, -10000, -8403, -10,
            1, 8493, 65648, 128003, 748930, 1256000, 105673456,
        ];

        let data_values: Vec<i128> = base_values.iter().map(|&v| v as i128 + offset as i128).collect::<Vec<i128>>();
        let mut tree: merkle-tree<H> = merkle-tree::new();
        for i in data_values {
            tree.insert(i as i128);
        }

        tree.rebuild();
        return tree;
    }

    #[test]
    fn empty_update() -> () {
        let mut tree: merkle-tree<digest_primitives::ripemd160::Ripemd160> = create_tree();
        assert!(!tree.update(15, 936), "Function update something, but tree don't contain any data!");
    }

    #[test]
    fn cmp_test() -> () {
        let mut ftree: merkle-tree<digest_primitives::ripemd160::Ripemd160> = create_nempty_tree(0);
        let mut stree: merkle-tree<digest_primitives::ripemd160::Ripemd160> = create_nempty_tree(0);
        assert!(ftree.equals(&mut stree), "Trees are not same, but should be!");
    }

    #[test]
    fn ncmp_test() -> () {
        let mut ftree: merkle-tree<digest_primitives::ripemd160::Ripemd160> = create_nempty_tree(0);
        let mut stree: merkle-tree<digest_primitives::ripemd160::Ripemd160> = create_nempty_tree(1);
        assert!(!ftree.equals(&mut stree), "Trees are same, but shouldn't be!");
    }

    #[test]
    fn update_test() -> () {
        let mut tree: merkle-tree<digest_primitives::ripemd160::Ripemd160> = create_nempty_tree(0);
        assert!(tree.update(4, 936), "Function can't update data, but should do this!"); 
    }

    #[test]
    fn clear_update() -> () {
        let mut tree: merkle-tree<digest_primitives::ripemd160::Ripemd160> = create_nempty_tree(15);
        assert!(tree.update(4, 936), "Function can't update data, but should do this!"); 
        tree.clear();
        assert!(!tree.update(4, 936), "Function update something, but tree don't contain any data!");
    }

    #[test]
    fn update_test2() -> () {
        let mut ftree: merkle-tree<digest_primitives::ripemd160::Ripemd160> = create_nempty_tree(0);
        let mut stree: merkle-tree<digest_primitives::ripemd160::Ripemd160> = create_nempty_tree(0);
        assert!(ftree.equals(&mut stree), "Trees are not same, but should be!");

        let prev: i128 = stree.get(4);
        assert!(stree.update(4, 936), "Function can't update data, but should do this!"); 
        assert!(!ftree.equals(&mut stree), "Trees are same, but shouldn't be!");
        assert!(stree.update(4, prev), "Function can't update data, but should do this!"); 
        assert!(ftree.equals(&mut stree), "Trees are not same, but should be!");
    }

    #[test]
    fn delete_test() -> () {
        let mut ftree: merkle-tree<digest_primitives::ripemd160::Ripemd160> = create_nempty_tree(0);
        let mut stree: merkle-tree<digest_primitives::ripemd160::Ripemd160> = create_nempty_tree(0);
        assert!(ftree.equals(&mut stree), "Trees are not same, but should be!");
        ftree.delete(0);
        assert!(!ftree.equals(&mut stree), "Trees are same, but shouldn't be!");
    }
}
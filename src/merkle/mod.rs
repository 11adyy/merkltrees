use std::{self, cell::RefCell, rc::Rc};
use digest_primitives;
mod leaf;

pub struct merkle-tree<H, T>
where
    H: digest_primitives::Hasher,
    T: digest_primitives::Hashable
{
    hfunc: H,
    leaves: Vec<Rc<RefCell<leaf::Leaf<T>>>>,
    head: Option<Rc<RefCell<leaf::Leaf<T>>>>,
}

impl<H, T> merkle-tree<H, T>
where
    H: digest_primitives::Hasher,
    T: digest_primitives::Hashable
{
    pub fn new() -> merkle-tree<H, T> {
        merkle-tree {
            hfunc:  H::new(),
            leaves: vec![],
            head:   None,
        }
    }

    /*
    Get data from merkle tree by index.
    Params:
    - index: usize - Index of element.

    Return data with T type.
     */
    pub fn get(&self, index: usize) -> T
    where
        T: Clone
    {
        self.leaves[index].borrow().data.clone()
    }
    

    /*
    Compare two instance of Merkle tree. This is a 'shallow' compare -> We compare only root hash.
    For deep compare with difference return use 'deep_equals'.
    Params:
    - tree: &merkle-tree<H> - Second Merkle tree with same hash function.

    Return true if tree's root hashes are equals.
    Note: Will return false if head is NULL.
     */
    pub fn equals<H2: digest_primitives::Hasher>(&self, tree: &merkle-tree<H2, T>) -> bool {
        if self.head.is_none() || tree.head.is_none() {
            return false;
        }

        return self.head.as_ref().unwrap().borrow().equals(&tree.head.as_ref().unwrap().borrow());
    }

    pub fn rebuild(&mut self) -> bool {
        if self.leaves.is_empty() {
            return false;
        }
        
        let mut current_level: Vec<Rc<RefCell<leaf::Leaf<T>>>> = self.leaves.clone();
        while current_level.len() > 1 {
            let mut next_level: Vec<Rc<RefCell<leaf::Leaf<T>>>> = Vec::new();
            let mut i = 0;
            while i < current_level.len() {
                let left: Rc<RefCell<leaf::Leaf<T>>> = current_level[i].clone();
                let right: Rc<RefCell<leaf::Leaf<T>>> = if i + 1 < current_level.len() {
                    current_level[i + 1].clone()
                } 
                else {
                    left.clone()
                };

                let parent = leaf::Leaf::create_from_childrens(left, right, &self.hfunc);
                next_level.push(parent);
                i += 2;
            }

            current_level = next_level;
        }

        self.head = current_level.pop();
        return true;
    }

    pub fn push(&mut self, data: T) -> bool {
        let new_leaf: Rc<RefCell<leaf::Leaf<T>>> = Rc::new(RefCell::new(leaf::Leaf::create_from_data(data, &self.hfunc)));
        self.leaves.push(new_leaf);
        return true;
    }

    pub fn update(&mut self, index: usize, data: T) -> bool {
        let leaf: std::rc::Rc<std::cell::RefCell<leaf::Leaf<T>>> = match self.leaves.get(index) {
            Some(leaf) => leaf.clone(),
            None => return false,
        };
    
        {
            let mut leaf_borrow = leaf.borrow_mut();
            leaf_borrow.update(data, &self.hfunc);
        }
    
        if self.head.is_none() || leaf.borrow().parent.is_none() {
            self.rebuild();
            return true;
        }        

        let mut current: Rc<RefCell<leaf::Leaf<T>>> = leaf;
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
    
        self.rebuild();
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

    #[derive(Clone)]
    pub struct I128Wrapper(pub i128);

    impl digest_primitives::Hashable for I128Wrapper {
        fn default() -> Self {
            return I128Wrapper(0);
        }

        fn to_bytes(&self) -> Vec<u8> {
            return self.0.to_be_bytes().to_vec();
        }
    }

    fn create_tree<H: digest_primitives::Hasher, T: digest_primitives::Hashable>() -> merkle-tree<H, T> {
        let mut tree: merkle-tree<H, T> = merkle-tree::new();
        tree.rebuild();
        return tree;
    }

    fn create_nempty_tree<H: digest_primitives::Hasher>(offset: i32) -> merkle-tree<H, I128Wrapper> {
        let base_values = [
            -109340, -12934, -10000, -8403, -10,
            1, 8493, 65648, 128003, 748930, 1256000, 105673456,
        ];

        let data_values: Vec<i128> = base_values.iter().map(|&v| v as i128 + offset as i128).collect::<Vec<i128>>();
        let mut tree: merkle-tree<H, I128Wrapper> = merkle-tree::new();
        for i in data_values {
            tree.push(I128Wrapper(i));
        }

        tree.rebuild();
        return tree;
    }

    #[test]
    fn empty_update() -> () {
        let mut tree: merkle-tree<digest_primitives::ripemd160::Ripemd160, I128Wrapper> = create_tree();
        assert!(!tree.update(15, I128Wrapper(936)), "Function update something, but tree don't contain any data!");
    }

    #[test]
    fn cmp_test() -> () {
        let ftree: merkle-tree<digest_primitives::ripemd160::Ripemd160, I128Wrapper> = create_nempty_tree(0);
        let stree: merkle-tree<digest_primitives::ripemd160::Ripemd160, I128Wrapper> = create_nempty_tree(0);
        assert!(ftree.equals(&stree), "Trees are not same, but should be!");
    }

    #[test]
    fn ncmp_test() -> () {
        let ftree: merkle-tree<digest_primitives::ripemd160::Ripemd160, I128Wrapper> = create_nempty_tree(0);
        let stree: merkle-tree<digest_primitives::ripemd160::Ripemd160, I128Wrapper> = create_nempty_tree(1);
        assert!(!ftree.equals(&stree), "Trees are same, but shouldn't be!");
    }

    #[test]
    fn ncmp_test2() -> () {
        let ftree: merkle-tree<digest_primitives::ripemd160::Ripemd160, I128Wrapper> = create_nempty_tree(0);
        let stree: merkle-tree<digest_primitives::md5::MD5, I128Wrapper> = create_nempty_tree(0);
        assert!(!ftree.equals(&stree), "Trees are same, but shouldn't be!");
    }

    #[test]
    fn update_test() -> () {
        let mut tree: merkle-tree<digest_primitives::tigerhash::TigerHash, I128Wrapper> = create_nempty_tree(0);
        assert!(tree.update(4, I128Wrapper(936)), "Function can't update data, but should do this!"); 
    }

    #[test]
    fn clear_update() -> () {
        let mut tree: merkle-tree<digest_primitives::sha512::SHA512, I128Wrapper> = create_nempty_tree(15);
        assert!(tree.update(4, I128Wrapper(936)), "Function can't update data, but should do this!"); 
        tree.clear();
        assert!(!tree.update(4, I128Wrapper(936)), "Function update something, but tree don't contain any data!");
    }

    #[test]
    fn update_test2() -> () {
        let ftree: merkle-tree<digest_primitives::sha3::SHA3, I128Wrapper> = create_nempty_tree(0);
        let mut stree: merkle-tree<digest_primitives::sha3::SHA3, I128Wrapper> = create_nempty_tree(0);
        assert!(ftree.equals(&stree), "Trees are not same, but should be!");

        let prev: I128Wrapper = stree.get(4);
        assert!(stree.update(4, I128Wrapper(936)), "Function can't update data, but should do this!"); 
        assert!(!ftree.equals(&stree), "Trees are same, but shouldn't be!");
        assert!(stree.update(4, prev), "Function can't update data, but should do this!"); 
        assert!(ftree.equals(&stree), "Trees are not same, but should be!");
    }

    #[test]
    fn delete_test() -> () {
        let mut ftree: merkle-tree<digest_primitives::blake2b::Blake2B, I128Wrapper> = create_nempty_tree(0);
        let stree: merkle-tree<digest_primitives::blake2b::Blake2B, I128Wrapper> = create_nempty_tree(0);
        assert!(ftree.equals(&stree), "Trees are not same, but should be!");
        ftree.delete(0);
        assert!(!ftree.equals(&stree), "Trees are same, but shouldn't be!");
    }
}
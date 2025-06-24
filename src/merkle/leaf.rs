use std::{self, cell::RefCell, rc::{Rc, Weak}};
use digest_primitives;

#[derive(Clone)]
pub struct Leaf<T: digest_primitives::Hashable> {
    pub parent: Option<Weak<RefCell<Leaf<T>>>>,
    pub left:   Option<Rc<RefCell<Leaf<T>>>>,
    pub right:  Option<Rc<RefCell<Leaf<T>>>>,
    pub hash:   digest_primitives::Hash,
    pub data:   T,
}

impl<T: digest_primitives::Hashable> Leaf<T> {
    pub fn new() -> Leaf<T> {
        return Leaf {
            parent: None, 
            left:   None, 
            right:  None,
            hash: digest_primitives::Hash::new(0), 
            data: T::default()
        }
    }

    /*
    Update leaf data with hash recalculation. Will not affect to parent.
    Params:
    - data: T - New data.
    - hfunc: &impl digest_primitives::Hasher - Hash function.

    Return true.
     */
    pub fn update(&mut self, data: T, hfunc: &impl digest_primitives::Hasher) -> bool {
        self.data = data;
        self.hash = hfunc.hash(self.data.to_bytes().as_slice());
        return true;
    }

    /*
    Compare current leaf instance with another by hashes.
    Note: If leaf's hashes are not hashed yet (flag is_hashed is false), will return false.
    Params:
    - leaf: &Leaf<T> - Another leaf.

    Return true is leaf hashes are same.
     */
    pub fn equals(&self, leaf: &Leaf<T>) -> bool {
        if !self.hash.is_hashed() || !leaf.hash.is_hashed() {
            return false;
        }

        return self.hash.equals(&leaf.hash);
    }

    /*
    Will create a new leaf with filled data field and hashed hash.
    Params:
    - data: T - Leaf data.
    - hfunc: &impl digest_primitives::Hasher - Hash function.

    Return a new leaf.
     */
    pub fn create_from_data(data: T, hfunc: &impl digest_primitives::Hasher) -> Leaf<T> {
        let mut body: Leaf<T> = Leaf::new();
        body.data = data;
        body.hash = hfunc.hash(body.data.to_bytes().as_slice());
        return body;
    }

    /*
    Will create a new leaf with filled childrean fields. 
    Note: Right and Left children will remember their new parent.
    Note 2: Function will re_hash parent hash with new children data.
    Params:
    - l: Rc<RefCell<Leaf<T>>> - Left children.
    - r: Rc<RefCell<Leaf<T>>> - Right children.
    - hfunc: &impl digest_primitives::Hasher - Hash function.

    Return a new leaf.
     */
    pub fn create_from_childrens(
        l: Rc<RefCell<Leaf<T>>>,
        r: Rc<RefCell<Leaf<T>>>,
        hfunc: &impl digest_primitives::Hasher
    ) -> Rc<RefCell<Leaf<T>>> {
        let body = Rc::new(RefCell::new(Leaf::new()));

        {
            let mut body_mut = body.borrow_mut();
            body_mut.left = Some(std::rc::Rc::clone(&l));
            body_mut.right = Some(std::rc::Rc::clone(&r));
        }

        {
            let weak_parent = std::rc::Rc::downgrade(&body);
            l.borrow_mut().parent = Some(weak_parent.clone());
            r.borrow_mut().parent = Some(weak_parent);
        }

        body.borrow_mut().rehash(hfunc);
        return body;
    }

    /*
    Will re-calculate leaf hash according to children hashes.
    Params:
    - hfunc: &impl digest_primitives::Hasher - Hash function.

    Return true if calculation success.
    Return false if left or right children is NULL.
     */
    pub fn rehash(&mut self, hfunc: &impl digest_primitives::Hasher) -> bool {
        if self.left.is_none() || self.right.is_none() {
            return false;
        }
    
        let left = self.left.as_ref().unwrap().borrow();
        let right = self.right.as_ref().unwrap().borrow();
        self.hash = hfunc.hash(left.hash.concat(&right.hash).to_bytes());
        return true;
    }
}

use std::{self, cell::RefCell, rc::{Rc, Weak}};
use digest_primitives;
use crate::merkle::changer;

pub fn compare_leafs<T: digest_primitives::Hashable + Clone>(
    a: &Option<Rc<RefCell<Leaf<T>>>>,
    b: &Option<Rc<RefCell<Leaf<T>>>>,
    changes: &mut Vec<changer::ChangeOp<T>>,
) -> () {
    match (a, b) {
        (None, None) => {}

        (Some(na), None) => {
            Leaf::get_changeset(&Some(na.clone()), changes, changer::ChangeKind::Delete);
        }

        (None, Some(nb)) => {
            Leaf::get_changeset(&Some(nb.clone()), changes, changer::ChangeKind::Insert);
        }

        (Some(na), Some(nb)) => {
            let na_borrow = na.borrow();
            let nb_borrow = nb.borrow();

            if na_borrow.hash.equals(&nb_borrow.hash) {
                return;
            }

            let key_a = na_borrow.data.get_id();
            let key_b = nb_borrow.data.get_id();

            let is_a_leaf = na_borrow.left.is_none() && na_borrow.right.is_none();
            let is_b_leaf = nb_borrow.left.is_none() && nb_borrow.right.is_none();

            drop(na_borrow);
            drop(nb_borrow);

            if is_a_leaf && is_b_leaf {
                let na = na.borrow();
                let nb = nb.borrow();

                if key_a == key_b {
                    changes.push(changer::ChangeOp::Update {
                        id: key_a,
                        value: na.data.clone(),
                        nvalue: nb.data.clone(),
                    });
                } else {
                    changes.push(changer::ChangeOp::Delete {
                        id: key_a,
                        value: na.data.clone(),
                    });
                    changes.push(changer::ChangeOp::Insert {
                        id: key_b,
                        value: nb.data.clone(),
                    });
                }
                return;
            }

            let left_a = &na.borrow().left;
            let right_a = &na.borrow().right;
            let left_b = &nb.borrow().left;
            let right_b = &nb.borrow().right;

            compare_leafs(left_a, left_b, changes);
            compare_leafs(right_a, right_b, changes);
        }
    }
}

#[derive(Clone)]
pub struct Leaf<T: digest_primitives::Hashable> {
    pub parent: Option<Weak<RefCell<Leaf<T>>>>,
    pub left:   Option<Rc<RefCell<Leaf<T>>>>,
    pub right:  Option<Rc<RefCell<Leaf<T>>>>,
    pub hash:   digest_primitives::Hash,
    pub data:   T
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

    /*
    Go deeper to data leafs for saving id and data.
    Params:
    - node: &Option<Rc<RefCell<Leaf<T>>>> - Entry point.
    - changes: &mut Vec<changer::ChangeOp<T>> - Storage for changeset.
    - kind: changer::ChangeKind - Change kind.
     */
    fn get_changeset(
        node: &Option<Rc<RefCell<Leaf<T>>>>,
        changes: &mut Vec<changer::ChangeOp<T>>,
        kind: changer::ChangeKind,
    ) -> () {
        if let Some(rc_leaf) = node {
            let leaf = rc_leaf.borrow();

            if leaf.left.is_none() && leaf.right.is_none() {
                let id = leaf.data.get_id();
                let value = leaf.data.clone();

                let change = match kind {
                    changer::ChangeKind::Insert => changer::ChangeOp::Insert { id, value },
                    changer::ChangeKind::Delete => changer::ChangeOp::Delete { id, value },
                    changer::ChangeKind::Update => return
                };

                changes.push(change);
            } 
            else {
                Self::get_changeset(&leaf.left, changes, kind.clone());
                Self::get_changeset(&leaf.right, changes, kind);
            }
        }
    }
}

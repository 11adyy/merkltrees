use std;
use ripemd160;

#[derive(Clone)]
pub struct Leaf {
    /* Tree structure information */
    pub parent: Option<std::rc::Weak<std::cell::RefCell<Leaf>>>,
    pub left: Option<std::rc::Rc<std::cell::RefCell<Leaf>>>,
    pub right: Option<std::rc::Rc<std::cell::RefCell<Leaf>>>,

    /* Additional information */
    pub sub_tree_size: i32,
    pub max_value: i32,
    pub avg_value: f64,
    pub min_value: i32,

    /* Data and hash */
    pub hash: [u8; 20],
    pub data: i128
}

impl Leaf {
    pub fn create() -> Leaf {
        return Leaf {
            parent: None, left: None, right: None,
            sub_tree_size: 0, max_value: 0, avg_value: 0., min_value: 0,
            hash: [0; 20], data: 0
        }
    }

    pub fn equals(&self, leaf: &Leaf) -> bool {
        return self.hash == leaf.hash;
    }

    pub fn create_from_data(data: i128) -> Leaf {
        let mut body: Leaf = Leaf::create();
        body.data = data;
        body.hash = ripemd160::hash(&data.to_le_bytes());
        return body;
    }

    pub fn create_from_childrens(
        l: std::rc::Rc<std::cell::RefCell<Leaf>>,
        r: std::rc::Rc<std::cell::RefCell<Leaf>>,
    ) -> std::rc::Rc<std::cell::RefCell<Leaf>> {
        let body = std::rc::Rc::new(std::cell::RefCell::new(Leaf::create()));

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

        body.borrow_mut().rehash();
        return body;
    }

    pub fn rehash(&mut self) -> bool {
        let left_rc = match &self.left {
            Some(rc) => rc.clone(),
            None => return false,
        };

        let right_rc = match &self.right {
            Some(rc) => rc.clone(),
            None => return false,
        };
    
        let left = left_rc.borrow();
        let right = right_rc.borrow();
    
        let summary: [u8; 40] = ripemd160::hashcat(&left.hash, &right.hash);
        self.hash = ripemd160::hash(&summary);
        return true;
    }
}

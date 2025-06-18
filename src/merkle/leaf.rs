use ripemd160;

pub struct Leaf {
    pub left: Option<Box<Leaf>>,
    pub right: Option<Box<Leaf>>,
    pub hash: [u8; 20],
    pub data: Option<String>
}

impl Leaf {
    pub fn create() -> Leaf {
        return Leaf {
            left: None,
            right: None,
            hash: [0; 20],
            data: None
        }
    }

    pub fn create_from_data(data: &str) -> Leaf {
        let mut body: Leaf = Leaf::create();
        body.data = Some(data.to_string());
        body.hash = ripemd160::hash(data.as_bytes());
        return body;
    }

    pub fn create_from_childrens(l: Box<Leaf>, r: Box<Leaf>) -> Leaf {
        let mut body: Leaf = Leaf::create();
        body.left  = Some(l);
        body.right = Some(r);

        body.rehash();
        return body;
    }

    pub fn rehash(&mut self) -> bool {
        if self.left.is_none() || self.right.is_none() {
            return false;
        }

        let left: &Box<Leaf>  = self.left.as_ref().unwrap();
        let right: &Box<Leaf> = self.right.as_ref().unwrap();
        let summary: [u8; 40] = ripemd160::hashcat(&left.hash, &right.hash);

        self.hash = ripemd160::hash(&summary);
        return true;
    }

    pub fn is_leaf(&self) -> bool {
        return self.left.is_none() && self.right.is_none();
    }
}

use crate::merkle::leaf;

pub struct merkle-tree {
    leaves: Vec<Box<leaf::Leaf>>,
    head: Option<Box<leaf::Leaf>>
}

impl merkle-tree {
    pub fn new() -> merkle-tree {
        return merkle-tree {
            leaves: vec![],
            head: None
        };
    }

    pub fn equals(&mut self, tree: &mut merkle-tree) -> bool {
        if !self.rebuild() || !tree.rebuild() {
            return false;
        }

        return self.head.as_ref().unwrap().equals(tree.head.as_ref().unwrap().as_ref());
    }

    pub fn rebuild(&mut self) -> bool {
        let mut current_level: Vec<Box<leaf::Leaf>> = self.leaves.clone();

        while current_level.len() > 1 {
            let mut next_level: Vec<Box<leaf::Leaf>> = Vec::new();

            let mut i = 0;
            while i < current_level.len() {
                let left = current_level[i].clone();
                let right = if i + 1 < current_level.len() {
                    current_level[i + 1].clone()
                } 
                else {
                    left.clone()
                };

                let parent = Box::new(leaf::Leaf::create_from_childrens(left, right));
                next_level.push(parent);
                i += 2;
            }

            current_level = next_level;
        }

        self.head = current_level.pop();
        return true;
    }

    pub fn insert(&mut self, data: &str) -> bool {
        let leaf_node = Box::new(leaf::Leaf::create_from_data(data));
        self.leaves.push(leaf_node);
        return true;
    }
}

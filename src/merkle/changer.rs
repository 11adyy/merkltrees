#[derive(Debug, Clone)]
pub enum ChangeKind {
    Insert,
    Delete,
    Update
}

#[derive(Debug, Clone)]
pub enum ChangeOp<T> {
    Insert { id: usize, value: T },
    Delete { id: usize, value: T },
    Update { id: usize, value: T, nvalue: T },
}
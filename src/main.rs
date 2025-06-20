mod merkle;

fn main() {
    let mut tree = merkle::init_merkle();
    tree.insert("Data1");
    tree.insert("Data2");
    tree.insert("Data3");
    tree.insert("Data4");
    tree.insert("Data31");
    tree.insert("Data41");
    tree.rebuild();

    let mut stree = merkle::init_merkle();
    stree.insert("Data1");
    stree.insert("Data2");
    stree.insert("Data34");
    stree.insert("Data4");
    stree.insert("Data31");
    stree.insert("Data41");
    stree.rebuild();

    if tree.equals(&mut stree) {
        print!("Same!\n");
    }
    else {
        print!("Not same!\n");
    }

    stree.update(2, "Data3");
    if tree.equals(&mut stree) {
        print!("Same!\n");
    }
    else {
        print!("Not same!\n");
    }
}

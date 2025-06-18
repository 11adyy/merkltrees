mod merkle;

fn main() {
    let mut tree = merkle::init_merkle();
    tree.insert("Data1");
    tree.insert("Data2");
    tree.insert("Data3");
    tree.insert("Data4");

    tree.insert("Data31");
    tree.insert("Data41");
    tree.print_tree();
}

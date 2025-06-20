mod merkle;

fn main() {
    let mut ftree: merkle::merkle-tree = merkle::merkle-tree::new();
    assert!(!ftree.update(15, 936), "Function update something, but tree don't contain any data!");
    let mut stree: merkle::merkle-tree = merkle::merkle-tree::new();
    assert!(!stree.update(2, 128), "Function update something, but tree don't contain any data!");

    {
        /* fdataset usage */
        let fdataset: Vec<i128> = vec![-109340, -12934, -10000, -8403, -10, 1, 8493, 65648, 128003, 748930, 1256000, 105673456];
        for i in fdataset {
            ftree.insert(i);
            stree.insert(i);
        }

        ftree.rebuild();
        stree.rebuild();

        assert!(ftree.equals(&mut stree), "Trees are not same, but should be!");
    }

    ftree.update(4, 999);
    assert!(!ftree.equals(&mut stree), "Trees are same, but shouldn't be!");
    ftree.update(4, -10);
    assert!(ftree.equals(&mut stree), "Trees are not same, but should be!");

    ftree.clear();
    stree.clear();
}

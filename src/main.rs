mod avl;

fn main() {
    let mut tree = avl::Tree::new();
    tree.insert(4);
    tree.insert(5);
    tree.insert(6);
    tree.insert(7);
    tree.rotate_left();
    tree.rotate_right();
    println!("tree: {:?}", tree);

    println!("contains({}) = {}", 6, tree.contains(6));
    tree.delete(6);
    println!("contains({}) = {}", 6, tree.contains(6));
}

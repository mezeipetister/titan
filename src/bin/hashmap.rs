use std::collections::HashMap;

fn main() {
    let mut commit: HashMap<&'static str, Vec<i32>> = HashMap::new();

    commit.insert("a", vec![1, 2, 3]);
    commit.insert("b", vec![4, 5, 6]);
    commit.insert("c", vec![7, 8, 9]);

    commit.retain(|&k, _| k == "a" || k == "c");

    println!("{:?}", &commit);
}

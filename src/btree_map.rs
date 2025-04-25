use std::collections::BTreeMap;
pub fn btree_map() {
    // Create a new BTreeMap
    let mut map = BTreeMap::new();
    map.insert("apple", 3);
    map.insert("banana", 2);
    map.insert("orange", 5);
    map.insert("grape", 7);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

#[test]
fn hashmap_entry_and_modified() {
    use std::collections::HashMap;
    let mut map = HashMap::<_, u8>::new();
    map.entry("key")
        .and_modify(|count| *count += 1)
        .or_default();
    *map.entry("key").or_default() += 1;
    assert_eq!(map["key"], 1);
}

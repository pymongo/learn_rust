macro_rules! hashmap {
    (@pair $a:tt) => (());
    ($($key:expr => $val:expr),* $(,)*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

fn main() {
    let map = hashmap!("key1" => 1, "key2" => 2,);
    dbg!(map);
}

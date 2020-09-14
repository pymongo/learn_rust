macro_rules! hashmap {
    // Rust的元组本质上是匿名结构体，不能获取长度，所以这里通过数组去获取hashmap长度
    (@count $($key:expr,)*) => {[$($key,)*].len()};
    // 末尾的$(,)*用于匹配末尾的逗号，例如("a"=>1,)
    // $(),*可以让末尾不带逗号，但是key-value分隔处必须带逗号
    ($($key:expr => $val:expr),* $(,)*) => {
        {
            // std::collections::HashMap::with_capacity(["key1", "key2"].len());
            let mut map = std::collections::HashMap::with_capacity(hashmap!(@count $($key,)*));
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

macro_rules! hashmap2 {
    // 将计算长度的宏依赖的两个宏作为hashmap内部的一个规则分支，注意要先定义后调用所以@pair_to_empty_tuple在最前面
    // 内部宏规则以'@'开头时Rust社区的一个惯用法
    (@pair_to_empty_tuple $($_:tt)*) => (());
    (@count $($key:expr),*) => {[$(hashmap2!(@pair_to_empty_tuple $key)),*].len()};
        // (<[()]>::len(&[$(hashmap2!(@unit $rest)),*]));
    ($($key:expr => $value:expr),* $(,)*) => {
        {
            let mut _map = std::collections::HashMap::with_capacity(hashmap2!(@count $($key),*));
            $(
               _map.insert($key, $value);
            )*
            _map
        }
   };
}

// declare_macro feature
// macro unless($statement:expr, $branch:expr) {
// (if !$statement {$branch});
// }

/*
let _map = {
    let mut map = std::collections::HashMap::with_capacity(["key1", "key2"].len());
    map.insert("key1", 1);
    map.insert("key2", 2);
    map
};
let _map2 = {
    let mut _map = std::collections::HashMap::with_capacity([(), ()].len());
    _map.insert("key1", 1);
    _map.insert("key2", 2);
    _map
};
*/
fn main() {
    let _map = hashmap!("key1" => 1, "key2" => 2);
    let _map2 = hashmap2!("key1" => 1, "key2" => 2);
    // unless!(1>2 {
    //     println!("if not 1>2");
    // })
}

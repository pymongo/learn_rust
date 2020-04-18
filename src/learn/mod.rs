// 如果module是在另一个文件夹内，这个文件夹内必须有一个mod.rs
// 如果是在main.rs同级文件夹中，直接mod 文件名;即可
pub mod argv;
pub mod enum_match;
pub mod env;
pub mod execute_command;
pub mod iter_vec;
pub mod http_request;
pub mod option_type;
pub mod random_and_tuple;
pub mod read_file;
pub mod reference;
pub mod std_input;
pub mod struct_and_impl;
pub mod unit_test;
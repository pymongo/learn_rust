use std::io;

pub fn run() {
  // std::collections::HashMap, insert/get

  let mut input = String::new();
  // 如果没有use，要写成std::io::stdin()
  io::stdin().read_line(&mut input)
    .ok()
    .expect("Error in read");
  // expect: 如果ok的返回值isn’t a successful one, panic!s with a message you passed it
  // references are immutable by default. Hence, we need to write &mut input
  // read_line的返回值是io::Result
  println!("input is {}", input);
  // 字符串转整形
  let input_to_int: u32 = input.trim().parse()
    .ok()
    .expect("Please type a number!");
}
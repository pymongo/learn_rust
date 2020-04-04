fn test() {
  // Read-Only reference 好像也叫borrowing
  let mut x = 0;
  // 此时x变量被干掉了，不能引用x了
  let pointer = &mut x; // 可变指针：
  *pointer += 1;
  println!("{}", pointer); // 注意获取值时不需要*pointer
  println!("{}", *pointer); // 注意获取值时不需要*pointer
  // 【警告】：一旦定义了x的可变指针，那么打印时不能打印x要用x的可变指针
  // (可变指针只能有一个，感觉是避免野指针的设计)
  // 或者把x的可变指针以及修改x内容的部分放到一个code block或函数中

  /* 尝试通过指针修改跨作用域修改一个值 */
  let mut number: u8 = 1;
  increase(&mut number);
  println!("{}", number);

  // OK，因为基本类型是存储在stack中
  let aa = 1;
  let bb = aa;
  println!("aa = {}, bb = {}", aa, bb);
  // OK，虽然String存储在Heap中，但是不可变指针可以有多个
  let aa = "2";
  let bb = aa;
  println!("aa = {}, bb = {}", aa, bb);
}

fn increase(input_number: &mut u8) {
  *input_number += 1;
}
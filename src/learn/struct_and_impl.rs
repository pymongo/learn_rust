struct Color(u8, u8, u8);

#[allow(dead_code)]
impl Color {
  // associated function
  fn print_rgb(&self) {
    println!("red's RGB is ({}, {}, {})", self.0, self.1, self.2);
  }
  // 构造方法
  fn new(r:u8, g:u8, b:u8) -> Color {
    Color(r, g, b)
  }
}
// trait相当于Java的Interface，这里实现了toString trait
impl ToString for Color {
  fn to_string(&self) -> String {
    // unimplemented!()
    String::from("Color.toString")
  }
}

trait OnMessage {
  fn callback(&self);
}

impl OnMessage for Color {
  fn callback(&self) {
    // unimplemented!()
    // doNoThing
  }
}

#[allow(dead_code)]
pub fn run() {
  // let user1 = User {name: 1, password: 12};
  // Error: user1.name = 1;
  struct User {
    name: u8,
    password: u8,
  }
  let mut user2 = User {name: 1, password: 23};
  user2.name = 12;
  println!("{}", user2.name);

  // let red = Color(255, 0, 0);
  let red = Color::new(255, 0, 0);
  red.print_rgb();

  let _numbers1: [u8; 4] = [1,2,3,4];
  let _numbers2: [u8; 4] = [0; 4];

  // let my_str = String::from("haha");
  // 修改字符串：my_str.push_str("asdf");
}
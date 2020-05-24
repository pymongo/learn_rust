struct Color(u8, u8, u8);

impl Color {
    // associated function
    fn print_rgb(&self) {
        println!("red's RGB is ({}, {}, {})", self.0, self.1, self.2);
    }
    // 构造方法
    fn new(r: u8, g: u8, b: u8) -> Color {
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

fn main() {
    let red = Color::new(255, 0, 0);
    red.print_rgb();
}

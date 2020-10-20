use std::fs::File;
use std::io::prelude::*;

/**
```no_run
// Write to file Example
let mut write_file = File::create("output.txt").expect("Error");
// b"" means byte slice
write_file.write_all(b"Haha").expect("Error");
```
*/
fn main() {
    let mut file = File::open("examples/basic_read_file.rs").expect("Open Failed");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Can't read file");

    println!("{}", content);
}

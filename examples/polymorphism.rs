trait Animal {
    fn eat(&self);
}

struct Cat;

impl Animal for Cat {
    fn eat(&self) {
        println!("Cat is eating");
    }
}

struct Dog;

impl Animal for Dog {
    fn eat(&self) {
        println!("Dog is eating");
    }
}

fn main() {
    let cat = Cat{};
    let dog = Dog{};
    let animals = vec![cat, dog];
}

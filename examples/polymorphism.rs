// Any主要是用于多态时upcast，但是Rust多态不需要向上塑型，而且我代码里没有管Any
// `trait A: B`也可以写成`trait A where Self: B`
// Rust没有继承, trait A : B 实际上是给A加上一个约束条件: implement B also need to implement A
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

fn make_animal_eating(animal: &dyn Animal) {
    animal.eat();
}

fn main() {
    make_animal_eating(&Cat{});
    make_animal_eating(&Dog{});
    // cat和dog实例需要分配在堆内存中才能装入Vec，否则会报错: Sized is not known at compile time
    let cat = Box::new(Cat{});
    let dog = Box::new(Dog{});
    let animals: Vec<Box<dyn Animal>> = vec![cat, dog];
    for animal in animals {
        animal.eat();
    }
}

// Any主要是用于多态时upcast，但是Rust多态不需要向上塑型，而且我代码里没有管Any
// `trait A: B`也可以写成`trait A where Self: B`
// Rust没有继承, trait A : B 实际上是给A加上一个约束条件: implement B also need to implement A
trait Animal {
    fn eat(&self);
    fn print_type_name(&self) {
        dbg!(std::any::type_name::<Self>());
        dbg!(std::mem::size_of::<&Self>());
        dbg!(std::mem::size_of_val(&self));
        // dbg!(std::mem::size_of_val(self));
    }
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

fn eat_dyn(animal: &dyn Animal) {
    animal.eat();
}

fn eat_static_impl_trait(animal: &impl Animal) {
    animal.eat();
}

fn eat_static<T: Animal>(animal: &T) {
    animal.eat();
}

// the trait `Animal` is not implemented for `&dyn Animal`
// fn make_animal_eating_2<T: Animal>(animal: T) {
//     animal.eat();
// }

fn main() {
    // make_animal_eating(&Cat{});
    // cat和dog实例需要分配在堆内存中才能装入Vec，否则会报错: Sized is not known at compile time
    // 或者只存cat和dog的指针
    let cat = Box::new(Cat);
    let dog = Box::new(Dog {});
    let animals: Vec<Box<dyn Animal>> = vec![cat, dog];
    for animal in animals {
        animal.eat();
    }

    // let b = Cat;
    // the size for values of type `dyn Animal` cannot be known at compilation time
    // trait object需要分配在堆内存中才能take ownership?
    // dyn Trait (unsized type) implements Trait. Implementations for &dyn Trait and/or &mut dyn Trait need to be explicitly provided.
    // let a: Vec<dyn Animal> = vec![b];

    println!("-- Vec<&dyn Animal> --");
    let cat2 = Cat;
    let dog2 = Dog {};
    eat_static(&cat2);
    eat_dyn_impl_trait(&dog2);
    let animals2: Vec<&dyn Animal> = vec![&cat2, &dog2];
    for animal in animals2 {
        eat_dyn(animal);
        // make_animal_eating_2(animal);
        animal.print_type_name();
    }
}

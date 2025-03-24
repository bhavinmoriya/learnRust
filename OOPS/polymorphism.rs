trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn make_noise(animals: Vec<&dyn Animal>) {
    for animal in animals {
        animal.speak();
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    let animals: Vec<&dyn Animal> = vec![&dog, &cat];
    make_noise(animals);
}

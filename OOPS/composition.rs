struct Engine {
    horsepower: u32,
}

struct Car {
    brand: String,
    engine: Engine,
}

fn main() {
    let my_engine = Engine { horsepower: 300 };
    let my_car = Car {
        brand: "BMW".to_string(),
        engine: my_engine,
    };

    println!("{} has {} HP", my_car.brand, my_car.engine.horsepower);
}

struct Car {
    brand: String,
    speed: u32,
}

impl Car {
    // Constructor
    fn new(brand: &str, speed: u32) -> Car {
        Car {
            brand: brand.to_string(),
            speed,
        }
    }

    // Method
    fn accelerate(&mut self, amount: u32) {
        self.speed += amount;
        println!("{} is now going at {} km/h", self.brand, self.speed);
    }

    // Getter method
    fn get_speed(&self) -> u32 {
        self.speed
    }
}

fn main() {
    let mut my_car = Car::new("Tesla", 60);
    my_car.accelerate(20);
    println!("Current speed: {}", my_car.get_speed());
}

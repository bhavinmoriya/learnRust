trait Vehicle {
    fn honk(&self);
   
}

struct Bike;

impl Vehicle for Bike {
    fn honk(&self) {
        println!("Bike: Ring ring!");
    }
}

struct Truck;

impl Vehicle for Truck {
    fn honk(&self) {
        println!("Truck: Honk Honk!");
    }
}

fn main() {
    let my_bike = Bike;
    let my_truck = Truck;
    
    
    my_bike.honk();
    my_truck.honk();
    
}

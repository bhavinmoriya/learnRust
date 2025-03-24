trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

struct Rectangle{
	length:f64,
	width:f64
}

impl Shape for Rectangle{
	fn area(&self)-> f64{
		self.length*self.width
	}
}

fn print_area(shape: &dyn Shape) {
    println!("The area is {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let r =Rectangle{length:3.,width:4.};
    print_area(&circle);
    print_area(&r);
}

use std::any::type_name_of_val;

fn main() {
    let x = 42;
    let y = "hello";
    let z = vec![1, 2, 3];

    println!("The type of x is: {}", type_name_of_val(&x));
    println!("The type of y is: {}", type_name_of_val(&y));
    println!("The type of z is: {}", type_name_of_val(&z));
}

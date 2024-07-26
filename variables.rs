fn main() {
    let x = 5; // immutable by default
    println!("The value of x is: {}", x);

    let mut y = 10; // mutable variable
    println!("The value of y is: {}", y);
    y = 15;
    println!("The new value of y is: {}", y);
}

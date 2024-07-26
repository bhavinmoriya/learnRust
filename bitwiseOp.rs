fn bitwise_add(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let carry = a & b;
        a = a ^ b;
        b = carry << 1;
    }
    a
}
fn bitwise_subtract(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        // Calculate the borrow
        let borrow = (!a) & b;
        
        // Subtract b from a using XOR (without considering borrow)
        a = a ^ b;
        
        // Shift the borrow left to subtract from the next significant bit
        b = borrow << 1;
    }
    a
}

fn bitwise_multiply(mut a: i32, mut b: i32) -> i32 {
    let mut result = 0;
    while b != 0 {
        if b & 1 != 0 {
            result += a;
        }
        a <<= 1; // Multiply a by 2
        b >>= 1; // Divide b by 2
    }
    result
}

fn bitwise_divide(mut dividend: i32, divisor: i32) -> (i32, i32) {
    if divisor == 0 {
        panic!("Division by zero is undefined");
    }

    let mut quotient = 0;
    let mut temp = 1;
    let mut divisor = divisor;

    // Align the divisor to the left
    while dividend >= (divisor << 1) {
        divisor <<= 1;
        temp <<= 1;
    }

    // Perform the division
    while temp > 0 {
        if dividend >= divisor {
            dividend -= divisor;
            quotient += temp;
        }
        divisor >>= 1;
        temp >>= 1;
    }

    (quotient, dividend) // Returns (quotient, remainder)
}


fn main() {
    let a = 7;
    let b = 5;
    println!("{} + {} = {}", a, b, bitwise_add(a, b));
    println!("{} - {} = {}", a, b, bitwise_subtract(a, b));
    println!("{} * {} = {}", a, b, bitwise_multiply(a, b));

    let dividend = 20;
    let divisor = 3;
    let (quotient, remainder) = bitwise_divide(dividend, divisor);
    println!("{} / {} = {} with a remainder of {}", dividend, divisor, quotient, remainder);

}

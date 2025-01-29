fn main() {
    let scale = 1000; // 3 decimal places
    let real_a = 3.75;
    let real_b = 2.25;

    // Encode as scaled integers
    let int_a = (real_a * scale as f64) as i32;  // 3.75 → 3750
    let int_b = (real_b * scale as f64) as i32;  // 2.25 → 2250

    // Perform integer arithmetic
    let int_sum = int_a + int_b;  // 3750 + 2250 = 6000

    // Decode back to real number
    let result = int_sum as f64 / scale as f64;  // 6000 → 6.00

    println!("Fixed-Point Sum: {}", result); // Output: 6.00
}

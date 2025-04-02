fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // s1 is borrowed

    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

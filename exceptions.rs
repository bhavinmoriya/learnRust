use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    let condition = true;
    //let condition = false;

    if condition {
        return Err("An error occurred".into()); // Converts string to Box<dyn Error>
    }

    println!("Function executed successfully.");
    Ok(())
}
fn main() {
    match run() {
        Ok(()) => println!("Success!"),
        Err(e) => println!("Error: {}", e),
    }
}

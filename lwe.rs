use rand::Rng;

fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn lwe_encrypt(message: i32, secret_key: &[i32]) -> (Vec<i32>, i32) {
    let mut rng = rand::thread_rng();
    
    // Generate a random vector (public key)
    let a: Vec<i32> = (0..secret_key.len()).map(|_| rng.gen_range(0..2)).collect();
    
    // Compute dot product of random vector with secret key
    let mut b = dot_product(&a, secret_key);
    
    // Add the message and a small random noise
    let error: i32 = rng.gen_range(-1..2); // Small error (noise)
    b += message + error;

    (a, b) // Return encrypted values
}

fn lwe_decrypt(a: &[i32], b: i32, secret_key: &[i32]) -> i32 {
    let dot = dot_product(a, secret_key);
    b - dot // Remove dot product to recover message (approx.)
}

fn main() {
    let secret_key = vec![1, 0, 1, 1]; // Example binary secret key
    
    let message = 3; // Message to encrypt
    let (a, b) = lwe_encrypt(message, &secret_key);
    
    println!("🔒 Encrypted: (a = {:?}, b = {})", a, b);
    
    let decrypted = lwe_decrypt(&a, b, &secret_key);
    println!("🔓 Decrypted Message: {}", decrypted);
}

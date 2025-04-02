#[derive(Debug, PartialEq)]
pub enum ComplexNumber {
    // Cartesian coordinates (real + imaginary parts)
    Cartesian { real: f64, imaginary: f64 },
    // Polar coordinates (magnitude + angle)
    Polar { magnitude: f64, angle: f64 },
}

impl ComplexNumber {
    // Create a new complex number in Cartesian form
    pub fn new_cartesian(real: f64, imaginary: f64) -> Self {
        ComplexNumber::Cartesian { real, imaginary }
    }

    // Create a new complex number in Polar form
    pub fn new_polar(magnitude: f64, angle: f64) -> Self {
        ComplexNumber::Polar { magnitude, angle }
    }

    // Convert to Cartesian coordinates
    pub fn to_cartesian(&self) -> (f64, f64) {
        match self {
            ComplexNumber::Cartesian { real, imaginary } => (*real, *imaginary),
            ComplexNumber::Polar { magnitude, angle } => (
                magnitude * angle.cos(),
                magnitude * angle.sin(),
            ),
        }
    }

    // Calculate magnitude (works for both representations)
    pub fn magnitude(&self) -> f64 {
        match self {
            ComplexNumber::Cartesian { real, imaginary } => (real.powi(2) + imaginary.powi(2)).sqrt(),
            ComplexNumber::Polar { magnitude, .. } => *magnitude,
        }
    }
}

// Example usage
fn main() {
    // Create complex numbers in different representations
    let cartesian = ComplexNumber::new_cartesian(3.0, 4.0);
    let polar = ComplexNumber::new_polar(5.0, 0.9273);

    // Convert between representations
    println!("Cartesian to polar magnitude: {:.2}", cartesian.magnitude());
    println!("Polar in Cartesian: {:?}", polar.to_cartesian());
    println!("Cartesian in Cartesian: {:?}", cartesian.to_cartesian());

    // Pattern matching for different representations
    match cartesian {
        ComplexNumber::Cartesian { real, imaginary } => {
            println!("Real: {}, Imaginary: {}", real, imaginary);
        }
        ComplexNumber::Polar { magnitude, angle } => {
            println!("Magnitude: {}, Angle: {}", magnitude, angle);
        }
    }
}

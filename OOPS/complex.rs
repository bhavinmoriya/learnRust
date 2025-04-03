struct Complex {
    real: f64,
    imag: f64,
}
impl Complex {
    fn print(&self) {
        if self.imag>=0.{
            println!("{} + {}*i", self.real, self.imag);
        }
        else {
            println!("{} - {}*i", self.real, -self.imag);
        }
        
    }
    fn add(&self, other: &Complex) -> Complex {
        let real = self.real + other.real;
        let imag = self.imag + other.imag;
        let result = Complex { real, imag };
        // println!("The sum is:{},{}", result.real, result.imag);
        result
    }
    fn mult(&self, other: &Complex) -> Complex {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        let result = Complex { real, imag };
        // println!("The product is:{},{}", result.real, result.imag);
        result
    }

    
    fn div(&self, other: &Complex) -> Result<Complex, String> {
        if other.imag == 0. && other.real==0. {
            Err(String::from("Cannot divide by zero"))
        } else {
            let modulus = other.real * other.real + other.imag * other.imag;
            let real = other.real / modulus;
            let imag = -other.imag / modulus;
            let result = Complex { real, imag };
            // println!("The product is:{},{}", result.real, result.imag);
            Ok(result.mult(&self))
        }
        
    }
}

fn main() {
    let c = Complex { real: 2., imag: 3. };
    let d = Complex { real: 0., imag: 0. };
    println!("The two numbers are:");
    c.print();
    d.print();
    let e = c.add(&d);
    println!("The Sum is:");
    e.print();
    let f = c.mult(&d);
    println!("The product is:");
    f.print();

    
    println!("The division is:");
    match c.div(&d) {
        Ok(result)=>result.print(),
        Err(err)=>println!("{}",err),
    } 
    
    // h.print();
}

struct Complex{
	real:f64,
	imag:f64
}
impl Complex{
	fn print(&self){
		println!("{} + i*{}", self.real, self.imag);
	}
	fn add(&self, other:&Complex)->Complex{
		let real=self.real+other.real;
		let imag=self.imag+other.imag;
		let result=Complex{real,imag};
		// println!("The sum is:{},{}", result.real, result.imag);
		result
	}
	fn mult(&self, other:&Complex)->Complex{
			let real=self.real*other.real-self.imag*other.imag;
			let imag=self.real*other.imag+self.imag*other.real;
			let result=Complex{real,imag};
			// println!("The product is:{},{}", result.real, result.imag);
			result
		}
}

fn main(){
	let c = Complex{real:2.,imag:3.};
	let d= Complex{real:2.,imag:34.};
	println!("The two numbers are:");
	c.print();
	d.print();
	let e=c.add(&d);
	println!("The Sum is:");
	e.print();
	let f=c.mult(&d);
	println!("The product is:");
	f.print();
}

// https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display.html
use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

// Implement Display for Complex
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

pub fn main() {
    let num = Complex { real:3.3, imag: 7.2 };
    println!("Display: {}", num);
    println!("Debug: {:?}", num);
}

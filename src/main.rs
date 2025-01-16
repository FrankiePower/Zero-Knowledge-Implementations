//use the sparse and the dense representation of polynomials
struct Monomial {
    coefficient: usize,
    degree: usize,
}

impl Monomial {
    fn new(coefficient: usize, degree: usize) -> Self {
        Monomial {
            coefficient,
            degree,
        }
    }

    fn evaluate(&self, x: usize) -> usize {
        // evaluate coefficient * x^degree
        self.coefficient * x.pow(self.degree as u32)
    }

    fn degree(&self) -> usize {
        self.degree
    }
}

fn main() {
    let poly = Monomial::new(3, 2);
    let result = poly.evaluate(5);
    println!("The polynomial evaluated at x=5 is: {}", result);
    println!("The degree of the polynomial is: {}", poly.degree());
}

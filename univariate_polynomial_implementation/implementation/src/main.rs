// implementation with sparse rep
// implementation with dense rep
// use generics too
// evaluate with for loop and without

struct PolynomialTerm {
    coefficient: usize,
    degree: usize,
}

fn main() {
    let poly = PolynomialTerm {
        coefficient: 3,
        degree: 2,
    }; // 3x^2

    // let poly = Monomial::new(3, 2);

    let result = poly.evaluate(5);

    println!("The polynomial evaluated at x = 5 is: {}", result);

    println!("The degree of the polynomial is: {}", poly.degree());
}

fn evaluate(self, x: usize) -> usize {
    // evaluate coefficient * x^degree
    self.coefficient * x.pow(self.degree as u32)
}

impl PolynomialTerm {
    fn degree(self) -> usize {
        self.degree
    }

    fn evaluate(self, x: usize) -> usize {
        self.coefficient * x.pow(self.degree as u32)
    }
}

/*

struct UnivariatePoly {
    // 1 coeffiecient for each power of x
    coefficient: Vec<f64>,
}

impl UnivariatePoly {
    fn degree(&self) -> usize {
        self.coefficient.len() - 1
    }

    fn evaluate(&self, x: f64) -> f64 [

    ]
}

    // Dense Representation of Polynomials

// 2 functions (evaluate and interpolate)

// f(x) = y

// [1,2], [2,3]

// how do we represent 5 , 2x + 5 , 3x^2 + 2x + 5

struct Monomial {
    coefficient: f64,
    degree: usize,
}

 */

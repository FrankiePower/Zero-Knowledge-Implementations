fn main() {
    // dense representation of a polynomial

    // Polynomial: 5x^3 + 0x^2 + 2x + 7

    let dense_poly = vec![7, 2, 0, 5];

    // Accessing the coefficient of x^2
    println!("Coefficient of x^2: {}", dense_poly[2]);

    // Printing the polynomial
    for (exp, coeff) in dense_poly.iter().enumerate() {
        if *coeff != 0 {
            print!("{}x^{} ", coeff, exp);
        }
    }

    println!();

    // sparse representation of a polynomial

    sparse();
}

fn sparse() {
    // Polynomial: 5x^3 + 2x + 7
    let sparse_poly = vec![(3, 5), (1, 2), (0, 7)];

    // Accessing the coefficient of x^2 (if it exists)
    let coeff = sparse_poly
        .iter()
        .find(|&&(exp, _)| exp == 2)
        .map(|&(_, coeff)| coeff)
        .unwrap_or(0);
    println!("Coefficient of x^2: {}", coeff);

    // Printing the polynomial
    for (exp, coeff) in &sparse_poly {
        println!("{}x^{}", coeff, exp);
    }
}

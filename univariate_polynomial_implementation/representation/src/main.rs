fn main() {
    dense();
    println!();
    println!();
    sparse();
    println!();
}

fn dense() {
    // Dense representation of a polynomial i.e. The dense representation uses an array (or list) where each index corresponds to a power of x, and the value at that index is the coefficient for that power. This format assumes all powers up to the highest degree are present, even if some coefficients are zero.

    // Polynomial: 5x^3 + 0x^2 + 2x + 7

    let dense_poly = vec![7, 2, 0, 5];

    // Accessing the coefficient of x^2
    println!("Dense Representation");

    println!("Coefficient of x^2: {}", dense_poly[2]);

    // Printing the polynomial
    for (exp, coeff) in dense_poly.iter().enumerate().rev() {
        if *coeff != 0 {
            if exp != 0 {
                print!("{}x^{} + ", coeff, exp);
            } else {
                print!("{}", coeff);
            }
        }
    }
}

fn sparse() {
    // The sparse representation only stores non-zero coefficients along with their corresponding powers (exponents), typically as a list of tuples or a dictionary.Each tuple/dictionary entry contains an exponent and its coefficient.

    // Polynomial: 5x^3 + 2x + 7

    let sparse_poly = vec![(5, 3), (2, 1), (7, 0)];

    // Accessing the coefficient of x^2 (if it exists)

    let coeff = sparse_poly
        .iter()
        .find(|&&(_, exp)| exp == 3)
        .map(|&(coeff, _)| coeff)
        .unwrap_or(0);

    println!("Sparse Representation");

    println!("Coefficient of x^3: {}", coeff);

    // Printing the polynomial
    for (coeff, exp) in sparse_poly.iter() {
        if *coeff != 0 {
            if *exp != 0 {
                print!("{}x^{} + ", coeff, exp);
            } else {
                print!("{}", coeff);
            }
        }
    }
}

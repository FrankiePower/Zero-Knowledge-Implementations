use ark_bls12_381::Fr;
use implementation::sparse_impl::SparseUnivariatePoly;

fn main() {
    let two_degree_poly = create_two_degree_poly();
    let five_degree_poly = create_five_degree_poly();

    println!("Two degree polynomial:");
    print_degree(&two_degree_poly);

    println!("\nFive degree polynomial:");
    print_degree(&five_degree_poly);

    println!("\nTwo degree polynomial evaluated:");

    print_poly_evaluation(&two_degree_poly);

    println!("\nFive degree polynomial evaluated:");

    print_poly_evaluation(&five_degree_poly);
}

fn create_two_degree_poly() -> SparseUnivariatePoly<Fr> {
    let mut sparse_poly = SparseUnivariatePoly::<Fr>::new();

    // 3x^2 + 2x - 5
    sparse_poly.insert_term(2, Fr::from(3u64)); // 3x^2
    sparse_poly.insert_term(1, Fr::from(2u64)); // 2x
    sparse_poly.insert_term(0, -Fr::from(5u64)); // -5

    sparse_poly
}

fn create_five_degree_poly() -> SparseUnivariatePoly<Fr> {
    let mut sparse_poly = SparseUnivariatePoly::<Fr>::new();

    // 4x^5 + 3x^4 - 2x^3 + x^2 - 6x + 7
    sparse_poly.insert_term(5, Fr::from(4u64)); // 4x^5
    sparse_poly.insert_term(4, Fr::from(3u64)); // 3x^4
    sparse_poly.insert_term(3, -Fr::from(2u64)); // -2x^3
    sparse_poly.insert_term(2, Fr::from(1u64)); // x^2
    sparse_poly.insert_term(1, -Fr::from(6u64)); // -6x
    sparse_poly.insert_term(0, Fr::from(7u64)); // 7

    sparse_poly
}

fn print_degree(poly: &SparseUnivariatePoly<Fr>) {
    match poly.degree() {
        Some(d) => println!("Degree of the polynomial: {}", d),
        None => println!("Polynomial is empty"),
    }
}

fn print_poly_evaluation(poly: &SparseUnivariatePoly<Fr>) {
    let value = Fr::from(2u64);
    let result = poly.evaluate(value);
    println!("Polynomial evaluated at {}: {}", value, result);
}

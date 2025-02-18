use ark_bls12_381::Fr;
use implementation::sparse_impl::SparseUnivariatePoly;
fn main() {
    let mut sparse_poly = SparseUnivariatePoly::<Fr>::new();

    sparse_poly.insert_term(2, Fr::from(3u64)); // 3x^2
    sparse_poly.insert_term(1, Fr::from(2u64)); // 2x
    sparse_poly.insert_term(0, Fr::from(5u64)); // 5

    match sparse_poly.degree() {
        Some(d) => println!("Degree of the polynomial: {}", d),
        None => println!("Polynomial is empty"),
    }
}

use ark_bn254::Fq;
use polynomial_utils::interpolation::lagrange_interpolate;


#[test]
fn test_lagrange_interpolation() {
    let x_vals = vec![Fq::from(1), Fq::from(2), Fq::from(3)];
    let y_vals = vec![Fq::from(6), Fq::from(11), Fq::from(18)];

    let poly = lagrange_interpolate(&x_vals, &y_vals);

    assert_eq!(
        poly.coefficients,
        vec![Fq::from(3), Fq::from(2), Fq::from(1)]
    );
}

use ark_bn254::Fr;
//use ark_ff::PrimeField;
use ark_std::{rand, UniformRand};
//use rand::Rng;

pub fn encode_location(lt: f64, ln: f64) -> (Fr, Fr) {
    // scale the longitude and latitude to integers
    let lt = (lt * 1e6) as i64;
    let ln = (ln * 1e6) as i64;

    // convert the integers to field elements
    let lt_fr = Fr::from(lt);
    let ln_fr = Fr::from(ln);

    (lt_fr, ln_fr)
}

pub fn generate_lt_polynomial(lt_secret: Fr) -> Vec<Fr> {
    let mut rng = ark_std::rand::thread_rng();
    let a1 = Fr::rand(&mut rng);
    let a2 = Fr::rand(&mut rng);

    // Coefficients for f(x) = a0 + a1*x + a2*x^2
    vec![lt_secret, a1, a2]
}

pub fn generate_ln_polynomial(ln_secret: Fr) -> Vec<Fr> {
    let mut rng = ark_std::rand::thread_rng();
    let a1 = Fr::rand(&mut rng);
    let a2 = Fr::rand(&mut rng);

    // Coefficients for f(x) = a0 + a1*x + a2*x^2
    vec![ln_secret, a1, a2]
}

pub fn 

fn main() {
    let ln_secret = Fr::from(1780650); // Example: 178.0650 * 10^4
    let ln_poly = generate_ln_polynomial(ln_secret);

    println!("Longitude polynomial coefficients: {:?}", ln_poly);

    let x = Fr::from(0);
    let f_x = ln_poly[0] + ln_poly[1] * x + ln_poly[2] * (x * x);

    println!("f(0) = {:?}", f_x); // Should print 1780650
}

#[cfg(test)]

mod tests {

    use super::*;
    use ark_bn254::Fr;

    #[test]
    fn test_encode_location() {
        let (lt, ln) = encode_location(37.7749, -122.4194);
        assert_eq!(lt, Fr::from(37774900));
        assert_eq!(ln, Fr::from(-122419400));
    }
}

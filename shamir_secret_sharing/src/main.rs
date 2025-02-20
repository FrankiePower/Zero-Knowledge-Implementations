use ark_bn254::Fr;
use ark_ff::PrimeField;

pub fn encode_location(lt: f64, ln: f64) -> (Fr, Fr) {
    // scale the longitude and latitude to integers
    let lt = (lt * 1e6) as i64;
    let ln = (ln * 1e6) as i64;

    // convert the integers to field elements
    let lt_fr = Fr::from(lt);
    let ln_fr = Fr::from(ln);

    (lt_fr, ln_fr)
}

fn main() {
    todo!();
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

use ark_bn254::Fr;
use ark_ff::PrimeField as _;
use ark_std::UniformRand;
use polynomial_utils::{interpolation::lagrange_interpolate, polynomial::DenseUnivariatePoly};

pub fn encode_location(lt: f64, ln: f64) -> (Fr, Fr) {
    // scale the longitude and latitude to integers
    let lt = (lt * 1e3) as i64;
    let ln = (ln * 1e3) as i64;

    // convert the integers to field elements
    let lt_fr = Fr::from(lt);
    let ln_fr = Fr::from(ln);

    (lt_fr, ln_fr)
}

pub fn decode_location(lt: f64, ln: f64) -> (f64, f64) {
    // scale the integers to floating point numbers
    let lt = lt as f64 / 1e3;
    let ln = ln as f64 / 1e3;

    (lt, ln)
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

pub fn generate_shares(poly: &DenseUnivariatePoly<Fr>, num_shares: usize) -> Vec<(Fr, Fr)> {
    let mut shares = Vec::new();

    for i in 1..=num_shares {
        let x = Fr::from(i as u64);
        let y = poly.evaluate(x);
        shares.push((x, y));
    }

    shares
}

fn distribute_shares(shares: Vec<(Fr, Fr)>) {
    let friends = vec![
        "psychemist",
        "kitchens",
        "wolfie",
        "belnades",
        "everythingvee",
        "kcmikee",
        "emarc99",
    ];

    for (i, friend) in friends.iter().enumerate() {
        let (lt_share, ln_share) = &shares[i];
        println!(
            "{}:\n  Latitude Share: {:?}\n  Longitude Share: {:?}\n",
            friend, lt_share, ln_share
        );
    }
}

fn reconstruct_secret(shares: &[(Fr, Fr)]) -> Fr {
    // Use the lagrange interpolation function from the crate
    let x_values: Vec<Fr> = shares.iter().map(|(x, _)| *x).collect();
    let y_values: Vec<Fr> = shares.iter().map(|(_, y)| *y).collect();

    let poly = lagrange_interpolate(&x_values, &y_values);

    poly.evaluate(Fr::from(0))
}

fn primefield_to_f64(field_element: &Fr) -> f64 {
    let num = field_element.into_bigint(); // Convert to a representation
    num.to_string().parse::<f64>().unwrap() // Convert BigInt to f64
}
fn main() {
    // the latitude and longitude of Fiji
    let fiji_lt = 17.7134;
    let fiji_ln = 178.0650;

    // encode the location
    let (lt, ln) = encode_location(fiji_lt, fiji_ln);

    let lt_secret = Fr::from(lt); // Example latitude secret
    let ln_secret = Fr::from(ln); // Example longitude secret

    println!(
        "Encoded Latitude Secret: {:?}\nEncoded Longitude Secret: {:?}",
        lt_secret, ln_secret
    );

    let lt_poly = generate_lt_polynomial(lt_secret); // Latitude polynomial
    let ln_poly = generate_ln_polynomial(ln_secret); // Longitude polynomial

    let lt_poly = DenseUnivariatePoly::new(lt_poly);
    let ln_poly = DenseUnivariatePoly::new(ln_poly);

    let lt_shares = generate_shares(&lt_poly, 7);
    let ln_shares = generate_shares(&ln_poly, 7);
    // Combine the latitude and longitude shares
    let combined_shares: Vec<(Fr, Fr)> = lt_shares
        .clone()
        .into_iter()
        .zip(ln_shares.clone().into_iter())
        .map(|(lt_share, ln_share)| (lt_share.1, ln_share.1)) // Using .1 to get the y-values
        .collect();

    // Distribute shares to friends
    println!("\nDistributing Shares:\n");
    distribute_shares(combined_shares.clone());

    // Let's pick 3 random shares (for testing)
    let random_lat_shares = &lt_shares[0..3];
    let random_ln_shares = &ln_shares[0..3];

    // Reconstruct the secrets using Lagrange interpolation
    let reconstructed_lt_secret = reconstruct_secret(random_lat_shares);
    let reconstructed_ln_secret = reconstruct_secret(random_ln_shares);

    // Convert the reconstructed secret back to a floating-point value for verification
    let reconstructed_lat = primefield_to_f64(&reconstructed_lt_secret);
    let reconstructed_ln = primefield_to_f64(&reconstructed_ln_secret);

    let (decoded_lat, decoded_ln) = decode_location(reconstructed_lat, reconstructed_ln);

    println!(
        "\nReconstructed Latitude: {}\nReconstructed Longitude: {}",
        decoded_lat, decoded_ln
    );
}

use ark_bn254::Fq;
use ark_ff::{BigInteger, PrimeField};
use sha3::{Digest, Keccak256};

#[derive(Clone)]
pub struct Transcript {
    hasher: Keccak256,
}

impl Transcript {
    pub fn new() -> Self {
        Self {
            hasher: Keccak256::new(),
        }
    }

    pub fn append(&mut self, preimage: &[u8]) {
        self.hasher.update(preimage);
    }

    pub fn get_random_challenge<F: PrimeField>(&mut self) -> F {
        let random_challenge = self.hasher.finalize_reset();
        self.append(&random_challenge);
        F::from_le_bytes_mod_order(&random_challenge)
    }
}

// Converts a vector of field elements into a byte array
pub fn fq_vec_to_bytes(values: &[Fq]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|x| x.into_bigint().to_bytes_le()) // Converts each field element to little-endian bytes
        .collect()
}

#[cfg(test)]
mod test {
    use super::Transcript;
    use ark_bn254::Fq;

    #[test]
    fn it_hashes() {
        let mut transcript: Transcript = Transcript::new(); // Initializes a new transcript
        transcript.append("zero knowledge".as_bytes()); // Appends the string to the transcript
        let random_challenge: Fq = transcript.get_random_challenge();
        println!("{:?}", random_challenge);
    }

    #[test]
    fn example_usage() {
        let mut transcript: Transcript = Transcript::new(); // Initializes a new transcript
        transcript.append("example data".as_bytes()); // Appends the string to the transcript
        let random_challenge: Fq = transcript.get_random_challenge();
        println!("{:?}", random_challenge);
    }
}

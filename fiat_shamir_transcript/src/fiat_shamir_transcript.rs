use ark_bn254::Fq;
use ark_ff::{BigInteger, PrimeField};
use sha3::{Digest, Keccak256};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Transcript<F: PrimeField> {
    _field: PhantomData<F>, // PhantomData ensures type safety without storing actual data
    hasher: Keccak256,      // Hasher instance for accumulating transcript data
}

impl<F: PrimeField> Transcript<F> {
    pub fn new() -> Self {
        Self {
            _field: PhantomData,
            hasher: Keccak256::new(),
        }
    }

    pub fn append(&mut self, preimage: &[u8]) {
        self.hasher.update(preimage) // Updates the  hash state with input data
    }

    pub fn get_random_challenge(&mut self) -> F {
        let random_challenge = self.hasher.finalize_reset(); // Computes the hash and resets state
        self.append(&random_challenge); // Re-appends the hash output to maintain continuity
        F::from_le_bytes_mod_order(&random_challenge) // Converts hash output into a field element
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
        let mut transcript: Transcript<Fq> = Transcript::new(); // Initializes a new transcript
        transcript.append("zero knowledge".as_bytes()); // Appends the string to the transcript
        let random_challenge = transcript.get_random_challenge();
        dbg!(random_challenge);
    }
}

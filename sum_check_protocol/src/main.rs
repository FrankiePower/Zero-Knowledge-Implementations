use ark_bls12_381::Fq;
use ark_ff::{BigInteger, PrimeField};
use fiat_shamir_transcript::fiat_shamir_transcript::Transcript;
use multilinear_polynomial::multilinear_polynomial_evaluation::MultilinearPoly;

pub struct SumcheckProver<F: PrimeField> {
    poly: MultilinearPoly<F>,
    round: usize,
    num_vars: usize,
    transcript: Transcript<F>, // Keep track of interaction history
}

impl<F: PrimeField> SumcheckProver<F> {
    pub fn new(poly: MultilinearPoly<F>) -> Self {
        let num_vars = poly.num_of_vars;
        let mut transcript = Transcript::new();

        // Commit to the first coefficient (part of Fiat-Shamir setup)
        let mut commitment = b"poly_commitment".to_vec();
        commitment.extend_from_slice(&poly.evaluation[0].into_bigint().to_bytes_le());
        transcript.append(&commitment);

        Self {
            poly,
            round: 0,
            num_vars,
            transcript,
        }
    }

    /// 🔹 Generates a Fiat-Shamir challenge using the transcript
    fn generate_challenge(&mut self) -> F {
        let challenge_bytes = self.transcript.challenge(b"round_challenge");
        F::from_le_bytes_mod_order(&challenge_bytes)
    }

    pub fn next_round(&mut self, verifier_challenge: Option<F>) -> Option<F> {
        if self.round >= self.num_vars {
            return None;
        }

        // Use Fiat-Shamir to generate challenge if verifier doesn't provide one
        let challenge = verifier_challenge.unwrap_or_else(|| self.generate_challenge());

        let g1_x = self.poly.partial_evaluate(self.round, &challenge);

        // Update state
        self.poly = g1_x;
        self.round += 1;

        // Return proof for the verifier
        Some(self.poly.evaluation[0])
    }
}

fn main() {
    // Multilinear polynomial: 2ab + 3bc
    let evaluations = vec![
        Fq::from(0), // (0,0,0)
        Fq::from(0), // (0,0,1)
        Fq::from(0), // (0,1,0)
        Fq::from(3), // (0,1,1)
        Fq::from(0), // (1,0,0)
        Fq::from(0), // (1,0,1)
        Fq::from(2), // (1,1,0)
        Fq::from(5), // (1,1,1)
    ];
    let poly = MultilinearPoly::new(evaluations);
    let mut prover = SumcheckProver::new(poly);

    println!("Starting Sumcheck Protocol...");
    let mut challenge = None;

    for round in 0..prover.num_vars {
        let proof = prover.next_round(challenge);
        match proof {
            Some(value) => {
                println!("Round {}: Prover sends proof = {}", round + 1, value);
                // Now using Fiat-Shamir for challenge generation
                challenge = Some(value);
            }
            None => {
                println!("Sumcheck protocol completed.");
                break;
            }
        }
    }
}

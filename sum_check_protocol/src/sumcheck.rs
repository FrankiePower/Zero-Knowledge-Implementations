use ark_ff::{BigInteger, PrimeField};
use fiat_shamir_transcript::fiat_shamir_transcript::Transcript;
use multilinear_polynomial::multilinear_polynomial_evaluation::MultilinearPoly;
use std::marker::PhantomData;

pub struct Prover<F: PrimeField> {
    pub initial_polynomial: MultilinearPoly<F>,
    pub initial_claimed_sum: F,
    pub transcript: Transcript,
    pub round_univariate_polynomials: Vec<MultilinearPoly<F>>,
    pub is_initialized: bool,
}

pub struct SumcheckProof<F: PrimeField> {
    pub initial_polynomial: MultilinearPoly<F>,
    pub initial_claimed_sum: F,
    pub round_univariate_polynomials: Vec<MultilinearPoly<F>>,
}

impl<F: PrimeField> Prover<F> {
    /// Initializes a prover with a multilinear polynomial's evaluated values.
    /// Panics if the length of evaluated_values is not a power of 2.
    pub fn init(multilinear_polynomial_evaluation: &Vec<F>) -> Self {
        assert!(
            multilinear_polynomial_evaluation.len().is_power_of_two(),
            "Polynomial evaluation length must be a power of 2"
        );
        let polynomial = MultilinearPoly::new(multilinear_polynomial_evaluation.clone());
        let transcript = Transcript::new();

        Prover {
            initial_polynomial: polynomial,
            initial_claimed_sum: multilinear_polynomial_evaluation.iter().sum(),
            transcript,
            round_univariate_polynomials: Vec::new(),
            is_initialized: true,
        }
    }

    /// Generates a Sumcheck proof by iteratively reducing the polynomial to univariate polynomials.
    pub fn prove(&mut self) -> SumcheckProof<F> {
        assert!(self.is_initialized, "Can't prove without init");

        self.transcript
            .append(&self.initial_polynomial.convert_to_bytes());
        self.transcript
            .append(&field_element_to_bytes(self.initial_claimed_sum));

        // Wrap `current_polynomial` in a `MultilinearPoly`
        let mut current_polynomial = MultilinearPoly {
            evaluation: self.initial_polynomial.evaluation.clone(),
            num_of_vars: self.initial_polynomial.num_of_vars,
        };

        for _ in 0..self.initial_polynomial.number_of_variables() {
            let univariate_polynomial_values =
                split_polynomial_and_sum_each(&current_polynomial.evaluation);
            let univariate_polynomial = MultilinearPoly::new(univariate_polynomial_values.clone());
            let univariate_poly_in_bytes = univariate_polynomial.convert_to_bytes();
            self.round_univariate_polynomials
                .push(univariate_polynomial);
            self.transcript.append(&univariate_poly_in_bytes);

            let random_challenge: F = self.transcript.get_random_challenge();
            current_polynomial = current_polynomial.partial_evaluate(0, &random_challenge);
        }

        SumcheckProof {
            initial_polynomial: self.initial_polynomial.clone(),
            initial_claimed_sum: self.initial_claimed_sum,
            round_univariate_polynomials: self.round_univariate_polynomials.clone(),
        }
    }
}

/// Splits a polynomial's evaluated values into two halves and sums each half.
/// Returns a univariate polynomial’s evaluations at 0 and 1.
pub fn split_polynomial_and_sum_each<F: PrimeField>(
    polynomial_evaluated_values: &Vec<F>,
) -> Vec<F> {
    let mut univariate_polynomial: Vec<F> = Vec::with_capacity(2);

    let mid = polynomial_evaluated_values.len() / 2;
    let (left, right) = polynomial_evaluated_values.split_at(mid);

    let left_sum: F = left.iter().sum();
    let right_sum: F = right.iter().sum();

    univariate_polynomial.push(left_sum);
    univariate_polynomial.push(right_sum);

    univariate_polynomial
}

pub fn field_element_to_bytes<F: PrimeField>(field_element: F) -> Vec<u8> {
    field_element.into_bigint().to_bytes_be()
}

pub struct Verifier<F: PrimeField> {
    pub transcript: Transcript,
    pub is_initialized: bool,
    _phantom: PhantomData<F>,
}

impl<F: PrimeField> Verifier<F> {
    pub fn init() -> Self {
        Self {
            transcript: Transcript::new(),
            is_initialized: true,
            _phantom: PhantomData,
        }
    }

    /// Verifies a Sumcheck proof by checking consistency of sums and final evaluation.
    pub fn verify(&mut self, proof: SumcheckProof<F>) -> bool {
        assert!(self.is_initialized, "Can't verify without init");

        if proof.round_univariate_polynomials.len()
            != proof.initial_polynomial.number_of_variables() as usize
        {
            return false;
        }

        let mut current_claim_sum = proof.initial_claimed_sum;

        self.transcript
            .append(&proof.initial_polynomial.convert_to_bytes());
        self.transcript
            .append(&field_element_to_bytes(proof.initial_claimed_sum));

        let mut challenges: Vec<F> = Vec::with_capacity(proof.round_univariate_polynomials.len());

        for i in 0..proof.round_univariate_polynomials.len() {
            let eval_at_zero = vec![F::zero()];
            let eval_at_one = vec![F::one()];

            if proof.round_univariate_polynomials[i].evaluate(eval_at_zero.clone())
                + proof.round_univariate_polynomials[i].evaluate(eval_at_one.clone())
                != current_claim_sum
            {
                return false;
            }

            self.transcript
                .append(&proof.round_univariate_polynomials[i].convert_to_bytes());

            let challenge: F = self.transcript.get_random_challenge();
            challenges.push(challenge);

            current_claim_sum =
                proof.round_univariate_polynomials[i].evaluate(vec![challenge].clone());
        }

        let final_evaluation = proof.initial_polynomial.evaluate(challenges);
        final_evaluation == current_claim_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_prover_init() {
        let evaluated_values = vec![Fq::from(0), Fq::from(0), Fq::from(3), Fq::from(8)];
        let prover = Prover::init(&evaluated_values);

        assert_eq!(prover.initial_claimed_sum, Fq::from(11));
        assert_eq!(prover.initial_polynomial.evaluation, evaluated_values);
    }

    #[test]
    fn test_sumcheck_roundtrip() {
        let evaluated_values = vec![Fq::from(0), Fq::from(0), Fq::from(3), Fq::from(8)];
        let mut prover = Prover::init(&evaluated_values);
        let proof = prover.prove();

        let mut verifier = Verifier::init();
        assert!(verifier.verify(proof), "Sumcheck proof verification failed");
    }

    #[test]
    #[should_panic(expected = "Polynomial evaluation length must be a power of 2")]
    fn test_invalid_length() {
        let evaluated_values = vec![Fq::from(1), Fq::from(2), Fq::from(3)]; // Not a power of 2
        let _ = Prover::init(&evaluated_values);
    }
}

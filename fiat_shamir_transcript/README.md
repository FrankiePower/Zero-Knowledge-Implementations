# Fiat-Shamir Heuristic

The **Fiat-Shamir heuristic** is a cryptographic technique used to transform an **interactive proof system** into a **non-interactive proof**. This is particularly useful in reducing communication overhead in cryptographic protocols.

## How It Works

Fiat-Shamir replaces the verifier's role in an interactive proof with a deterministic, publicly known **hash function**. The prover generates challenges on their own by hashing their previous responses, making the proof non-interactive.

### **1. Original Interactive Proof**

- The prover sends an initial commitment to the verifier.
- The verifier randomly selects a challenge.
- The prover responds to the challenge.
- The verifier checks whether the response is valid.

### **2. Fiat-Shamir Transformation (Non-Interactive Proof)**

- The prover generates a challenge by applying a cryptographic hash function (e.g., SHA-256) to the commitment.
- The prover computes a response based on the hash-generated challenge.
- The verifier checks the proof using only public data.

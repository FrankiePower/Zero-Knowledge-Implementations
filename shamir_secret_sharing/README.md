# Shamir's Secret Sharing with Lagrange Interpolation in Rust

This project demonstrates the implementation of Shamir's Secret Sharing Scheme (SSSS) using Rust, with a focus on encoding geographical coordinates (latitude and longitude). The secret sharing process is powered by Lagrange Interpolation, which allows the reconstruction of the secret from a minimum threshold of shares.

## Problem Overview

Shamir's Secret Sharing enables a secret (such as a geographical coordinate) to be split into multiple shares. To retrieve the secret, only a subset of the shares, determined by a threshold value, is required. In this case, the secret represents the latitude and longitude of a location.

For this implementation, the latitude and longitude of Fiji (17.7134° S, 178.0650° E) are encoded as integers, split into shares, and reconstructed using Lagrange interpolation.

## Key Features

- **Encoding and Decoding**: Geographical coordinates are encoded into field elements using scaling and then distributed as shares.
- **Share Generation**: Generates 7 shares for each coordinate (latitude and longitude) using Shamir's Secret Sharing scheme.
- **Lagrange Interpolation**: The secret is reconstructed using Lagrange interpolation, which ensures that any 3 out of the 7 shares can recover the original secret.
- **Security and Usability**: The system uses large integers for security, but they are scaled down to smaller values for easier memorization.

## How it Works

1. **Encode Latitude and Longitude**: The latitude and longitude of the location are scaled and encoded into finite field elements.
2. **Generate Polynomials**: Using the encoded coordinates, polynomials are generated to represent the secrets.
3. **Generate Shares**: The polynomials are evaluated at different points to produce shares. Each share is associated with an x-value and the corresponding y-value.
4. **Distribute Shares**: The shares are distributed to different participants (e.g., friends).
5. **Reconstruction**: A minimum of 3 shares (for both latitude and longitude) are required to reconstruct the original coordinates using Lagrange interpolation.

## Example Usage

```rust
// Encode coordinates
let (lt, ln) = encode_location(17.7134, 178.0650);

// Generate shares
let shares = generate_shares(lt, ln, 7, 3);

// Reconstruct secret (using any 3 shares)
let (decoded_lt, decoded_ln) = reconstruct_secret(&shares[0..3]);

// Decode coordinates
let (latitude, longitude) = decode_location(decoded_lt, decoded_ln);
```

## Benefits

- **Privacy**: The secret (latitude and longitude) can only be reconstructed when enough shares are combined.
- **Scalability**: The number of shares and the threshold can be easily adjusted based on the use case.
- **Security**: By using large field elements, the security of the secret is ensured, while still making it feasible to scale down the numbers for practical use.

## Installation

I created a polynomial utility crate,

```bash
   https://github.com/FrankiePower/Zero-Knowledge-Implementations/tree/main/polynomial-utils

```

Add this to your `Cargo.toml`:

## How to Run

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/shamir-secret-sharing
   ```

2. Build and run the project:

   ```bash
   cargo run
   ```

3. Run the tests:
   ```bash
   cargo test
   ```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

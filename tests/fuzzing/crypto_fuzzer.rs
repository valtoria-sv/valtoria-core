use arbitrary::Arbitrary;
use fuzzcheck::fuzz_test;

use your_project_name::crypto::{generate_keys, hash, sign};

// Struct to represent fuzzed data for hashing
#[derive(Arbitrary, Debug)]
struct FuzzData {
    data: Vec<u8>,
}

// Fuzz test for hashing function
#[fuzz_test]
fn fuzz_hash(data: FuzzData) {
    // Call the hash function with fuzzed byte data
    let result = hash(&data.data);
    assert!(!result.is_empty(), "Hash result should not be empty");
}

// Fuzz test for signature generation
#[fuzz_test]
fn fuzz_signature() {
    // Generate a pair of keys for signing
    let (private_key, public_key) = generate_keys();

    // Fuzz the input data
    let input_data: FuzzData = FuzzData::arbitrary();

    // Call the sign function with fuzzed data
    let signature = sign(&private_key, &input_data.data);

    // Check if the signature is valid
    assert!(!signature.is_empty(), "Signature should not be empty");

    // Optionally verify the signature
    let is_verified = your_project_name::crypto::verify(&public_key, &input_data.data, &signature);
    assert!(is_verified, "Signature verification failed");
}

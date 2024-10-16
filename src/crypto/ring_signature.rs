use super::keys::{PrivateKey, PublicKey};
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use sha2::{Digest, Sha512};

pub struct RingSignature {
    pub c: Vec<Scalar>,
    pub r: Vec<Scalar>,
}

pub fn sign(
    message: &[u8],
    public_keys: &[PublicKey],
    secret_key: &PrivateKey,
    secret_index: usize,
) -> RingSignature {
    let n = public_keys.len();
    let mut c = vec![Scalar::zero(); n];
    let mut r = vec![Scalar::zero(); n];

    // Initialize random scalar
    let mut rng = rand::thread_rng();
    let q = Scalar::random(&mut rng);

    // Compute key image
    let key_image = RistrettoPoint::scalar_mul(&public_keys[secret_index].0, &secret_key.0);

    // Generate random scalars for all other ring members
    for i in 0..n {
        if i != secret_index {
            r[i] = Scalar::random(&mut rng);
        }
    }

    // Compute c[1]
    let mut hasher = Sha512::new();
    hasher.update(message);
    hasher.update(key_image.compress().as_bytes());
    c[0] = Scalar::from_hash(hasher);

    // Compute remaining c and r values
    for i in 1..n {
        let index = (secret_index + i) % n;
        let point = RistrettoPoint::scalar_mul(&public_keys[index].0, &r[index])
            + RistrettoPoint::scalar_mul(&RistrettoPoint::default(), &c[(i - 1) % n]);

        let mut hasher = Sha512::new();
        hasher.update(message);
        hasher.update(point.compress().as_bytes());
        c[i % n] = Scalar::from_hash(hasher);
    }

    // Compute r[secret_index]
    r[secret_index] = q - c[secret_index] * secret_key.0;

    RingSignature { c, r }
}

pub fn verify(message: &[u8], public_keys: &[PublicKey], signature: &RingSignature) -> bool {
    let n = public_keys.len();
    if n != signature.c.len() || n != signature.r.len() {
        return false;
    }

    let mut c_verify = vec![Scalar::zero(); n];

    for i in 0..n {
        let point = RistrettoPoint::scalar_mul(&public_keys[i].0, &signature.r[i])
            + RistrettoPoint::scalar_mul(&RistrettoPoint::default(), &signature.c[i]);

        let mut hasher = Sha512::new();
        hasher.update(message);
        hasher.update(point.compress().as_bytes());
        c_verify[(i + 1) % n] = Scalar::from_hash(hasher);
    }

    c_verify == signature.c
}

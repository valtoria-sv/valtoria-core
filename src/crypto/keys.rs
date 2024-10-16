use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;

#[derive(Clone, Debug)]
pub struct PrivateKey(pub(crate) Scalar);

#[derive(Clone, Debug)]
pub struct PublicKey(pub(crate) RistrettoPoint);

#[derive(Clone, Debug)]
pub struct KeyPair {
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let private_key = PrivateKey(Scalar::random(&mut OsRng));
        let public_key = PublicKey(RistrettoPoint::scalar_mul(
            &RistrettoPoint::default(),
            &private_key.0,
        ));

        KeyPair {
            private_key,
            public_key,
        }
    }
}

use super::keys::{KeyPair, PrivateKey, PublicKey};
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;

pub struct StealthAddress {
    pub view_public: PublicKey,
    pub spend_public: PublicKey,
}

pub fn generate_stealth_address() -> (StealthAddress, PrivateKey, PrivateKey) {
    let view_keypair = KeyPair::generate();
    let spend_keypair = KeyPair::generate();

    let stealth_address = StealthAddress {
        view_public: view_keypair.public_key,
        spend_public: spend_keypair.public_key,
    };

    (
        stealth_address,
        view_keypair.private_key,
        spend_keypair.private_key,
    )
}

pub fn create_destination(stealth_address: &StealthAddress) -> (PublicKey, Scalar) {
    let r = Scalar::random(&mut OsRng);
    let R = RistrettoPoint::scalar_mul(&RistrettoPoint::default(), &r);

    let destination_key = RistrettoPoint::scalar_mul(&stealth_address.view_public.0, &r)
        + stealth_address.spend_public.0;

    (PublicKey(destination_key), r)
}

pub fn recover_destination(
    r: &Scalar,
    view_private: &PrivateKey,
    spend_public: &PublicKey,
) -> PublicKey {
    let shared_secret =
        RistrettoPoint::scalar_mul(&RistrettoPoint::default(), &(r * view_private.0));
    let destination_key = shared_secret + spend_public.0;

    PublicKey(destination_key)
}

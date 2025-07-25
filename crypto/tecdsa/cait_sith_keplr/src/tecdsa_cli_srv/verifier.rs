use ecdsa::Signature;
use elliptic_curve::AffinePoint;
use k256::ecdsa::signature::Verifier;
use k256::ecdsa::VerifyingKey;
use k256::{PublicKey, Secp256k1};

use crate::protocol::ProtocolError;

pub fn verify_sig(
    sig: Signature<Secp256k1>,
    public_key: AffinePoint<Secp256k1>,
    msg: &[u8],
) -> Result<(), ProtocolError> {
    VerifyingKey::from(&PublicKey::from_affine(public_key).unwrap())
        .verify(&msg[..], &sig)
        .map_err(|err| {
            ProtocolError::Other(format!("Error verifying the sig, err: {:?}", err).into())
        })
}

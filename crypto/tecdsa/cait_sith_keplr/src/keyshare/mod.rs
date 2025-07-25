mod state;
mod state_2;
pub mod steps;
pub mod steps_2;

pub use state::*;
pub use state_2::*;

use serde::{Deserialize, Serialize};

use crate::compat::CSCurve;

const LABEL: &[u8] = b"cait-sith v0.8.0 keygen";

/// Represents the output of the key generation protocol.
///
/// This contains our share of the private key, along with the public key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeygenOutput<C: CSCurve> {
    pub private_share: C::Scalar,
    pub public_key: C::AffinePoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct CentralizedKeygenOutput<C: CSCurve> {
    pub private_key: C::Scalar,
    pub keygen_outputs: Vec<KeygenOutput<C>>,
}

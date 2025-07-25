use serde::{Deserialize, Serialize};

use crate::compat::CSCurve;

/// The output of the presigning protocol.
///
/// This output is basically all the parts of the signature that we can perform
/// without knowing the message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignOutput<C: CSCurve> {
    /// The public nonce commitment.
    pub big_r: C::AffinePoint,
    /// Our share of the nonce value.
    pub k: C::Scalar,
    /// Our share of the sigma value.
    pub sigma: C::Scalar,
}

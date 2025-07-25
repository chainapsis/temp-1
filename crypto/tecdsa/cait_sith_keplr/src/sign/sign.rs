use elliptic_curve::{ops::Invert, Field, Group};
use serde::{Deserialize, Serialize};

use crate::compat::{self, CSCurve};

/// Represents a signature with extra information, to support different variants of ECDSA.
///
/// An ECDSA signature is usually two scalars. The first scalar is derived from
/// a point on the curve, and because this process is lossy, some other variants
/// of ECDSA also include some extra information in order to recover this point.
///
/// Furthermore, some signature formats may disagree on how precisely to serialize
/// different values as bytes.
///
/// To support these variants, this simply gives you a normal signature, along with the entire
/// first point.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullSignature<C: CSCurve> {
    /// This is the entire first point.
    pub big_r: C::AffinePoint,
    /// This is the second scalar, normalized to be in the lower range.
    pub s: C::Scalar,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct SignOutput<C: CSCurve> {
    pub sig: FullSignature<C>,
    pub is_high: bool,
}

impl<C: CSCurve> FullSignature<C> {
    #[must_use]
    pub fn verify(&self, public_key: &C::AffinePoint, msg_hash: &C::Scalar) -> bool {
        let r: C::Scalar = compat::x_coordinate::<C>(&self.big_r);
        if r.is_zero().into() || self.s.is_zero().into() {
            return false;
        }
        let s_inv = self.s.invert_vartime().unwrap();
        let reproduced = (C::ProjectivePoint::generator() * (*msg_hash * s_inv))
            + (C::ProjectivePoint::from(*public_key) * (r * s_inv));
        compat::x_coordinate::<C>(&reproduced.into()) == r
    }
}

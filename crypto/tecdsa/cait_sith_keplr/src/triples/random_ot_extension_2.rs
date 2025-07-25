use ck_meow::Meow;
use elliptic_curve::CurveArithmetic;
use magikitten::MeowRng;
use subtle::Choice;

use crate::{compat::CSCurve, constants::SECURITY_PARAMETER};

use super::bits::BitVector;

const MEOW_CTX: &[u8] = b"Random OT Extension Hash";

pub fn hash_to_scalar<C: CSCurve>(i: usize, v: &BitVector) -> C::Scalar {
    let mut meow = Meow::new(MEOW_CTX);
    let i64 = u64::try_from(i).expect("failed to convert usize to u64");
    meow.meta_ad(&i64.to_le_bytes(), false);
    meow.ad(&v.bytes(), false);
    let mut seed = [0u8; 32];
    meow.prf(&mut seed, false);
    // Could in theory avoid one PRF call by using a more direct RNG wrapper
    // over the prf function, but oh well.
    C::sample_scalar_constant_time(&mut MeowRng::new(&seed))
}

pub fn adjust_size(size: usize) -> usize {
    let r = size % SECURITY_PARAMETER;
    let padded = if r == 0 {
        size
    } else {
        size + (SECURITY_PARAMETER - r)
    };
    padded + 2 * SECURITY_PARAMETER
}

/// Parameters we need for random OT extension
#[derive(Debug, Clone, Copy)]
pub struct RandomOtExtensionParams<'sid> {
    pub sid: &'sid [u8],
    pub batch_size: usize,
}

/// The result that the sender gets.
pub type RandomOTExtensionSenderOut<C> = Vec<(
    <C as CurveArithmetic>::Scalar,
    <C as CurveArithmetic>::Scalar,
)>;

/// The result that the receiver gets.
pub type RandomOTExtensionReceiverOut<C> = Vec<(Choice, <C as CurveArithmetic>::Scalar)>;

use ck_meow::Meow;

use crate::{
    compat::{CSCurve, SerializablePoint},
    serde::encode,
};

use super::bits::{BitVector, SquareBitMatrix, SEC_PARAM_8};

const BATCH_RANDOM_OT_HASH: &[u8] = b"cait-sith v0.8.0 batch ROT";

pub fn hash<C: CSCurve>(
    i: usize,
    big_x_i: &SerializablePoint<C>,
    big_y: &SerializablePoint<C>,
    p: &C::ProjectivePoint,
) -> BitVector {
    let mut meow = Meow::new(BATCH_RANDOM_OT_HASH);
    meow.ad(&(i as u64).to_le_bytes(), false);
    meow.ad(&encode(&big_x_i), false);
    meow.ad(&encode(&big_y), false);
    meow.ad(&encode(&SerializablePoint::<C>::from_projective(p)), false);

    let mut bytes = [0u8; SEC_PARAM_8];
    meow.prf(&mut bytes, false);

    BitVector::from_bytes(&bytes)
}

pub type BatchRandomOTOutputSender = (SquareBitMatrix, SquareBitMatrix);

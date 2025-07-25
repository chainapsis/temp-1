use elliptic_curve::scalar::IsHigh;
use subtle::ConditionallySelectable;

use crate::{
    compat::{self, CSCurve},
    protocol::{Participant, ProtocolError},
};

use super::{
    state::{RcvdSignMessages, SignState},
    FullSignature,
};

pub struct Step1Output<C: CSCurve> {
    pub s_i: C::Scalar,
}

pub fn step_1<C: CSCurve>(
    state: &mut SignState<C>,
    _msgs: &RcvdSignMessages<C>,
    msg_hash: C::Scalar,
    me: Participant,
) -> Result<Step1Output<C>, ProtocolError> {
    // Spec 1.1
    let lambda = state.participants.lagrange::<C>(me);
    let k_i = lambda * state.presignature.k;

    // Spec 1.2
    let sigma_i = lambda * state.presignature.sigma;

    // Spec 1.3
    let r = compat::x_coordinate::<C>(&state.presignature.big_r);
    let s_i: C::Scalar = msg_hash * k_i + r * sigma_i;

    state.s_i = Some(s_i);

    let out = Step1Output { s_i };

    Ok(out)
}

pub fn step_2<C: CSCurve>(
    state: &mut SignState<C>,
    msgs: &RcvdSignMessages<C>,
    msg_hash: C::Scalar,
    me: Participant,
) -> Result<FullSignature<C>, ProtocolError> {
    let mut s = state.s_i.unwrap();

    // // Spec 2.1 + 2.2
    for p in state.participants.others(me) {
        let s_j = *msgs.wait_0.get(&p).unwrap();
        s += C::Scalar::from(s_j);
    }

    // Spec 2.3
    // Optionally, normalize s
    s.conditional_assign(&(-s), s.is_high());
    let sig: FullSignature<C> = FullSignature {
        big_r: state.presignature.big_r,
        s,
    };

    let public_key = state.keygen_out.public_key;

    if !sig.verify(&public_key, &msg_hash) {
        return Err(ProtocolError::AssertionFailed(
            "signature failed to verify".to_string(),
        ));
    }

    Ok(sig)
}

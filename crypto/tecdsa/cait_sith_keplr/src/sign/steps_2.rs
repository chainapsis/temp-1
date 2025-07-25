use elliptic_curve::{scalar::IsHigh, ScalarPrimitive};
use serde::{Deserialize, Serialize};
use subtle::ConditionallySelectable;

use super::{state::RcvdSignMessages, FullSignature, SignOutput, SignState2};
use crate::{
    compat::{self, CSCurve},
    participants::ParticipantList,
    protocol::{Participant, ProtocolError},
    PresignOutput,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Step1Output2<C: CSCurve> {
    pub s_i: ScalarPrimitive<C>,
}

pub fn step_1<C: CSCurve>(
    state: &mut SignState2<C>,
    me: Participant,
    msg_hash: C::Scalar,
    presig_0: PresignOutput<C>,
) -> Result<Step1Output2<C>, ProtocolError> {
    let participants = ParticipantList::new(&state.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    // Spec 1.1
    let lambda = participants.lagrange::<C>(me);
    let k_i = lambda * presig_0.k;

    // // Spec 1.2
    let sigma_i = lambda * presig_0.sigma;

    // // Spec 1.3
    let r = compat::x_coordinate::<C>(&presig_0.big_r);
    let s_i: C::Scalar = msg_hash * k_i + r * sigma_i;

    state.s_i = Some(s_i);

    let out = Step1Output2 { s_i: s_i.into() };

    Ok(out)
}

pub fn step_2_2<C: CSCurve>(
    state: &mut SignState2<C>,
    msgs: &RcvdSignMessages<C>,
    me: Participant,
    presig: PresignOutput<C>,
) -> Result<FullSignature<C>, ProtocolError> {
    let participant_list = ParticipantList::new(&state.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    let mut s = state.s_i.unwrap();

    // // Spec 2.1 + 2.2
    for p in participant_list.others(me) {
        let s_j = *msgs.wait_0.get(&p).unwrap();
        s += C::Scalar::from(s_j);
    }

    // Spec 2.3
    // Optionally, normalize s
    s.conditional_assign(&(-s), s.is_high());
    let sig: FullSignature<C> = FullSignature {
        big_r: presig.big_r,
        s,
    };

    Ok(sig)
}

pub fn step_2<C: CSCurve>(
    state: &mut SignState2<C>,
    msgs: &RcvdSignMessages<C>,
    me: Participant,
    presig: PresignOutput<C>,
) -> Result<SignOutput<C>, ProtocolError> {
    let participant_list = ParticipantList::new(&state.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    let mut s = state.s_i.unwrap();

    for p in participant_list.others(me) {
        let s_j = *msgs.wait_0.get(&p).unwrap();
        s += C::Scalar::from(s_j);
    }

    let was_flipped = s.is_high();
    s.conditional_assign(&(-s), was_flipped);

    let sig: FullSignature<C> = FullSignature {
        big_r: presig.big_r,
        s,
    };

    let out = SignOutput {
        sig,
        is_high: bool::from(was_flipped),
    };

    Ok(out)
}

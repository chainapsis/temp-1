use elliptic_curve::ScalarPrimitive;

use crate::{
    participants::ParticipantList,
    protocol::{Participant, ProtocolError},
    sign::{
        steps::{step_1, step_2},
        RcvdSignMessages, SignState,
    },
    CSCurve, FullSignature, KeygenOutput, PresignOutput,
};

pub fn sign_2<C: CSCurve>(
    participants: &[Participant],
    threshold: usize,
    p_0_keygen_output: KeygenOutput<C>,
    p_1_keygen_output: KeygenOutput<C>,
    p_0_presignature: PresignOutput<C>,
    p_1_presignature: PresignOutput<C>,
    msg_hash: C::Scalar,
) -> Result<Vec<(Participant, FullSignature<C>)>, ProtocolError> {
    if participants.len() < 2 {
        return Err(ProtocolError::Other(
            format!(
                "participant count cannot be < 2, found: {}",
                participants.len()
            )
            .into(),
        ));
    };

    let p_0 = participants.get(0).unwrap();
    let p_1 = participants.get(1).unwrap();

    let participants = ParticipantList::new(participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    let mut state_0 = SignState::new(
        participants.clone(),
        threshold,
        p_0_keygen_output,
        p_0_presignature,
    );
    let mut state_1 = SignState::new(
        participants.clone(),
        threshold,
        p_1_keygen_output,
        p_1_presignature,
    );

    let mut msgs_0 = RcvdSignMessages::new();
    let mut msgs_1 = RcvdSignMessages::new();

    {
        let out_0 = step_1(&mut state_0, &msgs_0, msg_hash, *p_0).unwrap();

        // Spec 1.4

        // p0 => p1
        // send many
        let p_0_s_i: ScalarPrimitive<C> = out_0.s_i.into();
        msgs_1.wait_0.insert(*p_0, p_0_s_i);

        let out_1 = step_1(&mut state_1, &msgs_1, msg_hash, *p_1).unwrap();

        // p1 => p0
        // send many
        let p_1_s_i: ScalarPrimitive<C> = out_1.s_i.into();
        msgs_0.wait_0.insert(*p_1, p_1_s_i);
    }

    let sign_result = {
        let sig_0 = step_2(&mut state_0, &msgs_0, msg_hash, *p_0).unwrap();

        let sig_1 = step_2(&mut state_1, &msgs_1, msg_hash, *p_1).unwrap();

        vec![(*p_0, sig_0), (*p_1, sig_1)]
    };

    Ok(sign_result)
}

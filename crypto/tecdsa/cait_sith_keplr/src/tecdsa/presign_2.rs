use elliptic_curve::ScalarPrimitive;

use crate::{
    participants::ParticipantList,
    presign::{
        steps::{step_1, step_2, step_3},
        PresignState, RcvdPresignMessages,
    },
    protocol::{InitializationError, Participant, ProtocolError},
    triples::{TriplePub, TripleShare},
    CSCurve, KeygenOutput, PresignOutput,
};

pub fn presign_2<C: CSCurve>(
    participants: &Vec<Participant>,
    threshold: usize,
    p_0_triple_0: (TripleShare<C>, TriplePub<C>),
    p_0_triple_1: (TripleShare<C>, TriplePub<C>),
    p_1_triple_0: (TripleShare<C>, TriplePub<C>),
    p_1_triple_1: (TripleShare<C>, TriplePub<C>),
    p_0_keygen_output: KeygenOutput<C>,
    p_1_keygen_output: KeygenOutput<C>,
) -> Result<Vec<(Participant, PresignOutput<C>)>, ProtocolError> {
    if participants.len() < 2 {
        return Err(ProtocolError::Other(
            format!(
                "participant count cannot be < 2, found: {}",
                participants.len()
            )
            .into(),
        ));
    };
    // Spec 1.1
    if threshold > participants.len() {
        return Err(ProtocolError::Other(
            "threshold must be <= participant count".to_string().into(),
        ));
    }
    // NOTE: We omit the check that the new participant set was present for
    // the triple generation, because presumably they need to have been present
    // in order to have shares.

    // Also check that we have enough participants to reconstruct shares.
    if threshold != p_0_triple_0.1.threshold || threshold != p_1_triple_1.1.threshold {
        return Err(ProtocolError::Other(
            "New threshold must match the threshold of both triples"
                .to_string()
                .into(),
        ));
    }

    let p_0 = participants.get(0).unwrap();
    let p_1 = participants.get(1).unwrap();

    let participant_list = ParticipantList::new(&participants)
        .ok_or_else(|| {
            InitializationError::BadParameters(
                "participant list cannot contain duplicates".to_string(),
            )
        })
        .unwrap();

    let mut state_0 = PresignState::new(
        participant_list.clone(),
        threshold,
        p_0_keygen_output,
        p_0_triple_0,
        p_0_triple_1,
    );
    let mut state_1 = PresignState::new(
        participant_list,
        threshold,
        p_1_keygen_output,
        p_1_triple_0,
        p_1_triple_1,
    );

    let mut msgs_0 = RcvdPresignMessages::<C>::new();
    let mut msgs_1 = RcvdPresignMessages::<C>::new();

    {
        let out_0 = step_1::<C>(&mut state_0, &msgs_0, *p_0, *p_0).unwrap();

        // p0 => p1
        // send many
        msgs_1.wait_0.insert(*p_0, out_0.kd_i.into());

        let out_1 = step_1::<C>(&mut state_1, &msgs_1, *p_1, *p_1).unwrap();

        // p1 => p0
        // send many
        msgs_0.wait_0.insert(*p_1, out_1.kd_i.into());
    }

    {
        let out_0 = step_2::<C>(&mut state_0, &msgs_0, *p_0).unwrap();

        // p0 => p1
        // send many
        let ka_i: ScalarPrimitive<C> = out_0.ka_i.into();
        let xb_i: ScalarPrimitive<C> = out_0.xb_i.into();
        msgs_1.wait_1.insert(*p_0, (ka_i, xb_i));

        let out_1 = step_2::<C>(&mut state_1, &msgs_1, *p_1).unwrap();

        // p1 => p0
        // send many
        let ka_i: ScalarPrimitive<C> = out_1.ka_i.into();
        let xb_i: ScalarPrimitive<C> = out_1.xb_i.into();
        msgs_0.wait_1.insert(*p_1, (ka_i, xb_i));
    }

    let presign_output = {
        let out_0 = step_3::<C>(&mut state_0, &msgs_0, *p_0).unwrap();

        let out_1 = step_3::<C>(&mut state_1, &msgs_1, *p_1).unwrap();

        vec![(*p_0, out_0), (*p_1, out_1)]
    };

    Ok(presign_output)
}

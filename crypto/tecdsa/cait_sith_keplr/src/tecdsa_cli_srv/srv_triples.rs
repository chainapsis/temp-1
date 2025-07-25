use k256::Secp256k1;
use serde::{Deserialize, Serialize};

use crate::{
    protocol::{Participant, ProtocolError},
    tecdsa::triples_2::{generate_triples_2, TriplesGenManyResult, TriplesGenResult},
    triples::{mul_steps, steps, RcvdTriplesMessages, TriplesState, Wait2Payload},
    CSCurve,
};

pub struct TriplesServer {}

impl TriplesServer {
    pub fn triples_step_1() -> TriplesGenResult<Secp256k1> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let threshold = 2;

        generate_triples_2::<Secp256k1>(&participants, threshold)
    }
}

pub struct TriplesServer2 {}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct TriplesSrvStepOutput<C: CSCurve> {
    pub st_1: TriplesState<C>,
    pub msgs_0: RcvdTriplesMessages<C>,
}

impl TriplesServer2 {
    pub fn triples_step_1() -> Result<TriplesSrvStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];
        let threshold = 2;

        let mut st_1 = TriplesState::<Secp256k1>::new(&participants.clone(), threshold, 2);

        steps::step_1::<Secp256k1>(&mut st_1)?;

        let mut msgs_0 = RcvdTriplesMessages::<Secp256k1>::new();
        let p_1 = participants
            .get(1)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        msgs_0.wait_0.insert(*p_1, st_1.my_commitments.clone());

        Ok(TriplesSrvStepOutput { st_1, msgs_0 })
    }

    pub fn triples_step_2(
        mut st_1: TriplesState<Secp256k1>,
        msgs_1: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesSrvStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_1 = participants
            .get(1)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;
        steps::step_2::<Secp256k1>(&mut st_1, &msgs_1, *p_1)?;

        let mut msgs_0 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_0.wait_1.insert(*p_1, st_1.my_confirmations.clone());

        Ok(TriplesSrvStepOutput { st_1, msgs_0 })
    }

    pub fn triples_step_3(
        mut st_1: TriplesState<Secp256k1>,
        msgs_1: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesSrvStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_1 = participants
            .get(1)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;
        steps::step_3::<Secp256k1>(&mut st_1, &msgs_1, *p_1)?;

        let mut msgs_0 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_0.wait_2.insert(
            *p_1,
            Wait2Payload {
                big_e_i_v: st_1.big_e_i_v.clone(),
                big_f_i_v: st_1.big_f_i_v.clone(),
                big_l_i_v: st_1.big_l_i_v.clone(),
                my_randomizers: st_1.my_randomizers.clone(),
                my_phi_proof0v: st_1.my_phi_proof0v.clone(),
                my_phi_proof1v: st_1.my_phi_proof1v.clone(),
            },
        );

        Ok(TriplesSrvStepOutput { st_1, msgs_0 })
    }

    pub fn triples_step_4(
        mut st_1: TriplesState<Secp256k1>,
        msgs_1: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesSrvStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_1 = participants
            .get(1)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;
        let msgs_to_send = steps::step_4::<Secp256k1>(&mut st_1, &msgs_1, *p_1)?;

        let mut msgs_0 = RcvdTriplesMessages::<Secp256k1>::new();

        // Dangerous! cardinality 1
        for (_p, msg) in msgs_to_send {
            msgs_0.wait_3.insert(*p_1, msg.clone());
        }

        Ok(TriplesSrvStepOutput { st_1, msgs_0 })
    }

    pub fn triples_step_5(
        mut st_1: TriplesState<Secp256k1>,
        msgs_1: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesSrvStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_1 = participants
            .get(1)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;
        let wait_4 = steps::step_5::<Secp256k1>(&mut st_1, &msgs_1, *p_1)?;

        let mut msgs_0 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_0.wait_4.insert(*p_1, wait_4);

        Ok(TriplesSrvStepOutput { st_1, msgs_0 })
    }

    pub fn triples_step_6(
        mut st_1: TriplesState<Secp256k1>,
        msgs_1: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesSrvStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_1 = participants
            .get(1)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        steps::step_6::<Secp256k1>(&mut st_1, &msgs_1, *p_1)?;
        mul_steps::step_1(&mut st_1, &msgs_1, *p_1)?;

        let big_y_affine_v_v =
            mul_steps::multiplication_sender_many_step_1(&mut st_1, &msgs_1, *p_1)?;

        let mut msgs_0 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_0.batch_random_ot_wait_0.insert(*p_1, big_y_affine_v_v);

        Ok(TriplesSrvStepOutput { st_1, msgs_0 })
    }

    pub fn triples_step_7(
        mut st_1: TriplesState<Secp256k1>,
        msgs_1: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesSrvStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_1 = participants
            .get(1)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        let q_v = mul_steps::multiplication_sender_many_step_2(&mut st_1, &msgs_1, *p_1)?;

        let mut msgs_0 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_0.random_ot_extension_wait_0.insert(*p_1, q_v);

        Ok(TriplesSrvStepOutput { st_1, msgs_0 })
    }

    pub fn triples_step_8(
        mut st_1: TriplesState<Secp256k1>,
        msgs_1: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesSrvStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_1 = participants
            .get(1)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        mul_steps::multiplication_sender_many_step_3(&mut st_1, &msgs_1, *p_1)?;

        let mta_wait_0_payload =
            mul_steps::multiplication_sender_many_step_4(&mut st_1, &msgs_1, *p_1)?;

        let mut msgs_0 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_0.mta_wait_0.insert(*p_1, mta_wait_0_payload);

        Ok(TriplesSrvStepOutput { st_1, msgs_0 })
    }

    pub fn triples_step_9(
        mut st_1: TriplesState<Secp256k1>,
        msgs_1: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesSrvStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_1 = participants
            .get(1)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;
        let _ = mul_steps::multiplication_sender_many_step_5(&mut st_1, &msgs_1, *p_1)?;

        let msgs_0 = RcvdTriplesMessages::<Secp256k1>::new();

        Ok(TriplesSrvStepOutput { st_1, msgs_0 })
    }

    pub fn triples_step_10(
        mut st_1: TriplesState<Secp256k1>,
        msgs_1: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesSrvStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_1 = participants
            .get(1)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        let (wait_5_payload, wait_6_payload) = steps::step_7(&mut st_1, &msgs_1, *p_1)?;

        let mut msgs_0 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_0.wait_5.insert(*p_1, wait_5_payload);
        msgs_0.wait_6.insert(*p_1, wait_6_payload);

        Ok(TriplesSrvStepOutput { st_1, msgs_0 })
    }

    pub fn triples_step_11(
        mut st_1: TriplesState<Secp256k1>,
        msgs_1: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesGenManyResult<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_1 = participants
            .get(1)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        let triples_gen_result_1 = steps::step_8(&mut st_1, &msgs_1, *p_1)?;

        Ok(triples_gen_result_1)
    }
}

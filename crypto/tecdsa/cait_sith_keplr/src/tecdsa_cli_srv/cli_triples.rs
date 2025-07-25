use k256::Secp256k1;
use serde::{Deserialize, Serialize};

use crate::{
    protocol::{Participant, ProtocolError},
    tecdsa::triples_2::TriplesGenManyResult,
    triples::{mul_steps, steps, RcvdTriplesMessages, TriplesState, Wait2Payload},
    CSCurve,
};

pub struct TriplesClient {}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct TriplesCliStepOutput<C: CSCurve> {
    pub st_0: TriplesState<C>,
    pub msgs_1: RcvdTriplesMessages<C>,
}

impl TriplesClient {
    pub fn triples_step_1() -> Result<TriplesCliStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];
        let threshold = 2;

        let mut st_0 = TriplesState::<Secp256k1>::new(&participants.clone(), threshold, 2);

        steps::step_1::<Secp256k1>(&mut st_0)?;

        let mut msgs_1 = RcvdTriplesMessages::<Secp256k1>::new();
        let p_0 = participants
            .get(0)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        msgs_1.wait_0.insert(*p_0, st_0.my_commitments.clone());

        Ok(TriplesCliStepOutput { st_0, msgs_1 })
    }

    pub fn triples_step_2(
        mut st_0: TriplesState<Secp256k1>,
        msgs_0: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesCliStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants
            .get(0)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;
        steps::step_2::<Secp256k1>(&mut st_0, &msgs_0, *p_0)?;

        let mut msgs_1 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_1.wait_1.insert(*p_0, st_0.my_confirmations.clone());

        Ok(TriplesCliStepOutput { st_0, msgs_1 })
    }

    pub fn triples_step_3(
        mut st_0: TriplesState<Secp256k1>,
        msgs_0: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesCliStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants
            .get(0)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;
        steps::step_3::<Secp256k1>(&mut st_0, &msgs_0, *p_0)?;

        let mut msgs_1 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_1.wait_2.insert(
            *p_0,
            Wait2Payload {
                big_e_i_v: st_0.big_e_i_v.clone(),
                big_f_i_v: st_0.big_f_i_v.clone(),
                big_l_i_v: st_0.big_l_i_v.clone(),
                my_randomizers: st_0.my_randomizers.clone(),
                my_phi_proof0v: st_0.my_phi_proof0v.clone(),
                my_phi_proof1v: st_0.my_phi_proof1v.clone(),
            },
        );

        Ok(TriplesCliStepOutput { st_0, msgs_1 })
    }

    pub fn triples_step_4(
        mut st_0: TriplesState<Secp256k1>,
        msgs_0: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesCliStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants
            .get(0)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;
        let msgs_to_send = steps::step_4::<Secp256k1>(&mut st_0, &msgs_0, *p_0)?;

        let mut msgs_1 = RcvdTriplesMessages::<Secp256k1>::new();

        // Dangerous! cardinality 1
        for (_p, msg) in msgs_to_send {
            msgs_1.wait_3.insert(*p_0, msg.clone());
        }

        Ok(TriplesCliStepOutput { st_0, msgs_1 })
    }

    pub fn triples_step_5(
        mut st_0: TriplesState<Secp256k1>,
        msgs_0: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesCliStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants
            .get(0)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;
        let wait_4 = steps::step_5::<Secp256k1>(&mut st_0, &msgs_0, *p_0)?;

        let mut msgs_1 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_1.wait_4.insert(*p_0, wait_4);

        Ok(TriplesCliStepOutput { st_0, msgs_1 })
    }

    pub fn triples_step_6(
        mut st_0: TriplesState<Secp256k1>,
        msgs_0: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesCliStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants
            .get(0)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        steps::step_6::<Secp256k1>(&mut st_0, &msgs_0, *p_0)?;
        mul_steps::step_1(&mut st_0, &msgs_0, *p_0)?;

        let big_y_affine_v_v =
            mul_steps::multiplication_receiver_many_step_1(&mut st_0, &msgs_0, *p_0)?;

        let mut msgs_1 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_1.batch_random_ot_wait_0.insert(*p_0, big_y_affine_v_v);

        Ok(TriplesCliStepOutput { st_0, msgs_1 })
    }

    pub fn triples_step_7(
        mut st_0: TriplesState<Secp256k1>,
        msgs_0: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesCliStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants
            .get(0)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        let u_v = mul_steps::multiplication_receiver_many_step_2(&mut st_0, &msgs_0, *p_0)?;

        let mut msgs_1 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_1.correlated_ot_wait_0.insert(*p_0, u_v);

        Ok(TriplesCliStepOutput { st_0, msgs_1 })
    }

    pub fn triples_step_8(
        mut st_0: TriplesState<Secp256k1>,
        msgs_0: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesCliStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants
            .get(0)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        let small_x_t_v = mul_steps::multiplication_receiver_many_step_3(&mut st_0, &msgs_0, *p_0)?;

        let mut msgs_1 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_1.random_ot_extension_wait_1.insert(*p_0, small_x_t_v);

        Ok(TriplesCliStepOutput { st_0, msgs_1 })
    }

    pub fn triples_step_9(
        mut st_0: TriplesState<Secp256k1>,
        msgs_0: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesCliStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants
            .get(0)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        let mta_wait_1_payload =
            mul_steps::multiplication_receiver_many_step_4(&mut st_0, &msgs_0, *p_0)?;

        let mut msgs_1 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_1.mta_wait_1.insert(*p_0, mta_wait_1_payload);

        Ok(TriplesCliStepOutput { st_0, msgs_1 })
    }

    pub fn triples_step_10(
        mut st_0: TriplesState<Secp256k1>,
        msgs_0: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesCliStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants
            .get(0)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        let (wait_5_payload, wait_6_payload) = steps::step_7(&mut st_0, &msgs_0, *p_0)?;

        let mut msgs_1 = RcvdTriplesMessages::<Secp256k1>::new();
        msgs_1.wait_5.insert(*p_0, wait_5_payload);
        msgs_1.wait_6.insert(*p_0, wait_6_payload);

        Ok(TriplesCliStepOutput { st_0, msgs_1 })
    }

    pub fn triples_step_11(
        mut st_0: TriplesState<Secp256k1>,
        msgs_0: RcvdTriplesMessages<Secp256k1>,
    ) -> Result<TriplesGenManyResult<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants
            .get(0)
            .ok_or(ProtocolError::Other("participant not exists".into()))?;

        let triples_gen_result_0 = steps::step_8(&mut st_0, &msgs_0, *p_0)?;

        Ok(triples_gen_result_0)
    }
}

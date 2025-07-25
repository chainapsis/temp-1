use rand_core::OsRng;
use serde::{Deserialize, Serialize};

use crate::{
    protocol::{Participant, ProtocolError},
    triples::{
        self, mul_steps, steps, RcvdTriplesMessages, TriplePub, TripleShare, TriplesState,
        Wait2Payload,
    },
    CSCurve,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct TriplesGenResult<C: CSCurve> {
    pub pub0: TriplePub<C>,

    pub shares0: Vec<TripleShare<C>>,

    pub pub1: TriplePub<C>,

    pub shares1: Vec<TripleShare<C>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct TriplesGenManyResult<C: CSCurve> {
    pub pub_v: Vec<TriplePub<C>>,
    pub share_v: Vec<TripleShare<C>>,
}

pub fn generate_triples_2<C: CSCurve>(
    participants: &[Participant],
    threshold: usize,
) -> TriplesGenResult<C> {
    let (pub0, shares0) = triples::deal::<C>(&mut OsRng, &participants, threshold);
    let (pub1, shares1) = triples::deal::<C>(&mut OsRng, &participants, threshold);

    TriplesGenResult {
        pub0,
        shares0,
        pub1,
        shares1,
    }
}

pub fn generate_triples_3<C: CSCurve>(
    participants: &[Participant],
    threshold: usize,
) -> Result<Vec<TriplesGenManyResult<C>>, ProtocolError> {
    let p_0 = participants[0];
    let p_1 = participants[1];

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

    let mut st_0: TriplesState<C> = TriplesState::new(&participants, threshold, 3);
    let mut st_1: TriplesState<C> = TriplesState::new(&participants, threshold, 3);

    let mut msgs_0: RcvdTriplesMessages<C> = RcvdTriplesMessages::new();
    let mut msgs_1: RcvdTriplesMessages<C> = RcvdTriplesMessages::new();

    {
        steps::step_1(&mut st_0).unwrap();

        // send_many
        msgs_1.wait_0.insert(p_0, st_0.my_commitments.clone());

        steps::step_1(&mut st_1).unwrap();

        // send_many
        msgs_0.wait_0.insert(p_1, st_1.my_commitments.clone());

        println!("commitments: {:?}", st_0.my_commitments);
    }

    {
        steps::step_2(&mut st_0, &msgs_0, p_0).unwrap();

        // send_many
        msgs_1.wait_1.insert(p_0, st_0.my_confirmations.clone());

        steps::step_2(&mut st_1, &msgs_1, p_1).unwrap();

        // send_many
        msgs_0.wait_1.insert(p_1, st_1.my_confirmations.clone());
    }

    {
        steps::step_3(&mut st_0, &msgs_0, p_0).unwrap();

        // send_many
        msgs_1.wait_2.insert(
            p_0,
            Wait2Payload {
                big_e_i_v: st_0.big_e_i_v.clone(),
                big_f_i_v: st_0.big_f_i_v.clone(),
                big_l_i_v: st_0.big_l_i_v.clone(),
                my_randomizers: st_0.my_randomizers.clone(),
                my_phi_proof0v: st_0.my_phi_proof0v.clone(),
                my_phi_proof1v: st_0.my_phi_proof1v.clone(),
            },
        );

        steps::step_3(&mut st_1, &msgs_1, p_1).unwrap();

        // send_many
        msgs_0.wait_2.insert(
            p_1,
            Wait2Payload {
                big_e_i_v: st_1.big_e_i_v.clone(),
                big_f_i_v: st_1.big_f_i_v.clone(),
                big_l_i_v: st_1.big_l_i_v.clone(),
                my_randomizers: st_1.my_randomizers.clone(),
                my_phi_proof0v: st_1.my_phi_proof0v.clone(),
                my_phi_proof1v: st_1.my_phi_proof1v.clone(),
            },
        );
    }

    {
        let msgs_to_send = steps::step_4(&mut st_0, &msgs_0, p_0).unwrap();

        // Dangerous! cardinality 1
        for (_p, msg) in msgs_to_send {
            msgs_1.wait_3.insert(p_0, msg.clone());
        }

        let msgs_to_send = steps::step_4(&mut st_1, &msgs_1, p_1).unwrap();

        for (_p, msg) in msgs_to_send {
            msgs_0.wait_3.insert(p_1, msg.clone());
        }
    }

    {
        let wait_4 = steps::step_5(&mut st_0, &msgs_0, p_0).unwrap();

        msgs_1.wait_4.insert(p_0, wait_4);

        let wait_4 = steps::step_5(&mut st_1, &msgs_1, p_1).unwrap();

        msgs_0.wait_4.insert(p_1, wait_4);
    }

    {
        steps::step_6(&mut st_0, &msgs_0, p_0).unwrap();

        steps::step_6(&mut st_1, &msgs_1, p_1).unwrap();
    }

    {
        // multiplication receiver
        mul_steps::step_1(&mut st_0, &msgs_0, p_0).unwrap();

        let big_y_affine_v_v =
            mul_steps::multiplication_receiver_many_step_1(&mut st_0, &msgs_0, p_0).unwrap();

        msgs_1.batch_random_ot_wait_0.insert(p_0, big_y_affine_v_v);

        // multiplication sender
        mul_steps::step_1(&mut st_1, &msgs_1, p_1).unwrap();

        let big_y_affine_v_v =
            mul_steps::multiplication_sender_many_step_1(&mut st_1, &msgs_1, p_1).unwrap();

        msgs_0.batch_random_ot_wait_0.insert(p_1, big_y_affine_v_v);
    }

    {
        let u_v = mul_steps::multiplication_receiver_many_step_2(&mut st_0, &msgs_0, p_0).unwrap();

        msgs_1.correlated_ot_wait_0.insert(p_0, u_v);

        let q_v = mul_steps::multiplication_sender_many_step_2(&mut st_1, &msgs_1, p_1).unwrap();

        msgs_0.random_ot_extension_wait_0.insert(p_1, q_v);
    }

    {
        let small_x_t_v =
            mul_steps::multiplication_receiver_many_step_3(&mut st_0, &msgs_0, p_0).unwrap();

        msgs_1.random_ot_extension_wait_1.insert(p_0, small_x_t_v);

        let _ = mul_steps::multiplication_sender_many_step_3(&mut st_1, &msgs_1, p_1).unwrap();
    }

    {
        let mta_wait_0_payload =
            mul_steps::multiplication_sender_many_step_4(&mut st_1, &msgs_1, p_1).unwrap();

        msgs_0.mta_wait_0.insert(p_1, mta_wait_0_payload);

        let mta_wait_1_payload =
            mul_steps::multiplication_receiver_many_step_4(&mut st_0, &msgs_0, p_0).unwrap();

        msgs_1.mta_wait_1.insert(p_0, mta_wait_1_payload);

        let _ = mul_steps::multiplication_sender_many_step_5(&mut st_1, &msgs_1, p_1).unwrap();
    }

    {
        let (wait_5_payload, wait_6_payload) = steps::step_7(&mut st_0, &msgs_0, p_0).unwrap();

        msgs_1.wait_5.insert(p_0, wait_5_payload);
        msgs_1.wait_6.insert(p_0, wait_6_payload);

        let (wait_5_payload, wait_6_payload) = steps::step_7(&mut st_1, &msgs_1, p_1).unwrap();

        msgs_0.wait_5.insert(p_1, wait_5_payload);
        msgs_0.wait_6.insert(p_1, wait_6_payload);

        let triples_gen_result_0 = steps::step_8(&mut st_0, &msgs_0, p_0).unwrap();
        println!("triples_gen_result_0: {:?}", triples_gen_result_0);

        let triples_gen_result_1 = steps::step_8(&mut st_1, &msgs_1, p_1).unwrap();
        println!("triples_gen_result_1: {:?}", triples_gen_result_1);

        return Ok(vec![triples_gen_result_0, triples_gen_result_1]);
    }
}

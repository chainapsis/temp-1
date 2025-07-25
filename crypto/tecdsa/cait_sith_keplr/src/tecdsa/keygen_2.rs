use crate::{
    keyshare::{
        steps::{self, step_2, step_3, step_4, step_5},
        KeyshareState, RcvdKeyshareMessages,
    },
    participants::ParticipantList,
    protocol::{InitializationError, Participant, ProtocolError},
    CSCurve, KeygenOutput,
};

pub fn keygen_2<C: CSCurve>(
    participants: &Vec<Participant>,
    threshold: usize,
) -> Result<Vec<(Participant, KeygenOutput<C>)>, ProtocolError> {
    let p_0 = participants[0];
    let p_1 = participants[1];
    let p_2 = participants[2];

    let participant_list = ParticipantList::new(&participants)
        .ok_or_else(|| {
            InitializationError::BadParameters(
                "participant list cannot contain duplicates".to_string(),
            )
        })
        .unwrap();

    println!("\nparticipant list: {:#?}", participant_list);

    let mut st_0 = KeyshareState::new(&participant_list, threshold);
    let mut st_1 = KeyshareState::new(&participant_list, threshold);
    let mut st_2 = KeyshareState::new(&participant_list, threshold);

    let mut msgs_0 = RcvdKeyshareMessages::new();
    let mut msgs_1 = RcvdKeyshareMessages::new();
    let mut msgs_2 = RcvdKeyshareMessages::new();

    // Step 1
    {
        steps::step_1::<C>(&mut st_0, None).unwrap();

        // send_many
        msgs_1.wait_0.insert(p_0, st_0.commitment.unwrap());
        msgs_2.wait_0.insert(p_0, st_0.commitment.unwrap());

        steps::step_1::<C>(&mut st_1, None).unwrap();

        // send_many
        msgs_0.wait_0.insert(p_1, st_1.commitment.unwrap());
        msgs_2.wait_0.insert(p_1, st_1.commitment.unwrap());

        steps::step_1::<C>(&mut st_2, None).unwrap();

        // send_many
        msgs_0.wait_0.insert(p_2, st_2.commitment.unwrap());
        msgs_1.wait_0.insert(p_2, st_2.commitment.unwrap());

        println!("\n comm 0: {:#?}", st_0.commitment);
        // println!("\n comm 0: {:#?}", st_0.rcvd_commitments);
    }

    // Step 2
    {
        // p0
        let out = step_2::<C>(&mut st_0, &msgs_0, p_0).unwrap();
        msgs_1.wait_1.insert(p_0, out.confirmation);
        msgs_2.wait_1.insert(p_0, out.confirmation);

        let out = step_2::<C>(&mut st_1, &msgs_1, p_1).unwrap();
        msgs_0.wait_1.insert(p_1, out.confirmation);
        msgs_2.wait_1.insert(p_1, out.confirmation);

        let out = step_2::<C>(&mut st_2, &msgs_2, p_2).unwrap();
        msgs_0.wait_1.insert(p_2, out.confirmation);
        msgs_1.wait_1.insert(p_2, out.confirmation);
    }

    // Step 3
    {
        // p0
        step_3::<C>(&mut st_0, p_0).unwrap();

        // send many
        msgs_1.wait_2.insert(
            p_0,
            (
                st_0.big_f.clone().unwrap(),
                st_0.randomizer.clone().unwrap(),
                st_0.phi_proof.clone().unwrap(),
            ),
        );

        msgs_2.wait_2.insert(
            p_0,
            (
                st_0.big_f.clone().unwrap(),
                st_0.randomizer.clone().unwrap(),
                st_0.phi_proof.clone().unwrap(),
            ),
        );

        // p1
        step_3::<C>(&mut st_1, p_1).unwrap();

        // send many
        msgs_0.wait_2.insert(
            p_1,
            (
                st_1.big_f.clone().unwrap(),
                st_1.randomizer.clone().unwrap(),
                st_1.phi_proof.clone().unwrap(),
            ),
        );

        msgs_2.wait_2.insert(
            p_1,
            (
                st_1.big_f.clone().unwrap(),
                st_1.randomizer.clone().unwrap(),
                st_1.phi_proof.clone().unwrap(),
            ),
        );

        // p2
        step_3::<C>(&mut st_2, p_2).unwrap();

        // send many
        msgs_0.wait_2.insert(
            p_2,
            (
                st_2.big_f.clone().unwrap(),
                st_2.randomizer.clone().unwrap(),
                st_2.phi_proof.clone().unwrap(),
            ),
        );

        msgs_1.wait_2.insert(
            p_2,
            (
                st_2.big_f.clone().unwrap(),
                st_2.randomizer.clone().unwrap(),
                st_2.phi_proof.clone().unwrap(),
            ),
        );
    }

    // Step 4
    {
        // p0
        let out = step_4(&mut st_0, p_0).unwrap();

        // send private (0 => 1)
        let x_i_j_1 = out.x_i_js.get(&p_1).unwrap();
        msgs_1.wait_3.insert(p_0, x_i_j_1.clone());

        // send private (0 => 2)
        let x_i_j_2 = out.x_i_js.get(&p_2).unwrap();
        msgs_2.wait_3.insert(p_0, x_i_j_2.clone());

        // p1
        let out = step_4(&mut st_1, p_1).unwrap();

        // send private (1 => 0)
        let x_i_j_0 = out.x_i_js.get(&p_0).unwrap();
        msgs_0.wait_3.insert(p_1, x_i_j_0.clone());

        // send private (1 => 2)
        let x_i_j_2 = out.x_i_js.get(&p_2).unwrap();
        msgs_2.wait_3.insert(p_1, x_i_j_2.clone());

        // p2
        let out = step_4(&mut st_2, p_2).unwrap();

        // send private (2 => 0)
        let x_i_j_0 = out.x_i_js.get(&p_0).unwrap();
        msgs_0.wait_3.insert(p_2, x_i_j_0.clone());

        // send private (2 => 1)
        let x_i_j_1 = out.x_i_js.get(&p_1).unwrap();
        msgs_1.wait_3.insert(p_2, x_i_j_1.clone());
    }

    let mut result = vec![];
    // Step 5
    {
        // p0
        let (sk_0, pk_0) = step_5::<C>(&mut st_0, &msgs_0, p_0, None).unwrap();
        let out_0: KeygenOutput<C> = KeygenOutput {
            public_key: pk_0,
            private_share: sk_0,
        };

        result.push((p_0, out_0));

        // p1
        let (sk_1, pk_1) = step_5(&mut st_1, &msgs_1, p_1, None).unwrap();
        let out_1: KeygenOutput<C> = KeygenOutput {
            public_key: pk_1,
            private_share: sk_1,
        };

        result.push((p_1, out_1));

        // p2
        let (sk_2, pk_2) = step_5(&mut st_2, &msgs_2, p_2, None).unwrap();
        let out_2: KeygenOutput<C> = KeygenOutput {
            public_key: pk_2,
            private_share: sk_2,
        };

        result.push((p_2, out_2));
    }

    Ok(result)
}

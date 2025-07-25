use elliptic_curve::{Field, Group, ScalarPrimitive};
use magikitten::Transcript;
use rand_core::OsRng;
use std::collections::HashMap;

use crate::compat::CSCurve;
use crate::crypto::{commit, hash, Digest};
use crate::math::Polynomial;
use crate::proofs::dlog;
use crate::protocol::{Participant, Protocol, ProtocolError};
use crate::serde::encode;

use super::state::KeyshareState;
use super::{RcvdKeyshareMessages, LABEL};

// 1. many (commit(f))
// 2. many (sigma_c)
// 3. many (pi_t(c))
// 4. priv (e(f, 1))
// 5. gen

type To = Participant;
pub struct Step2Output {
    pub confirmation: Digest,
}

pub struct Step4Output<C: CSCurve> {
    pub x_i_js: HashMap<To, ScalarPrimitive<C>>,
}

pub fn step_1<C: CSCurve>(
    state: &mut KeyshareState<C>,
    _big_s: Option<C::ProjectivePoint>,
) -> Result<(), ProtocolError> {
    let s_i = C::Scalar::random(state.rng);

    // Spec 1.2
    state.transcript.message(b"group", C::NAME);
    state
        .transcript
        .message(b"participants", &encode(&state.participant_list));
    // To allow interop between platforms where usize is different!
    state.transcript.message(
        b"threshold",
        &u64::try_from(state.threshold).unwrap().to_be_bytes(),
    );

    // Spec 1.3
    let f: Polynomial<C> = Polynomial::extend_random(&mut state.rng, state.threshold, &s_i);

    // Spec 1.4
    let big_f = f.commit();

    // Spec 1.5
    let (my_commitment, my_randomizer) = commit(&mut state.rng, &big_f);

    println!(
        "\nbig_f: {:?}, my_comm: {:?}, my_rnd: {:?}",
        big_f, my_commitment, my_randomizer
    );

    state.commitment = Some(my_commitment);
    state.big_f = Some(big_f);
    state.f = Some(f);
    state.randomizer = Some(my_randomizer);

    Ok(())
}

pub fn step_2<C: CSCurve>(
    state: &mut KeyshareState<C>,
    msgs: &RcvdKeyshareMessages<C>,
    me: Participant,
) -> Result<Step2Output, ProtocolError> {
    state.all_commitments.put(me, state.commitment.unwrap());
    for (p, commitment) in &msgs.wait_0 {
        state.all_commitments.put(*p, *commitment);
    }

    let mut transcript = Transcript::new(LABEL);

    let my_confirmation = hash(&state.all_commitments);

    transcript.message(b"confirmation", my_confirmation.as_ref());

    state.confirmation = Some(my_confirmation);

    let out = Step2Output {
        confirmation: my_confirmation,
    };

    Ok(out)
}

pub fn step_3<C: CSCurve>(
    state: &mut KeyshareState<C>,
    me: Participant,
) -> Result<(), ProtocolError> {
    let f = state.f.clone().unwrap();
    let big_f = state.big_f.clone().unwrap();

    // Spec 2.5
    let statement = dlog::Statement::<C> {
        public: &big_f.evaluate_zero(),
    };

    let witness = dlog::Witness::<C> {
        x: &f.evaluate_zero(),
    };

    let my_phi_proof = dlog::prove(
        &mut OsRng,
        &mut state.transcript.forked(b"dlog0", &me.bytes()),
        statement,
        witness,
    );

    state.phi_proof = Some(my_phi_proof);

    Ok(())
}

pub fn step_4<C: CSCurve>(
    state: &mut KeyshareState<C>,
    me: Participant,
) -> Result<Step4Output<C>, ProtocolError> {
    let f = state.f.clone().unwrap();
    let mut output = Step4Output {
        x_i_js: HashMap::new(),
    };

    for p in state.participant_list.others(me) {
        let x_i_j: ScalarPrimitive<C> = f.evaluate(&p.scalar::<C>()).into();
        output.x_i_js.insert(p, x_i_j);
    }

    let x_i = f.evaluate(&me.scalar::<C>());
    state.x_i = Some(x_i);

    Ok(output)
}

pub fn step_5<C: CSCurve>(
    state: &mut KeyshareState<C>,
    msgs: &RcvdKeyshareMessages<C>,
    me: Participant,
    big_s: Option<C::ProjectivePoint>,
) -> Result<(C::Scalar, C::AffinePoint), ProtocolError> {
    let confirmation = state.confirmation.unwrap();

    for (p, conf) in &msgs.wait_1 {
        if confirmation != *conf {
            return Err(ProtocolError::AssertionFailed(format!(
                "confirmation from {p:?} did not match expectation"
            )));
        }
    }

    for (p, (their_big_f, their_randomizer, their_phi_proof)) in &msgs.wait_2 {
        if their_big_f.len() != state.threshold {
            return Err(ProtocolError::AssertionFailed(format!(
                "polynomial from {p:?} has the wrong length"
            )));
        }

        if !state.all_commitments[*p].check(&their_big_f, &their_randomizer) {
            return Err(ProtocolError::AssertionFailed(format!(
                "commitment from {p:?} did not match revealed F"
            )));
        }

        let statement = dlog::Statement::<C> {
            public: &their_big_f.evaluate_zero(),
        };

        if !dlog::verify(
            &mut state.transcript.forked(b"dlog0", &p.bytes()),
            statement,
            &their_phi_proof,
        ) {
            return Err(ProtocolError::AssertionFailed(format!(
                "dlog proof from {p:?} failed to verify"
            )));
        }

        let mut big_f = state.big_f.clone().unwrap();
        big_f += &their_big_f;
        state.big_f = Some(big_f);
    }

    let mut x_i = state.x_i.unwrap();
    for (_p, x_j_i) in &msgs.wait_3 {
        x_i += C::Scalar::from(*x_j_i);
    }
    state.x_i = Some(x_i);

    let big_f = state.big_f.clone().unwrap();
    // Spec 3.7
    if big_f.evaluate(&me.scalar::<C>()) != C::ProjectivePoint::generator() * x_i {
        return Err(ProtocolError::AssertionFailed(
            "received bad private share".to_string(),
        ));
    }

    // // Spec 3.8
    let big_x = big_f.evaluate_zero();
    match big_s {
        Some(big_s) if big_s != big_x => {
            return Err(ProtocolError::AssertionFailed(
                "new public key does not match old public key".to_string(),
            ))
        }
        _ => {}
    };

    // Spec 3.9
    Ok((x_i, big_x.into()))
}

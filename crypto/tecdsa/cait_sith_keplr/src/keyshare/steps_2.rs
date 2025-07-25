use elliptic_curve::{Field, Group, ScalarPrimitive};
use magikitten::Transcript;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::steps::Step4Output;
use super::{KeyshareState2, RcvdKeyshareMessages, LABEL};
use crate::compat::CSCurve;
use crate::crypto::{commit, hash, Commitment, Digest};
use crate::math::Polynomial;
use crate::participants::{ParticipantList, ParticipantMap};
use crate::proofs::dlog;
use crate::protocol::{Participant, ProtocolError};
use crate::serde::encode;

// transcript labels, order matters!
const GROUP: &'static [u8] = b"group";
const PARTICIPANTS: &[u8] = b"participants";
const THRESHOLD: &[u8] = b"participants";
const CONFIRMATION: &[u8] = b"confirmation";
const DLOG0: &[u8] = b"dlog0";

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Debug, Clone)]
pub enum KeyshareTranscriptKeys {
    Group = 0,
    Participants,
    Threshold,
    Confirmation,
    DLog0,
}

pub struct Step2Output {
    pub confirmation: Digest,
}

pub struct WaitPoint1 {}

pub struct WaitPoint2 {}

pub struct WaitPoint3 {}

pub struct WaitPoint4 {}

pub fn step_1<C: CSCurve>(
    state: &mut KeyshareState2<C>,
    _big_s: Option<C::ProjectivePoint>,
) -> Result<(), ProtocolError> {
    let mut rng = OsRng;

    let participant_list = ParticipantList::new(&state.participants)
        .ok_or_else(|| {
            ProtocolError::Other(
                "participant list cannot contain duplicates"
                    .to_string()
                    .into(),
            )
        })
        .unwrap();

    let s_i = C::Scalar::random(rng);

    state
        .transcript_feed
        .insert(KeyshareTranscriptKeys::Group, C::NAME.to_vec());
    state.transcript_feed.insert(
        KeyshareTranscriptKeys::Participants,
        encode(&participant_list).to_vec(),
    );
    state.transcript_feed.insert(
        KeyshareTranscriptKeys::Threshold,
        u64::try_from(state.threshold)
            .unwrap()
            .to_be_bytes()
            .to_vec(),
    );

    // Spec 1.3
    let f: Polynomial<C> = Polynomial::extend_random(&mut rng, state.threshold, &s_i);

    // // Spec 1.4
    let big_f = f.commit();

    // // Spec 1.5
    let (my_commitment, my_randomizer) = commit(&mut rng, &big_f);

    let f_coefficients: Vec<ScalarPrimitive<C>> =
        f.coefficients.iter().map(|c| (*c).into()).collect();

    state.commitment = Some(my_commitment);
    state.big_f = Some(big_f);
    state.f_coefficients = Some(f_coefficients);
    state.randomizer = Some(my_randomizer);

    Ok(())
}

pub fn step_2<C: CSCurve>(
    state: &mut KeyshareState2<C>,
    msgs: &RcvdKeyshareMessages<C>,
    me: Participant,
) -> Result<(), ProtocolError> {
    let participant_list = ParticipantList::new(&state.participants)
        .ok_or_else(|| {
            ProtocolError::Other(
                "participant list cannot contain duplicates"
                    .to_string()
                    .into(),
            )
        })
        .unwrap();

    let mut all_commitments: ParticipantMap<'_, Commitment> =
        ParticipantMap::new(&participant_list);

    all_commitments.put(me, state.commitment.unwrap());
    for (p, commitment) in &msgs.wait_0 {
        all_commitments.put(*p, *commitment);
    }

    let my_confirmation = hash(&all_commitments);

    state.transcript_feed.insert(
        KeyshareTranscriptKeys::Confirmation,
        my_confirmation.as_ref().to_vec(),
    );

    state.confirmation = Some(my_confirmation);

    Ok(())
}

pub fn step_3<C: CSCurve>(
    state: &mut KeyshareState2<C>,
    me: Participant,
) -> Result<(), ProtocolError> {
    let f_coefficients = state.f_coefficients.clone().unwrap();
    let coefficients: Vec<C::Scalar> = f_coefficients.iter().map(|c| (*c).into()).collect();
    let f: Polynomial<C> = Polynomial { coefficients };

    let big_f = state.big_f.clone().unwrap();

    // // Spec 2.5
    let statement = dlog::Statement::<C> {
        public: &big_f.evaluate_zero(),
    };

    let witness = dlog::Witness::<C> {
        x: &f.evaluate_zero(),
    };

    let transcript = make_transcript(state.transcript_feed.clone());

    let my_phi_proof = dlog::prove(
        &mut OsRng,
        &mut transcript.forked(DLOG0, &me.bytes()),
        statement,
        witness,
    );

    state.phi_proof = Some(my_phi_proof);

    Ok(())
}

pub fn step_4<C: CSCurve>(
    state: &mut KeyshareState2<C>,
    me: Participant,
) -> Result<Step4Output<C>, ProtocolError> {
    let f_coefficients = state.f_coefficients.clone().unwrap();
    let coefficients: Vec<C::Scalar> = f_coefficients.iter().map(|c| (*c).into()).collect();
    let f: Polynomial<C> = Polynomial { coefficients };

    let mut output = Step4Output {
        x_i_js: HashMap::new(),
    };

    let participant_list = ParticipantList::new(&state.participants)
        .ok_or_else(|| {
            ProtocolError::Other(
                "participant list cannot contain duplicates"
                    .to_string()
                    .into(),
            )
        })
        .unwrap();

    for p in participant_list.others(me) {
        let x_i_j: ScalarPrimitive<C> = f.evaluate(&p.scalar::<C>()).into();
        output.x_i_js.insert(p, x_i_j);
    }

    let x_i = f.evaluate(&me.scalar::<C>());
    state.x_i = Some(x_i.into());

    Ok(output)
}

pub fn step_5<C: CSCurve>(
    state: &mut KeyshareState2<C>,
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

    let participant_list = ParticipantList::new(&state.participants)
        .ok_or_else(|| {
            ProtocolError::Other(
                "participant list cannot contain duplicates"
                    .to_string()
                    .into(),
            )
        })
        .unwrap();

    let mut all_commitments: ParticipantMap<'_, Commitment> =
        ParticipantMap::new(&participant_list);

    all_commitments.put(me, state.commitment.unwrap());
    for (p, commitment) in &msgs.wait_0 {
        all_commitments.put(*p, *commitment);
    }

    for (p, (their_big_f, their_randomizer, their_phi_proof)) in &msgs.wait_2 {
        if their_big_f.len() != state.threshold {
            return Err(ProtocolError::AssertionFailed(format!(
                "polynomial from {p:?} has the wrong length"
            )));
        }

        if !all_commitments[*p].check(&their_big_f, &their_randomizer) {
            return Err(ProtocolError::AssertionFailed(format!(
                "commitment from {p:?} did not match revealed F"
            )));
        }

        let statement = dlog::Statement::<C> {
            public: &their_big_f.evaluate_zero(),
        };

        let transcript = make_transcript(state.transcript_feed.clone());

        if !dlog::verify(
            &mut transcript.forked(DLOG0, &p.bytes()),
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

    let x_i = state.x_i.unwrap();
    let mut x_i: C::Scalar = C::Scalar::from(x_i);
    for (_p, x_j_i) in &msgs.wait_3 {
        x_i += C::Scalar::from(*x_j_i);
    }
    state.x_i = Some(x_i.into());

    let big_f = state.big_f.clone().unwrap();
    // Spec 3.7
    if big_f.evaluate(&me.scalar::<C>()) != C::ProjectivePoint::generator() * x_i {
        return Err(ProtocolError::AssertionFailed(
            "received bad private share".to_string(),
        ));
    }

    // // // Spec 3.8
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

fn make_transcript(transcript_feed: BTreeMap<KeyshareTranscriptKeys, Vec<u8>>) -> Transcript {
    let mut transcript = Transcript::new(LABEL);
    let group = transcript_feed.get(&KeyshareTranscriptKeys::Group).unwrap();
    let participants = transcript_feed
        .get(&KeyshareTranscriptKeys::Participants)
        .unwrap();
    let threshold = transcript_feed
        .get(&KeyshareTranscriptKeys::Threshold)
        .unwrap();
    let confirmation = transcript_feed
        .get(&KeyshareTranscriptKeys::Confirmation)
        .unwrap();

    transcript.message(GROUP, group);

    transcript.message(PARTICIPANTS, participants);

    transcript.message(THRESHOLD, threshold);

    transcript.message(CONFIRMATION, confirmation);

    transcript
}

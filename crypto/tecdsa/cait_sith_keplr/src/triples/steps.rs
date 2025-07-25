use elliptic_curve::{Field, Group, ProjectivePoint, ScalarPrimitive};
use magikitten::Transcript;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::state::TriplesState;
use super::{
    RcvdTriplesMessages, TriplePub, TripleShare, Wait2Payload, Wait3Payload, Wait4Payload,
    Wait5Payload, Wait6Payload,
};
use crate::compat::{CSCurve, SerializablePoint};
use crate::crypto::{commit, hash, Commitment};
use crate::math::Polynomial;
use crate::participants::{ParticipantList, ParticipantMap};
use crate::proofs::{dlog, dlogeq};
use crate::protocol::{Participant, ProtocolError};
use crate::serde::encode;
use crate::tecdsa::triples_2::TriplesGenManyResult;

const LABEL: &[u8] = b"cait-sith v0.8.0 triple generation";

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Debug, Clone)]
pub enum TriplesTranscriptKeys {
    Group = 0,
    Participants,
    Threshold,
    Confirmation,
}

const GROUP: &'static [u8] = b"group";
const PARTICIPANTS: &[u8] = b"participants";
const THRESHOLD: &[u8] = b"participants";
const CONFIRMATION: &[u8] = b"confirmation";
const DLOG0: &[u8] = b"dlog0";
const DLOG1: &[u8] = b"dlog1";
const DLOG2: &[u8] = b"dlog2";
const DLOGEQ0: &[u8] = b"dlogeq0";

pub fn step_1<C: CSCurve>(st: &mut TriplesState<C>) -> Result<(), ProtocolError> {
    let mut rng = OsRng;

    let participant_list = ParticipantList::new(&st.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    st.transcript_feed
        .insert(TriplesTranscriptKeys::Group, C::NAME.to_vec());
    st.transcript_feed.insert(
        TriplesTranscriptKeys::Participants,
        encode(&participant_list).to_vec(),
    );
    st.transcript_feed.insert(
        TriplesTranscriptKeys::Threshold,
        u64::try_from(st.threshold).unwrap().to_be_bytes().to_vec(),
    );

    for _ in 0..st.triples_count {
        let e: Polynomial<C> = Polynomial::random(&mut rng, st.threshold);
        let f: Polynomial<C> = Polynomial::random(&mut rng, st.threshold);
        let mut l: Polynomial<C> = Polynomial::random(&mut rng, st.threshold);

        // Spec 1.3
        l.set_zero(C::Scalar::ZERO);

        // Spec 1.4
        let big_e_i = e.commit();
        let big_f_i = f.commit();
        let big_l_i = l.commit();

        // Spec 1.5
        let (my_commitment, my_randomizer) = commit(&mut rng, &(&big_e_i, &big_f_i, &big_l_i));

        st.my_commitments.push(my_commitment);
        st.my_randomizers.push(my_randomizer);

        st.big_e_i_v.push(big_e_i);
        st.big_f_i_v.push(big_f_i);
        st.big_l_i_v.push(big_l_i);

        st.e_v_coefficients.push(e.coefficients);
        st.f_v_coefficients.push(f.coefficients);
        st.l_v_coefficients.push(l.coefficients);
    }

    Ok(())
}

pub fn step_2<C: CSCurve>(
    st: &mut TriplesState<C>,
    msgs: &RcvdTriplesMessages<C>,
    me: Participant,
) -> Result<(), ProtocolError> {
    let participant_list = ParticipantList::new(&st.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    // Spec 2.1
    let mut all_commitments_vec: Vec<ParticipantMap<Commitment>> = vec![];
    let mut all_commitments_vec_2 = vec![];
    for i in 0..st.triples_count {
        let mut m = ParticipantMap::new(&participant_list);
        m.put(me, st.my_commitments[i]);
        all_commitments_vec.push(m);

        let mut m_2 = HashMap::new();
        m_2.insert(me, st.my_commitments[i]);
        all_commitments_vec_2.push(m_2);
    }

    while all_commitments_vec
        .iter()
        .any(|all_commitments| !all_commitments.full())
    {
        for i in 0..st.triples_count {
            for (from, commitments) in &msgs.wait_0 {
                all_commitments_vec[i].put(*from, commitments[i]);
                all_commitments_vec_2[i].insert(*from, commitments[i]);
            }
        }
    }

    // Spec 2.2
    let mut my_confirmations = vec![];
    for i in 0..st.triples_count {
        let all_commitments = &all_commitments_vec[i];
        let my_confirmation = hash(all_commitments);
        my_confirmations.push(my_confirmation);
    }

    // Spec 2.3
    st.transcript_feed.insert(
        TriplesTranscriptKeys::Confirmation,
        encode(&my_confirmations),
    );

    st.my_confirmations = my_confirmations;
    st.all_commitments_vec_2 = all_commitments_vec_2;

    Ok(())
}

pub fn step_3<C: CSCurve>(
    st: &mut TriplesState<C>,
    _msgs: &RcvdTriplesMessages<C>,
    me: Participant,
) -> Result<(), ProtocolError> {
    let mut rng = OsRng;

    let mut my_phi_proof0v = vec![];
    let mut my_phi_proof1v = vec![];

    let transcript = make_transcript(&st.transcript_feed);

    for i in 0..st.triples_count {
        let big_e_i = &st.big_e_i_v[i];
        let big_f_i = &st.big_f_i_v[i];

        let e_cofficients = st.e_v_coefficients[i].clone();
        let e: Polynomial<C> = Polynomial {
            coefficients: e_cofficients,
        };

        let f_cofficients = st.f_v_coefficients[i].clone();
        let f: Polynomial<C> = Polynomial {
            coefficients: f_cofficients,
        };

        // Spec 2.6
        let statement0 = dlog::Statement::<C> {
            public: &big_e_i.evaluate_zero(),
        };
        let witness0 = dlog::Witness::<C> {
            x: &e.evaluate_zero(),
        };

        let my_phi_proof0 = dlog::prove(
            &mut rng,
            &mut transcript.forked(DLOG0, &me.bytes()),
            statement0,
            witness0,
        );
        let statement1 = dlog::Statement::<C> {
            public: &big_f_i.evaluate_zero(),
        };
        let witness1 = dlog::Witness::<C> {
            x: &f.evaluate_zero(),
        };
        let my_phi_proof1 = dlog::prove(
            &mut rng,
            &mut transcript.forked(DLOG1, &me.bytes()),
            statement1,
            witness1,
        );
        my_phi_proof0v.push(my_phi_proof0);
        my_phi_proof1v.push(my_phi_proof1);
    }

    st.my_phi_proof0v = my_phi_proof0v;
    st.my_phi_proof1v = my_phi_proof1v;

    Ok(())
}

pub fn step_4<C: CSCurve>(
    st: &mut TriplesState<C>,
    _msgs: &RcvdTriplesMessages<C>,
    me: Participant,
) -> Result<Vec<(Participant, Wait3Payload<C>)>, ProtocolError> {
    let participant_list = ParticipantList::new(&st.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    let mut ret = vec![];
    for p in participant_list.others(me) {
        let mut a_i_j_v = vec![];
        let mut b_i_j_v = vec![];
        for i in 0..st.triples_count {
            // let e = &e_v[i];
            let e_cofficients = st.e_v_coefficients[i].clone();
            let e: Polynomial<C> = Polynomial {
                coefficients: e_cofficients,
            };

            // let f = &f_v[i];
            let f_cofficients = st.f_v_coefficients[i].clone();
            let f: Polynomial<C> = Polynomial {
                coefficients: f_cofficients,
            };

            let a_i_j: ScalarPrimitive<C> = e.evaluate(&p.scalar::<C>()).into();
            let b_i_j: ScalarPrimitive<C> = f.evaluate(&p.scalar::<C>()).into();
            a_i_j_v.push(a_i_j);
            b_i_j_v.push(b_i_j);
        }

        // chan.send_private(wait3, p, &(a_i_j_v, b_i_j_v)).await;
        ret.push((p, Wait3Payload { a_i_j_v, b_i_j_v }));
    }

    Ok(ret)
}

pub fn step_5<C: CSCurve>(
    st: &mut TriplesState<C>,
    msgs: &RcvdTriplesMessages<C>,
    me: Participant,
) -> Result<Wait4Payload<C>, ProtocolError> {
    let mut rng = OsRng;

    let participant_list = ParticipantList::new(&st.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    let mut a_i_v = vec![];
    let mut b_i_v = vec![];

    for i in 0..st.triples_count {
        // let e = &e_v[i];
        let e_cofficients = st.e_v_coefficients[i].clone();
        let e: Polynomial<C> = Polynomial {
            coefficients: e_cofficients,
        };

        // let f = &f_v[i];
        let f_cofficients = st.f_v_coefficients[i].clone();
        let f: Polynomial<C> = Polynomial {
            coefficients: f_cofficients,
        };

        let a_i = e.evaluate(&me.scalar::<C>());
        let b_i = f.evaluate(&me.scalar::<C>());
        a_i_v.push(a_i);
        b_i_v.push(b_i);
    }

    // Spec 3.1 + 3.2
    for p in participant_list.others(me) {
        let confirmation = msgs
            .wait_1
            .get(&p)
            .ok_or(ProtocolError::Other(format!("msg not exists").into()))?;

        if *confirmation != st.my_confirmations {
            return Err(ProtocolError::AssertionFailed(format!(
                "confirmation from {p:?} did not match expectation"
            )));
        }
    }

    // Spec 3.3 + 3.4, and also part of 3.6, 5.3, for summing up the Es, Fs, and Ls.
    let mut big_e_v = vec![];
    let mut big_f_v = vec![];
    let mut big_l_v = vec![];

    let mut big_e_j_zero_v: Vec<ParticipantMap<ProjectivePoint<C>>> = vec![];
    let mut big_e_j_zero_v_2: Vec<HashMap<Participant, C::AffinePoint>> = vec![];

    for i in 0..st.triples_count {
        big_e_v.push(st.big_e_i_v[i].clone());
        big_f_v.push(st.big_f_i_v[i].clone());
        big_l_v.push(st.big_l_i_v[i].clone());

        big_e_j_zero_v.push(ParticipantMap::new(&participant_list));
        big_e_j_zero_v_2.push(HashMap::new());
    }

    for p in participant_list.others(me) {
        let Wait2Payload {
            big_e_i_v: their_big_e_v,
            big_f_i_v: their_big_f_v,
            big_l_i_v: their_big_l_v,
            my_randomizers: their_randomizers,
            my_phi_proof0v: their_phi_proof0_v,
            my_phi_proof1v: their_phi_proof1_v,
        } = msgs
            .wait_2
            .get(&p)
            .ok_or(ProtocolError::Other(format!("msg not exists").into()))?;

        let mut all_commitments_vec: Vec<ParticipantMap<Commitment>> = vec![];

        for i in 0..st.triples_count {
            let all_commitments_2 = &st.all_commitments_vec_2[i];

            let mut m = ParticipantMap::new(&participant_list);
            for (p, commitment) in all_commitments_2 {
                m.put(*p, *commitment);
            }

            all_commitments_vec.push(m);
        }

        for i in 0..st.triples_count {
            let all_commitments = &all_commitments_vec[i];
            let their_big_e = &their_big_e_v[i];
            let their_big_f = &their_big_f_v[i];
            let their_big_l = &their_big_l_v[i];
            let their_randomizer = &their_randomizers[i];
            let their_phi_proof0 = &their_phi_proof0_v[i];
            let their_phi_proof1 = &their_phi_proof1_v[i];

            if their_big_e.len() != st.threshold
                || their_big_f.len() != st.threshold
                || their_big_l.len() != st.threshold
            {
                return Err(ProtocolError::AssertionFailed(format!(
                    "polynomial from {p:?} has the wrong length"
                )));
            }

            if !bool::from(their_big_l.evaluate_zero().is_identity()) {
                return Err(ProtocolError::AssertionFailed(format!(
                    "L(0) from {p:?} is not 0"
                )));
            }

            if !all_commitments[p].check(
                &(&their_big_e, &their_big_f, &their_big_l),
                their_randomizer,
            ) {
                return Err(ProtocolError::AssertionFailed(format!(
                    "commitment from {p:?} did not match revealed F"
                )));
            }

            let statement0 = dlog::Statement::<C> {
                public: &their_big_e.evaluate_zero(),
            };

            let transcript = make_transcript(&st.transcript_feed);

            if !dlog::verify(
                &mut transcript.forked(b"dlog0", &p.bytes()),
                statement0,
                their_phi_proof0,
            ) {
                return Err(ProtocolError::AssertionFailed(format!(
                    "dlog proof from {p:?} failed to verify"
                )));
            }

            let statement1 = dlog::Statement::<C> {
                public: &their_big_f.evaluate_zero(),
            };

            if !dlog::verify(
                &mut transcript.forked(b"dlog1", &p.bytes()),
                statement1,
                their_phi_proof1,
            ) {
                return Err(ProtocolError::AssertionFailed(format!(
                    "dlog proof from {p:?} failed to verify"
                )));
            }

            big_e_j_zero_v[i].put(p, their_big_e.evaluate_zero());
            big_e_j_zero_v_2[i].insert(p, their_big_e.evaluate_zero().into());

            big_e_v[i] += &their_big_e;
            big_f_v[i] += &their_big_f;
            big_l_v[i] += &their_big_l;
        }
    }

    for p in participant_list.others(me) {
        // let (from, (a_j_i_v, b_j_i_v)): (_, (Vec<ScalarPrimitive<C>>, Vec<ScalarPrimitive<C>>)) =
        let Wait3Payload {
            a_i_j_v: a_j_i_v,
            b_i_j_v: b_j_i_v,
        } = msgs
            .wait_3
            .get(&p)
            .ok_or(ProtocolError::Other(format!("msg not exists").into()))?;

        //     if !seen.put(from) {
        //         continue;
        //     }
        for i in 0..st.triples_count {
            let a_j_i = &a_j_i_v[i];
            let b_j_i = &b_j_i_v[i];
            a_i_v[i] += &(*a_j_i).into();
            b_i_v[i] += &(*b_j_i).into();
        }
    }

    let mut big_c_i_points = vec![];
    let mut big_c_i_v = vec![];
    let mut my_phi_proofs = vec![];

    for i in 0..st.triples_count {
        let big_e = &big_e_v[i];
        let big_f = &big_f_v[i];
        let a_i = &a_i_v[i];
        let b_i = &b_i_v[i];

        // let e = &e_v[i];
        let e_cofficients = st.e_v_coefficients[i].clone();
        let e: Polynomial<C> = Polynomial {
            coefficients: e_cofficients,
        };

        // Spec 3.7
        let check1 = big_e.evaluate(&me.scalar::<C>()) != C::ProjectivePoint::generator() * a_i;
        let check2 = big_f.evaluate(&me.scalar::<C>()) != C::ProjectivePoint::generator() * b_i;
        if check1 || check2 {
            return Err(ProtocolError::AssertionFailed(
                "received bad private share".to_string(),
            ));
        }

        // Spec 3.8
        let big_c_i = big_f.evaluate_zero() * e.evaluate_zero();
        let big_e_i = &st.big_e_i_v[i];

        // Spec 3.9
        let statement = dlogeq::Statement::<C> {
            public0: &big_e_i.evaluate_zero(),
            generator1: &big_f.evaluate_zero(),
            public1: &big_c_i,
        };

        let witness = dlogeq::Witness {
            x: &e.evaluate_zero(),
        };

        let transcript = make_transcript(&st.transcript_feed);

        let my_phi_proof = dlogeq::prove(
            &mut rng,
            &mut transcript.forked(b"dlogeq0", &me.bytes()),
            statement,
            witness,
        );

        big_c_i_points.push(SerializablePoint::<C>::from_projective(&big_c_i));
        big_c_i_v.push(big_c_i.into());
        my_phi_proofs.push(my_phi_proof);
    }

    st.a_i_v = a_i_v;
    st.b_i_v = b_i_v;

    st.big_e_v = big_e_v;
    st.big_f_v = big_f_v;
    st.big_l_v = big_l_v;

    st.big_c_i_points = big_c_i_points;
    st.big_c_i_v = big_c_i_v;
    st.my_phi_proofs = my_phi_proofs;

    st.big_e_j_zero_v_2 = big_e_j_zero_v_2;

    let ret = Wait4Payload {
        big_c_i_points: st.big_c_i_points.clone(),
        my_phi_proofs: st.my_phi_proofs.clone(),
    };

    Ok(ret)
}

pub fn step_6<C: CSCurve>(
    st: &mut TriplesState<C>,
    msgs: &RcvdTriplesMessages<C>,
    me: Participant,
) -> Result<(), ProtocolError> {
    let participant_list = ParticipantList::new(&st.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    // Spec 4.1 + 4.2 + 4.3
    let mut big_c_v = vec![];

    for i in 0..st.triples_count {
        big_c_v.push(st.big_c_i_v[i]);
    }

    let mut big_e_j_zero_v = vec![];

    for i in 0..st.triples_count {
        let big_e_j_zero_2 = &st.big_e_j_zero_v_2[i];

        let mut m = ParticipantMap::new(&participant_list);

        for (p, point) in big_e_j_zero_2 {
            m.put(*p, point);
        }

        big_e_j_zero_v.push(m);
    }

    for p in participant_list.others(me) {
        let Wait4Payload {
            big_c_i_points: big_c_j_v,
            my_phi_proofs: their_phi_proofs,
        } = msgs
            .wait_4
            .get(&p)
            .ok_or(ProtocolError::Other(format!("msg not exists").into()))?;

        for i in 0..st.triples_count {
            let big_e_j_zero = &big_e_j_zero_v[i];
            let big_f = &st.big_f_v[i];
            let big_c_j = big_c_j_v[i].to_projective();
            let their_phi_proof = &their_phi_proofs[i];

            let statement = dlogeq::Statement::<C> {
                public0: &(ProjectivePoint::<C>::from(*big_e_j_zero[p])),
                generator1: &big_f.evaluate_zero(),
                public1: &big_c_j,
            };

            let transcript = make_transcript(&st.transcript_feed);

            if !dlogeq::verify(
                &mut transcript.forked(DLOGEQ0, &p.bytes()),
                statement,
                their_phi_proof,
            ) {
                return Err(ProtocolError::AssertionFailed(format!(
                    "dlogeq proof from {p:?} failed to verify"
                )));
            }

            let mut src = ProjectivePoint::<C>::from(big_c_v[i]);
            src += big_c_j;
            big_c_v[i] = src.into();
        }
    }

    st.big_c_v = big_c_v;

    Ok(())
}

pub fn step_7<C: CSCurve>(
    st: &mut TriplesState<C>,
    _msgs: &RcvdTriplesMessages<C>,
    me: Participant,
) -> Result<(Wait5Payload<C>, Wait6Payload<C>), ProtocolError> {
    let participant_list = ParticipantList::new(&st.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    let mut rng = OsRng;

    let mut hat_big_c_i_points = vec![];
    let mut hat_big_c_i_v = vec![];
    let mut my_phi_proofs = vec![];

    let transcript = make_transcript(&st.transcript_feed);

    for i in 0..st.triples_count {
        // Spec 4.5
        let l0 = st.l0_v[i];
        let hat_big_c_i = C::ProjectivePoint::generator() * l0;

        // Spec 4.6
        let statement = dlog::Statement::<C> {
            public: &hat_big_c_i,
        };
        let witness = dlog::Witness::<C> { x: &l0 };
        let my_phi_proof = dlog::prove(
            &mut rng,
            &mut transcript.forked(DLOG2, &me.bytes()),
            statement,
            witness,
        );
        hat_big_c_i_points.push(SerializablePoint::<C>::from_projective(&hat_big_c_i));
        hat_big_c_i_v.push(hat_big_c_i.into());
        my_phi_proofs.push(my_phi_proof);
    }

    let mut l_v = vec![];
    for i in 0..st.triples_count {
        // let e = &e_v[i];
        let l_cofficients = st.l_v_coefficients[i].clone();
        let l: Polynomial<C> = Polynomial {
            coefficients: l_cofficients,
        };
        l_v.push(l);
    }

    // Spec 4.9
    for i in 0..st.triples_count {
        let l = &mut l_v[i];
        let l0 = &st.l0_v[i];
        l.set_zero(*l0);
    }

    let mut wait_6_payload = Wait6Payload { c_i_j_v: vec![] };

    let mut c_i_v = vec![];
    for p in participant_list.others(me) {
        let mut c_i_j_v = Vec::new();
        for i in 0..st.triples_count {
            let l = &mut l_v[i];
            let c_i_j: ScalarPrimitive<C> = l.evaluate(&p.scalar::<C>()).into();
            c_i_j_v.push(c_i_j);
        }
        wait_6_payload.c_i_j_v = c_i_j_v;
    }
    for i in 0..st.triples_count {
        let l = &mut l_v[i];
        let c_i = l.evaluate(&me.scalar::<C>());
        c_i_v.push(c_i);
    }

    st.c_i_v = c_i_v;
    st.hat_big_c_i_v = hat_big_c_i_v;

    let wait_5_payload = Wait5Payload {
        hat_big_c_i_points,
        my_phi_proofs,
    };

    Ok((wait_5_payload, wait_6_payload))
}

pub fn step_8<C: CSCurve>(
    st: &mut TriplesState<C>,
    msgs: &RcvdTriplesMessages<C>,
    me: Participant,
) -> Result<TriplesGenManyResult<C>, ProtocolError> {
    let participant_list = ParticipantList::new(&st.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    let mut hat_big_c_v = vec![];
    for i in 0..st.triples_count {
        hat_big_c_v.push(st.hat_big_c_i_v[i]);
    }

    let transcript = make_transcript(&st.transcript_feed);

    let mut their_hat_big_c_i_points = vec![];
    let mut their_phi_proofs = vec![];
    let mut from = me;
    for (p, payload) in &msgs.wait_5 {
        their_hat_big_c_i_points = payload.hat_big_c_i_points.clone();
        their_phi_proofs = payload.my_phi_proofs.clone();
        from = *p;
        break;
    }

    for _ in participant_list.others(me) {
        for i in 0..st.triples_count {
            let their_hat_big_c = their_hat_big_c_i_points[i].to_projective();
            let their_phi_proof = &their_phi_proofs[i];

            let statement = dlog::Statement::<C> {
                public: &their_hat_big_c,
            };
            if !dlog::verify(
                &mut transcript.forked(DLOG2, &from.bytes()),
                statement,
                their_phi_proof,
            ) {
                return Err(ProtocolError::AssertionFailed(format!(
                    "dlog proof from {from:?} failed to verify"
                )));
            }

            let mut src = ProjectivePoint::<C>::from(hat_big_c_v[i]);
            src += &their_hat_big_c;
            hat_big_c_v[i] = src.into();
        }
    }

    for i in 0..st.triples_count {
        let big_l = &mut st.big_l_v[i];
        let hat_big_c = &hat_big_c_v[i];
        let big_c = &st.big_c_v[i];

        // Spec 5.3
        big_l.set_zero(ProjectivePoint::<C>::from(*hat_big_c));

        // Spec 5.4
        if big_l.evaluate_zero() != (*big_c).into() {
            return Err(ProtocolError::AssertionFailed(
                "final polynomial doesn't match C value".to_owned(),
            ));
        }
    }

    // Spec 5.5 + 5.6

    let mut c_j_i_v = vec![];
    for (_, payload) in &msgs.wait_6 {
        c_j_i_v = payload.c_i_j_v.clone();
        break;
    }

    for _ in participant_list.others(me) {
        for i in 0..st.triples_count {
            let c_j_i = c_j_i_v[i];
            st.c_i_v[i] += C::Scalar::from(c_j_i);
        }
    }

    let mut ret = TriplesGenManyResult {
        pub_v: vec![],
        share_v: vec![],
    };

    // Spec 5.7
    for i in 0..st.triples_count {
        let big_l = &st.big_l_v[i];
        let c_i = &st.c_i_v[i];
        let a_i = &st.a_i_v[i];
        let b_i = &st.b_i_v[i];
        let big_e = &st.big_e_v[i];
        let big_f = &st.big_f_v[i];
        let big_c = &st.big_c_v[i];

        if big_l.evaluate(&me.scalar::<C>()) != C::ProjectivePoint::generator() * c_i {
            return Err(ProtocolError::AssertionFailed(
                "received bad private share of c".to_string(),
            ));
        }

        let big_a = big_e.evaluate_zero().into();
        let big_b = big_f.evaluate_zero().into();
        let big_c = (*big_c).into();

        ret.pub_v.push(TriplePub {
            big_a,
            big_b,
            big_c,
            participants: st.participants.clone().into(),
            threshold: st.threshold,
        });

        ret.share_v.push(TripleShare {
            a: *a_i,
            b: *b_i,
            c: *c_i,
        });
    }

    Ok(ret)
}

fn make_transcript(transcript_feed: &BTreeMap<TriplesTranscriptKeys, Vec<u8>>) -> Transcript {
    let mut transcript = Transcript::new(LABEL);
    let group = transcript_feed.get(&TriplesTranscriptKeys::Group).unwrap();
    let participants = transcript_feed
        .get(&TriplesTranscriptKeys::Participants)
        .unwrap();
    let threshold = transcript_feed
        .get(&TriplesTranscriptKeys::Threshold)
        .unwrap();
    let confirmation = transcript_feed
        .get(&TriplesTranscriptKeys::Confirmation)
        .unwrap();

    transcript.message(GROUP, group);

    transcript.message(PARTICIPANTS, participants);

    transcript.message(THRESHOLD, threshold);

    transcript.message(CONFIRMATION, confirmation);

    transcript
}

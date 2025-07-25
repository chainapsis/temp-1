use elliptic_curve::ScalarPrimitive;
use magikitten::Transcript;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::crypto::Commitment;
use crate::crypto::Digest;
use crate::crypto::Randomizer;
use crate::math::GroupPolynomial;
use crate::math::Polynomial;
use crate::participants::ParticipantList;
use crate::participants::ParticipantMap;
use crate::proofs::dlog::Proof;
use crate::protocol::Participant;
use crate::CSCurve;

use super::LABEL;

pub struct KeyshareState<'a, C: CSCurve> {
    pub rng: OsRng,
    pub threshold: usize,
    pub participant_list: &'a ParticipantList,

    pub transcript: Transcript,
    pub f: Option<Polynomial<C>>,
    pub big_f: Option<GroupPolynomial<C>>,

    pub randomizer: Option<Randomizer>,
    pub phi_proof: Option<Proof<C>>,

    pub commitment: Option<Commitment>,
    pub all_commitments: ParticipantMap<'a, Commitment>,

    pub confirmation: Option<Digest>,

    pub x_i: Option<C::Scalar>,
}

impl<'a, C: CSCurve> KeyshareState<'a, C> {
    pub fn new(participant_list: &'a ParticipantList, threshold: usize) -> KeyshareState<'a, C> {
        let rng = OsRng;
        let transcript = Transcript::new(LABEL);

        let all_commitments = ParticipantMap::new(&participant_list);

        KeyshareState {
            participant_list,
            threshold,

            rng,
            transcript,
            f: None,
            big_f: None,

            randomizer: None,
            commitment: None,
            all_commitments,

            confirmation: None,

            phi_proof: None,

            x_i: None,
        }
    }
}

type From = Participant;
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>,
")]
pub struct RcvdKeyshareMessages<C: CSCurve> {
    #[serde(with = "crate::protocol::participant_serde")]
    pub wait_0: HashMap<From, Commitment>,
    #[serde(with = "crate::protocol::participant_serde")]
    pub wait_1: HashMap<From, Digest>,
    #[serde(with = "crate::protocol::participant_serde")]
    pub wait_2: HashMap<From, (GroupPolynomial<C>, Randomizer, Proof<C>)>,
    #[serde(with = "crate::protocol::participant_serde")]
    pub wait_3: HashMap<From, ScalarPrimitive<C>>,
    pub public_key: Option<C::AffinePoint>,
}

impl<C: CSCurve> RcvdKeyshareMessages<C> {
    pub fn new() -> RcvdKeyshareMessages<C> {
        RcvdKeyshareMessages {
            wait_0: HashMap::new(),
            wait_1: HashMap::new(),
            wait_2: HashMap::new(),
            wait_3: HashMap::new(),
            public_key: None,
        }
    }
}

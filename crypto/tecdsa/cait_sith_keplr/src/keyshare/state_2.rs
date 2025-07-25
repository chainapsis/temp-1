use elliptic_curve::ScalarPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::steps_2::KeyshareTranscriptKeys;
use crate::crypto::Commitment;
use crate::crypto::Digest;
use crate::crypto::Randomizer;
use crate::math::GroupPolynomial;
use crate::proofs::dlog::Proof;
use crate::protocol::Participant;
use crate::CSCurve;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TranscriptFeed {
    group: Vec<u8>,
    participants: Vec<u8>,
    threshold: Vec<u8>,
    confirmation: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>
")]
pub struct KeyshareState2<C: CSCurve> {
    pub participants: Vec<Participant>,
    pub threshold: usize,
    pub transcript_feed: BTreeMap<KeyshareTranscriptKeys, Vec<u8>>,

    pub f_coefficients: Option<Vec<ScalarPrimitive<C>>>,
    pub big_f: Option<GroupPolynomial<C>>,

    pub randomizer: Option<Randomizer>,
    pub phi_proof: Option<Proof<C>>,

    pub commitment: Option<Commitment>,

    pub confirmation: Option<Digest>,

    pub x_i: Option<ScalarPrimitive<C>>,
}

impl<'a, C: CSCurve> KeyshareState2<C> {
    pub fn new(participants: Vec<Participant>, threshold: usize) -> KeyshareState2<C> {
        let transcript_feed = BTreeMap::new();

        KeyshareState2 {
            participants,
            threshold,
            transcript_feed,
            f_coefficients: None,
            big_f: None,
            randomizer: None,
            commitment: None,
            confirmation: None,
            phi_proof: None,
            x_i: None,
        }
    }
}

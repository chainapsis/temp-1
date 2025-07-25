use elliptic_curve::Scalar;
use elliptic_curve::ScalarPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::compat::SerializablePoint;
use crate::crypto::Commitment;
use crate::crypto::Digest;
use crate::crypto::Randomizer;
use crate::math::GroupPolynomial;
use crate::proofs::dlog::Proof;
use crate::proofs::dlogeq;
use crate::protocol::Participant;
use crate::CSCurve;

use super::bits::BitMatrix;
use super::bits::BitVector;
use super::bits::ChoiceVector;
use super::bits::DoubleBitVector;
use super::bits::SquareBitMatrix;
use super::steps::TriplesTranscriptKeys;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TranscriptFeed {
    group: Vec<u8>,
    participants: Vec<u8>,
    threshold: Vec<u8>,
    confirmation: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct TriplesState<C: CSCurve> {
    pub transcript_feed: BTreeMap<TriplesTranscriptKeys, Vec<u8>>,
    pub participants: Vec<Participant>,
    pub threshold: usize,
    pub triples_count: usize,

    pub my_commitments: Vec<Commitment>,
    pub my_randomizers: Vec<Randomizer>,
    // pub e_v: Vec<Polynomial<C>>,
    pub e_v_coefficients: Vec<Vec<Scalar<C>>>,

    // pub f_v: Vec<Polynomial<C>>,
    pub f_v_coefficients: Vec<Vec<Scalar<C>>>,

    // pub l_v: Vec<Polynomial<C>>,
    pub l_v_coefficients: Vec<Vec<Scalar<C>>>,

    pub all_commitments_vec_2: Vec<HashMap<Participant, Commitment>>,

    pub big_e_i_v: Vec<GroupPolynomial<C>>,
    pub big_f_i_v: Vec<GroupPolynomial<C>>,
    pub big_l_i_v: Vec<GroupPolynomial<C>>,

    pub my_confirmations: Vec<Digest>,

    pub my_phi_proof0v: Vec<Proof<C>>,
    pub my_phi_proof1v: Vec<Proof<C>>,

    pub big_c_i_points: Vec<SerializablePoint<C>>,
    // pub big_c_i_v: Vec<ProjectivePoint<C>>,
    pub big_c_i_v: Vec<C::AffinePoint>,
    pub my_phi_proofs: Vec<dlogeq::Proof<C>>,

    pub a_i_v: Vec<Scalar<C>>,
    pub b_i_v: Vec<Scalar<C>>,

    pub big_e_v: Vec<GroupPolynomial<C>>,
    pub big_f_v: Vec<GroupPolynomial<C>>,
    pub big_l_v: Vec<GroupPolynomial<C>>,
    // pub big_e_j_zero_v_2: Vec<HashMap<Participant, ProjectivePoint<C>>>,
    pub big_e_j_zero_v_2: Vec<HashMap<Participant, C::AffinePoint>>,

    pub e0_v: Vec<Scalar<C>>,
    pub f0_v: Vec<Scalar<C>>,

    pub big_y_affine_v: Vec<SerializablePoint<C>>,
    pub yv: Vec<Scalar<C>>,
    // pub big_z_v: Vec<ProjectivePoint<C>>,
    pub big_z_v: Vec<C::AffinePoint>,

    pub dkv: Vec<(BitVector, SquareBitMatrix)>,

    pub t_v: Vec<BitMatrix>,
    pub q_v: Vec<BitMatrix>,
    pub b_v: Vec<ChoiceVector>,
    pub seed_v: Vec<[u8; 32]>,

    pub delta_1_v: Vec<Vec<Scalar<C>>>,
    pub delta_2_v: Vec<Vec<Scalar<C>>>,

    pub receiver_res0_v: Vec<Vec<(u8, Scalar<C>)>>,
    pub receiver_res1_v: Vec<Vec<(u8, Scalar<C>)>>,

    pub sender_res0_v: Vec<Vec<(Scalar<C>, Scalar<C>)>>,
    pub sender_res1_v: Vec<Vec<(Scalar<C>, Scalar<C>)>>,

    pub l0_v: Vec<Scalar<C>>,

    pub big_c_v: Vec<C::AffinePoint>,

    pub c_i_v: Vec<Scalar<C>>,
    // pub hat_big_c_i_v: Vec<ProjectivePoint<C>>,
    pub hat_big_c_i_v: Vec<C::AffinePoint>,
}

impl<'a, C: CSCurve> TriplesState<C> {
    pub fn new(
        participants: &[Participant],
        threshold: usize,
        triples_count: usize,
    ) -> TriplesState<C> {
        TriplesState {
            participants: participants.to_vec(),
            transcript_feed: BTreeMap::new(),
            threshold,
            triples_count,
            my_commitments: vec![],
            my_randomizers: vec![],
            e_v_coefficients: vec![],
            f_v_coefficients: vec![],
            l_v_coefficients: vec![],

            all_commitments_vec_2: vec![],

            a_i_v: vec![],
            b_i_v: vec![],

            big_e_i_v: vec![],
            big_f_i_v: vec![],
            big_l_i_v: vec![],

            my_confirmations: vec![],

            my_phi_proof0v: vec![],
            my_phi_proof1v: vec![],

            big_c_i_points: vec![],
            big_c_i_v: vec![],
            my_phi_proofs: vec![],

            big_e_v: vec![],
            big_f_v: vec![],
            big_l_v: vec![],
            big_e_j_zero_v_2: vec![],

            e0_v: vec![],
            f0_v: vec![],

            big_y_affine_v: vec![],
            yv: vec![],
            big_z_v: vec![],

            dkv: vec![],

            t_v: vec![],
            q_v: vec![],
            b_v: vec![],

            seed_v: vec![],

            delta_1_v: vec![],
            delta_2_v: vec![],

            receiver_res0_v: vec![],
            receiver_res1_v: vec![],

            sender_res0_v: vec![],
            sender_res1_v: vec![],

            l0_v: vec![],

            big_c_v: vec![],

            c_i_v: vec![],
            hat_big_c_i_v: vec![],
        }
    }
}

type From = Participant;
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct RcvdTriplesMessages<C: CSCurve> {
    #[serde(with = "crate::protocol::participant_serde")]
    pub wait_0: HashMap<From, Vec<Commitment>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub wait_1: HashMap<From, Vec<Digest>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub wait_2: HashMap<From, Wait2Payload<C>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub wait_3: HashMap<From, Wait3Payload<C>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub wait_4: HashMap<From, Wait4Payload<C>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub wait_5: HashMap<From, Wait5Payload<C>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub wait_6: HashMap<From, Wait6Payload<C>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub batch_random_ot_wait_0: HashMap<From, Vec<Vec<SerializablePoint<C>>>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub correlated_ot_wait_0: HashMap<From, Vec<BitMatrix>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub random_ot_extension_wait_0: HashMap<From, Vec<[u8; 32]>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub random_ot_extension_wait_1: HashMap<From, Vec<(DoubleBitVector, Vec<DoubleBitVector>)>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub mta_wait_0: HashMap<From, MTAWait0Payload<C>>,

    #[serde(with = "crate::protocol::participant_serde")]
    pub mta_wait_1: HashMap<From, MTAWait1Payload<C>>,
}

impl<C: CSCurve> RcvdTriplesMessages<C> {
    pub fn new() -> RcvdTriplesMessages<C> {
        RcvdTriplesMessages {
            wait_0: HashMap::new(),
            wait_1: HashMap::new(),
            wait_2: HashMap::new(),
            wait_3: HashMap::new(),
            wait_4: HashMap::new(),
            wait_5: HashMap::new(),
            wait_6: HashMap::new(),
            batch_random_ot_wait_0: HashMap::new(),
            correlated_ot_wait_0: HashMap::new(),
            random_ot_extension_wait_0: HashMap::new(),
            random_ot_extension_wait_1: HashMap::new(),
            mta_wait_0: HashMap::new(),
            mta_wait_1: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct Wait2Payload<C: CSCurve> {
    pub big_e_i_v: Vec<GroupPolynomial<C>>,
    pub big_f_i_v: Vec<GroupPolynomial<C>>,
    pub big_l_i_v: Vec<GroupPolynomial<C>>,
    pub my_randomizers: Vec<Randomizer>,
    pub my_phi_proof0v: Vec<Proof<C>>,
    pub my_phi_proof1v: Vec<Proof<C>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct Wait3Payload<C: CSCurve> {
    pub a_i_j_v: Vec<ScalarPrimitive<C>>,
    pub b_i_j_v: Vec<ScalarPrimitive<C>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct Wait4Payload<C: CSCurve> {
    pub big_c_i_points: Vec<SerializablePoint<C>>,
    pub my_phi_proofs: Vec<dlogeq::Proof<C>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct MTAWait0Payload<C: CSCurve> {
    pub c1_v: Vec<Vec<(ScalarPrimitive<C>, ScalarPrimitive<C>)>>,
    pub c2_v: Vec<Vec<(ScalarPrimitive<C>, ScalarPrimitive<C>)>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct MTAWait1Payload<C: CSCurve> {
    pub chi1_seed_1_v: Vec<(Scalar<C>, [u8; 32])>,
    pub chi1_seed_2_v: Vec<(Scalar<C>, [u8; 32])>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct Wait5Payload<C: CSCurve> {
    pub hat_big_c_i_points: Vec<SerializablePoint<C>>,
    pub my_phi_proofs: Vec<Proof<C>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct Wait6Payload<C: CSCurve> {
    pub c_i_j_v: Vec<ScalarPrimitive<C>>,
}

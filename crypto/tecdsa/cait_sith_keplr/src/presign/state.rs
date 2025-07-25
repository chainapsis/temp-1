use elliptic_curve::{CurveArithmetic, ProjectivePoint, ScalarPrimitive};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    compat::CSCurve,
    participants::ParticipantList,
    protocol::Participant,
    triples::{TriplePub, TripleShare},
    KeygenOutput,
};

pub struct PresignState<C: CSCurve> {
    pub rng: OsRng,
    pub threshold: usize,
    pub participants: ParticipantList,
    pub bt_participants: ParticipantList,

    pub triple_0: (TripleShare<C>, TriplePub<C>),
    pub triple_1: (TripleShare<C>, TriplePub<C>),

    pub keygen_out: KeygenOutput<C>,

    pub big_kd: Option<C::AffinePoint>,

    pub big_k: Option<ProjectivePoint<C>>,
    pub big_a: Option<ProjectivePoint<C>>,
    pub big_x: Option<ProjectivePoint<C>>,
    pub big_b: Option<ProjectivePoint<C>>,
    pub big_d: Option<C::AffinePoint>,

    pub sk_lambda: Option<C::Scalar>,
    pub bt_lambda: Option<C::Scalar>,

    pub a_i: Option<C::Scalar>,
    pub c_i: Option<C::Scalar>,
    pub k_i: Option<C::Scalar>,

    pub k_prime_i: Option<C::Scalar>,
    pub a_prime_i: Option<C::Scalar>,
    pub b_prime_i: Option<C::Scalar>,
    pub x_prime_i: Option<C::Scalar>,

    pub kd_i: Option<C::Scalar>,
    pub ka_i: Option<C::Scalar>,
    pub xb_i: Option<C::Scalar>,

    pub kd_sum: Option<C::Scalar>,
    pub ka_sum: Option<C::Scalar>,
    pub xb_sum: Option<C::Scalar>,

    pub big_r: Option<C::AffinePoint>,
    pub k_final: Option<C::Scalar>,
    pub sigma_i_final: Option<C::Scalar>,
}

impl<'a, C: CSCurve> PresignState<C> {
    pub fn new(
        participant_list: ParticipantList,
        threshold: usize,
        keygen_out: KeygenOutput<C>,
        triple_0: (TripleShare<C>, TriplePub<C>),
        triple_1: (TripleShare<C>, TriplePub<C>),
    ) -> Self {
        Self {
            rng: OsRng,
            participants: participant_list.clone(),
            bt_participants: participant_list.clone(),

            threshold,
            keygen_out,

            triple_0,
            triple_1,

            big_kd: None,

            big_k: None,
            big_a: None,
            big_x: None,
            big_b: None,
            big_d: None,

            sk_lambda: None,
            bt_lambda: None,

            a_i: None,
            c_i: None,
            k_i: None,

            k_prime_i: None,
            a_prime_i: None,
            b_prime_i: None,
            x_prime_i: None,

            kd_i: None,
            ka_i: None,
            xb_i: None,

            kd_sum: None,
            ka_sum: None,
            xb_sum: None,

            big_r: None,
            k_final: None,
            sigma_i_final: None,
        }
    }
}

type From = Participant;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RcvdPresignMessages<C: CSCurve> {
    #[serde(bound = "C::Scalar: Serialize + for<'d> Deserialize<'d>")]
    pub wait_0: HashMap<From, <C as CurveArithmetic>::Scalar>,
    pub wait_1: HashMap<From, (ScalarPrimitive<C>, ScalarPrimitive<C>)>,
}

impl<C: CSCurve> RcvdPresignMessages<C> {
    pub fn new() -> Self {
        Self {
            wait_0: HashMap::new(),
            wait_1: HashMap::new(),
        }
    }
}

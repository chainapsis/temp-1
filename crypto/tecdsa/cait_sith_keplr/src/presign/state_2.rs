use elliptic_curve::ScalarPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    compat::CSCurve,
    protocol::Participant,
    triples::{TriplePub, TripleShare},
    KeygenOutput,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct PresignState2<C: CSCurve> {
    pub threshold: usize,

    pub participants: Vec<Participant>,
    pub pub_0: TriplePub<C>,

    pub pub_1: TriplePub<C>,

    pub share_0_0: TripleShare<C>,

    pub share_1_0: TripleShare<C>,

    pub keygen_out: KeygenOutput<C>,

    pub big_kd: Option<C::AffinePoint>,

    pub big_k: Option<C::AffinePoint>,
    pub big_a: Option<C::AffinePoint>,
    pub big_x: Option<C::AffinePoint>,
    pub big_b: Option<C::AffinePoint>,
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

impl<'a, C: CSCurve> PresignState2<C> {
    pub fn new(
        participants: Vec<Participant>,
        threshold: usize,
        keygen_out: KeygenOutput<C>,
        pub_0: TriplePub<C>,
        pub_1: TriplePub<C>,
        share_0_0: TripleShare<C>,
        share_1_0: TripleShare<C>,
    ) -> Self {
        Self {
            threshold,

            participants,

            keygen_out,
            pub_0,
            pub_1,
            share_0_0,
            share_1_0,

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
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct RcvdPresignMessages2<C: CSCurve> {
    pub wait_0: HashMap<From, ScalarPrimitive<C>>,
    pub wait_1: HashMap<From, (ScalarPrimitive<C>, ScalarPrimitive<C>)>,
}

impl<C: CSCurve> RcvdPresignMessages2<C> {
    pub fn new() -> Self {
        Self {
            wait_0: HashMap::new(),
            wait_1: HashMap::new(),
        }
    }
}

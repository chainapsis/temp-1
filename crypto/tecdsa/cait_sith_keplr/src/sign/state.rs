use elliptic_curve::ScalarPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    compat::CSCurve, participants::ParticipantList, protocol::Participant, KeygenOutput,
    PresignOutput,
};

pub struct SignState<C: CSCurve> {
    pub threshold: usize,
    pub participants: ParticipantList,
    pub keygen_out: KeygenOutput<C>,
    pub presignature: PresignOutput<C>,

    pub s_i: Option<C::Scalar>,
}

impl<'a, C: CSCurve> SignState<C> {
    pub fn new(
        participant_list: ParticipantList,
        threshold: usize,
        keygen_out: KeygenOutput<C>,
        presignature: PresignOutput<C>,
    ) -> Self {
        Self {
            participants: participant_list.clone(),
            threshold,
            keygen_out,
            presignature,

            s_i: None,
        }
    }
}

type From = Participant;
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct RcvdSignMessages<C: CSCurve> {
    pub wait_0: HashMap<From, ScalarPrimitive<C>>,
}

impl<C: CSCurve> RcvdSignMessages<C> {
    pub fn new() -> RcvdSignMessages<C> {
        RcvdSignMessages {
            wait_0: HashMap::new(),
        }
    }
}

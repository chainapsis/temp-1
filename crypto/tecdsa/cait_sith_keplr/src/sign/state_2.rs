use serde::{Deserialize, Serialize};

use crate::compat::CSCurve;
use crate::protocol::Participant;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound = "C::Scalar: Serialize + for<'d> Deserialize<'d>")]
pub struct SignState2<C: CSCurve> {
    pub threshold: usize,
    pub participants: Vec<Participant>,
    pub s_i: Option<C::Scalar>,
}

impl<'a, C: CSCurve> SignState2<C> {
    pub fn new(participants: Vec<Participant>, threshold: usize) -> Self {
        Self {
            participants,
            threshold,
            s_i: None,
        }
    }
}

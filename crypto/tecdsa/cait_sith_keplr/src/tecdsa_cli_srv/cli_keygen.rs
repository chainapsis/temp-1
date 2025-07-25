use std::collections::HashMap;

use elliptic_curve::CurveArithmetic;
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

use crate::keyshare::steps_2;
use crate::keyshare::RcvdKeyshareMessages;
use crate::keyshare::{CentralizedKeygenOutput, KeyshareState2};
use crate::protocol::ProtocolError;
use crate::{
    protocol::Participant,
    tecdsa::{keygen_centralized::combine_shares, keygen_centralized::keygen_centralized},
};
use crate::{CSCurve, KeygenOutput};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct ClientKeygenStepOutput<C: CSCurve> {
    pub st_0: KeyshareState2<C>,
    pub msgs_1: RcvdKeyshareMessages<C>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "C::Scalar: Serialize + for<'a> Deserialize<'a>")]
pub struct KeyCombineInput<C: CSCurve> {
    pub shares: HashMap<Participant, <C as CurveArithmetic>::Scalar>,
}

pub struct KeygenClient {}

impl KeygenClient {
    pub fn cli_keygen_centralized() -> Result<CentralizedKeygenOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let threshold = 2;

        keygen_centralized::<Secp256k1>(&participants, threshold)
    }

    pub fn cli_combine_shares(
        key_combine_input: KeyCombineInput<Secp256k1>,
    ) -> Result<<Secp256k1 as CurveArithmetic>::Scalar, ProtocolError> {
        let shares: Vec<(Participant, <Secp256k1 as CurveArithmetic>::Scalar)> =
            key_combine_input.shares.into_iter().collect();

        combine_shares::<Secp256k1>(&shares)
    }

    // interface
    pub fn cli_keygen_step_1() -> Result<ClientKeygenStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let threshold = 2;

        let mut st_0 = KeyshareState2::<Secp256k1>::new(participants.clone(), threshold);

        let mut msgs_1 = RcvdKeyshareMessages::<Secp256k1>::new();

        steps_2::step_1::<Secp256k1>(&mut st_0, None).unwrap();

        let p_0 = st_0.participants[0];

        // Send {p0} => p1
        msgs_1.wait_0.insert(p_0, st_0.commitment.unwrap());

        Ok(ClientKeygenStepOutput { st_0, msgs_1 })
    }

    pub fn cli_keygen_step_2(
        mut st_0: KeyshareState2<Secp256k1>,
        msgs_0: &RcvdKeyshareMessages<Secp256k1>,
    ) -> Result<ClientKeygenStepOutput<Secp256k1>, ProtocolError> {
        let p_0 = st_0.participants[0];

        steps_2::step_2::<Secp256k1>(&mut st_0, &msgs_0, p_0).unwrap();

        let mut msgs_1 = RcvdKeyshareMessages::<Secp256k1>::new();

        msgs_1.wait_1.insert(p_0, st_0.confirmation.unwrap());

        Ok(ClientKeygenStepOutput { st_0, msgs_1 })
    }

    pub fn cli_keygen_step_3(
        mut st_0: KeyshareState2<Secp256k1>,
        _msgs_0: &RcvdKeyshareMessages<Secp256k1>,
    ) -> Result<ClientKeygenStepOutput<Secp256k1>, ProtocolError> {
        let p_0 = st_0.participants[0];

        steps_2::step_3::<Secp256k1>(&mut st_0, p_0).unwrap();

        let mut msgs_1 = RcvdKeyshareMessages::<Secp256k1>::new();

        // 0 => 1
        msgs_1.wait_2.insert(
            p_0,
            (
                st_0.big_f.clone().unwrap(),
                st_0.randomizer.clone().unwrap(),
                st_0.phi_proof.clone().unwrap(),
            ),
        );

        Ok(ClientKeygenStepOutput { st_0, msgs_1 })
    }

    pub fn cli_keygen_step_4(
        mut st_0: KeyshareState2<Secp256k1>,
        _msgs_0: &RcvdKeyshareMessages<Secp256k1>,
    ) -> Result<ClientKeygenStepOutput<Secp256k1>, ProtocolError> {
        let p_0 = st_0.participants[0];
        let p_1 = st_0.participants[1];

        let out_0 = steps_2::step_4(&mut st_0, p_0).unwrap();

        let mut msgs_1 = RcvdKeyshareMessages::<Secp256k1>::new();

        // 0 => {1} (send private)
        let x_i_j_for_1 = out_0.x_i_js.get(&p_1).unwrap();
        msgs_1.wait_3.insert(p_0, x_i_j_for_1.clone());

        Ok(ClientKeygenStepOutput { st_0, msgs_1 })
    }

    pub fn cli_keygen_step_5(
        mut st_0: KeyshareState2<Secp256k1>,
        msgs_0: &RcvdKeyshareMessages<Secp256k1>,
    ) -> Result<KeygenOutput<Secp256k1>, ProtocolError> {
        let p_0 = st_0.participants[0];

        let (sk_0, pk_0) = steps_2::step_5(&mut st_0, &msgs_0, p_0, None).unwrap();

        let keygen_0: KeygenOutput<Secp256k1> = KeygenOutput {
            public_key: pk_0,
            private_share: sk_0,
        };

        Ok(keygen_0)
    }
}

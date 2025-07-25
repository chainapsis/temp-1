use k256::Secp256k1;
use serde::{Deserialize, Serialize};

use crate::keyshare::{steps_2, KeyshareState2, RcvdKeyshareMessages};
use crate::protocol::Participant;
use crate::protocol::ProtocolError;
use crate::{CSCurve, KeygenOutput};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>,
")]
pub struct ServerKeygenStepOutput<C: CSCurve> {
    pub st_1: KeyshareState2<C>,
    pub msgs_0: RcvdKeyshareMessages<C>,
}

pub struct KeygenServer {}

impl KeygenServer {
    pub fn srv_keygen_step_1() -> Result<ServerKeygenStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_1 = participants[1];

        let threshold = 2;

        let mut msgs_0 = RcvdKeyshareMessages::new();

        let mut st_1 = KeyshareState2::<Secp256k1>::new(participants.clone(), threshold);

        steps_2::step_1::<Secp256k1>(&mut st_1, None).unwrap();

        // send_many
        msgs_0.wait_0.insert(p_1, st_1.commitment.unwrap());

        Ok(ServerKeygenStepOutput { st_1, msgs_0 })
    }

    pub fn srv_keygen_step_2(
        mut st_1: KeyshareState2<Secp256k1>,
        msgs_1: &RcvdKeyshareMessages<Secp256k1>,
    ) -> Result<ServerKeygenStepOutput<Secp256k1>, ProtocolError> {
        let p_1 = st_1.participants[1];

        let mut msgs_0 = RcvdKeyshareMessages::<Secp256k1>::new();

        steps_2::step_2::<Secp256k1>(&mut st_1, &msgs_1, p_1).unwrap();

        msgs_0.wait_1.insert(p_1, st_1.confirmation.unwrap());

        Ok(ServerKeygenStepOutput { st_1, msgs_0 })
    }

    pub fn srv_keygen_step_3(
        mut st_1: KeyshareState2<Secp256k1>,
        _msgs_1: &RcvdKeyshareMessages<Secp256k1>,
    ) -> Result<ServerKeygenStepOutput<Secp256k1>, ProtocolError> {
        let p_1 = st_1.participants[1];
        steps_2::step_3::<Secp256k1>(&mut st_1, p_1).unwrap();

        let mut msgs_0 = RcvdKeyshareMessages::<Secp256k1>::new();

        // 1 => 0
        msgs_0.wait_2.insert(
            p_1,
            (
                st_1.big_f.clone().unwrap(),
                st_1.randomizer.clone().unwrap(),
                st_1.phi_proof.clone().unwrap(),
            ),
        );

        Ok(ServerKeygenStepOutput { st_1, msgs_0 })
    }

    pub fn srv_keygen_step_4(
        mut st_1: KeyshareState2<Secp256k1>,
        _msgs_1: &RcvdKeyshareMessages<Secp256k1>,
    ) -> Result<ServerKeygenStepOutput<Secp256k1>, ProtocolError> {
        let p_0 = st_1.participants[0];
        let p_1 = st_1.participants[1];

        let out_0 = steps_2::step_4(&mut st_1, p_1).unwrap();

        let mut msgs_0 = RcvdKeyshareMessages::<Secp256k1>::new();

        // 1 => {0} (send private)
        let x_i_j_for_0 = out_0.x_i_js.get(&p_0).unwrap();
        msgs_0.wait_3.insert(p_1, x_i_j_for_0.clone());

        Ok(ServerKeygenStepOutput { st_1, msgs_0 })
    }

    pub fn srv_keygen_step_5(
        mut st_1: KeyshareState2<Secp256k1>,
        msgs_1: &RcvdKeyshareMessages<Secp256k1>,
    ) -> Result<KeygenOutput<Secp256k1>, ProtocolError> {
        let p_1 = st_1.participants[1];

        let (sk_1, pk_1) = steps_2::step_5(&mut st_1, &msgs_1, p_1, None).unwrap();

        let keygen_1: KeygenOutput<Secp256k1> = KeygenOutput {
            public_key: pk_1,
            private_share: sk_1,
        };

        Ok(keygen_1)
    }
}

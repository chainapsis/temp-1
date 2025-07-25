use elliptic_curve::CurveArithmetic;
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

use crate::protocol::Participant;
use crate::protocol::ProtocolError;
use crate::sign::SignOutput;
use crate::sign::{steps_2, RcvdSignMessages, SignState2};
use crate::{CSCurve, PresignOutput};

pub struct SignClient {}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct ClientSignStepOutput<C: CSCurve> {
    pub st_0: SignState2<C>,
    pub msgs_1: RcvdSignMessages<C>,
}

impl SignClient {
    pub fn sign_step_1(
        msg_hash: <Secp256k1 as CurveArithmetic>::Scalar,
        presig_0: PresignOutput<Secp256k1>,
    ) -> Result<ClientSignStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let threshold = 2;

        if participants.len() < 2 {
            return Err(ProtocolError::Other(
                format!(
                    "participant count cannot be < 2, found: {}",
                    participants.len()
                )
                .into(),
            ));
        };

        let p_0 = participants.get(0).unwrap();

        let mut st_0 = SignState2::<Secp256k1>::new(participants.clone(), threshold);

        let mut msgs_1 = RcvdSignMessages::<Secp256k1>::new();

        let out = steps_2::step_1(&mut st_0, *p_0, msg_hash, presig_0).unwrap();

        msgs_1.wait_0.insert(*p_0, out.s_i);

        Ok(ClientSignStepOutput { st_0, msgs_1 })
    }

    pub fn sign_step_2(
        st_0: &mut SignState2<Secp256k1>,
        msgs_0: &RcvdSignMessages<Secp256k1>,
        presig_0: PresignOutput<Secp256k1>,
    ) -> Result<SignOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        if participants.len() < 2 {
            return Err(ProtocolError::Other(
                format!(
                    "participant count cannot be < 2, found: {}",
                    participants.len()
                )
                .into(),
            ));
        };

        let p_0 = participants.get(0).unwrap();

        let out = steps_2::step_2(st_0, &msgs_0, *p_0, presig_0).unwrap();

        Ok(out)
    }
}

use elliptic_curve::CurveArithmetic;
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

use crate::protocol::Participant;
use crate::protocol::ProtocolError;
use crate::sign::{steps_2, RcvdSignMessages, SignOutput, SignState2};
use crate::{CSCurve, PresignOutput};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct ServerSignStepOutput<C: CSCurve> {
    pub st_1: SignState2<C>,
    pub msgs_0: RcvdSignMessages<C>,
}

pub struct SignServer {}

impl SignServer {
    pub fn sign_step_1(
        msg_hash: <Secp256k1 as CurveArithmetic>::Scalar,
        presig_1: PresignOutput<Secp256k1>,
    ) -> Result<ServerSignStepOutput<Secp256k1>, ProtocolError> {
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

        let p_1 = participants.get(1).unwrap();

        let mut st_1 = SignState2::<Secp256k1>::new(participants.clone(), threshold);

        let mut msgs_0 = RcvdSignMessages::<Secp256k1>::new();

        let out = steps_2::step_1(&mut st_1, *p_1, msg_hash, presig_1).unwrap();

        msgs_0.wait_0.insert(*p_1, out.s_i);

        Ok(ServerSignStepOutput { st_1, msgs_0 })
    }

    pub fn sign_step_2(
        st_1: &mut SignState2<Secp256k1>,
        msgs_1: &RcvdSignMessages<Secp256k1>,
        presig_1: PresignOutput<Secp256k1>,
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

        let p_1 = participants.get(1).unwrap();

        let out = steps_2::step_2(st_1, &msgs_1, *p_1, presig_1).unwrap();

        Ok(out)
    }
}

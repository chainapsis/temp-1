use elliptic_curve::ScalarPrimitive;
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

use crate::presign::{steps_2, PresignState2, RcvdPresignMessages2};
use crate::protocol::Participant;
use crate::protocol::ProtocolError;
use crate::triples::{TriplePub, TripleShare};
use crate::{CSCurve, KeygenOutput, PresignOutput};

pub struct PresignClient {}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "
    C::Scalar: Serialize + for<'a> Deserialize<'a>,
    C::AffinePoint: Serialize + for<'a> Deserialize<'a>
")]
pub struct PresignStepOutput<C: CSCurve> {
    pub st_0: PresignState2<C>,
    pub msgs_1: RcvdPresignMessages2<C>,
}

impl PresignClient {
    pub fn presign_step_1(
        pub_0: TriplePub<Secp256k1>,
        pub_1: TriplePub<Secp256k1>,
        share_0_0: TripleShare<Secp256k1>,
        share_1_0: TripleShare<Secp256k1>,
        keygen_0: KeygenOutput<Secp256k1>,
    ) -> Result<PresignStepOutput<Secp256k1>, ProtocolError> {
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

        // Spec 1.1
        if threshold > participants.len() {
            return Err(ProtocolError::Other(
                "threshold must be <= participant count".to_string().into(),
            ));
        }

        // NOTE: We omit the check that the new participant set was present for
        // the triple generation, because presumably they need to have been present
        // in order to have shares.

        // Also check that we have enough participants to reconstruct shares.
        // if threshold != p_0_triple_0.1.threshold || threshold != p_1_triple_1.1.threshold {
        //     return Err(ProtocolError::Other(
        //         "New threshold must match the threshold of both triples"
        //             .to_string()
        //             .into(),
        //     ));
        // }

        let mut state_0 = PresignState2::new(
            participants.clone(),
            threshold,
            keygen_0,
            pub_0,
            pub_1,
            share_0_0,
            share_1_0,
        );

        let mut msgs_1 = RcvdPresignMessages2::<Secp256k1>::new();

        let p_0 = participants.get(0).unwrap();

        let out = steps_2::step_1(&mut state_0, *p_0, *p_0).unwrap();

        msgs_1.wait_0.insert(*p_0, out.kd_i.into());

        Ok(PresignStepOutput {
            st_0: state_0,
            msgs_1,
        })
    }

    pub fn presign_step_2(
        mut st_0: PresignState2<Secp256k1>,
    ) -> Result<PresignStepOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants.get(0).unwrap();

        let mut msgs_1 = RcvdPresignMessages2::<Secp256k1>::new();

        let out = steps_2::step_2(&mut st_0).unwrap();

        let ka_i: ScalarPrimitive<Secp256k1> = out.ka_i.into();
        let xb_i: ScalarPrimitive<Secp256k1> = out.xb_i.into();
        msgs_1.wait_1.insert(*p_0, (ka_i, xb_i));

        Ok(PresignStepOutput { st_0, msgs_1 })
    }

    pub fn presign_step_3(
        mut st_0: PresignState2<Secp256k1>,
        msgs_0: &RcvdPresignMessages2<Secp256k1>,
    ) -> Result<PresignOutput<Secp256k1>, ProtocolError> {
        let participants = vec![Participant::from(0u32), Participant::from(1u32)];

        let p_0 = participants.get(0).unwrap();

        let out = steps_2::step_3(&mut st_0, &msgs_0, *p_0).unwrap();

        Ok(out)
    }
}

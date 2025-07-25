use elliptic_curve::{Field, Group};

use super::{PresignState2, RcvdPresignMessages2};
use crate::compat::CSCurve;
use crate::participants::ParticipantList;
use crate::presign::steps::{Step1Output, Step2Output};
use crate::protocol::{Participant, ProtocolError};
use crate::PresignOutput;

pub fn step_1<C: CSCurve>(
    state: &mut PresignState2<C>,
    me: Participant,
    bt_id: Participant,
) -> Result<Step1Output<C>, ProtocolError> {
    let big_k: C::ProjectivePoint = state.pub_0.big_a.into();

    let big_d = state.pub_0.big_b;
    let big_kd = state.pub_0.big_c;

    let big_x: C::ProjectivePoint = state.keygen_out.public_key.into();

    let big_a: C::ProjectivePoint = state.pub_1.big_a.into();
    let big_b: C::ProjectivePoint = state.pub_1.big_b.into();

    let participant_list = ParticipantList::new(&state.participants)
        .ok_or_else(|| {
            ProtocolError::Other(
                "participant list cannot contain duplicates"
                    .to_string()
                    .into(),
            )
        })
        .unwrap();

    let sk_lambda = participant_list.lagrange::<C>(me);
    let bt_lambda = participant_list.lagrange::<C>(bt_id);

    let k_i = state.share_0_0.a;
    let k_prime_i = bt_lambda * k_i;
    let kd_i: C::Scalar = bt_lambda * state.share_0_0.c; // if this is zero, then the broadcast kdi is also zero.

    let a_i = state.share_1_0.a;
    let b_i = state.share_1_0.b;
    let c_i = state.share_1_0.c;
    let a_prime_i = bt_lambda * a_i;
    let b_prime_i = bt_lambda * b_i;

    let x_prime_i = sk_lambda * state.keygen_out.private_share;

    state.big_kd = Some(big_kd);
    state.big_k = Some(big_k.into());
    state.big_a = Some(big_a.into());
    state.big_x = Some(big_x.into());
    state.big_b = Some(big_b.into());
    state.big_d = Some(big_d.into());

    state.sk_lambda = Some(sk_lambda);
    state.bt_lambda = Some(bt_lambda);

    state.a_i = Some(a_i);
    state.c_i = Some(c_i);
    state.k_i = Some(k_i);

    state.k_prime_i = Some(k_prime_i);
    state.a_prime_i = Some(a_prime_i);
    state.b_prime_i = Some(b_prime_i);
    state.x_prime_i = Some(x_prime_i);
    state.kd_i = Some(kd_i);

    let out = Step1Output { kd_i };

    Ok(out)
}

pub fn step_2<C: CSCurve>(state: &mut PresignState2<C>) -> Result<Step2Output<C>, ProtocolError> {
    let k_prime_i = state.k_prime_i.unwrap();
    let a_prime_i = state.a_prime_i.unwrap();
    let b_prime_i = state.b_prime_i.unwrap();
    let x_prime_i = state.x_prime_i.unwrap();

    let ka_i: C::Scalar = k_prime_i + a_prime_i;
    let xb_i: C::Scalar = x_prime_i + b_prime_i;

    state.ka_i = Some(ka_i);
    state.xb_i = Some(xb_i);

    let out = Step2Output { ka_i, xb_i };

    return Ok(out);
}

pub fn step_3<C: CSCurve>(
    state: &mut PresignState2<C>,
    msgs: &RcvdPresignMessages2<C>,
    me: Participant,
) -> Result<PresignOutput<C>, ProtocolError> {
    let kd_i = state.kd_i.unwrap();

    let participant_list = ParticipantList::new(&state.participants)
        .ok_or_else(|| {
            ProtocolError::Other(
                "participant list cannot contain duplicates"
                    .to_string()
                    .into(),
            )
        })
        .unwrap();

    // Spec 2.1 and 2.2
    let mut kd = kd_i;
    for p in participant_list.others(me) {
        let kd_j = *msgs.wait_0.get(&p).unwrap();

        if kd_j.is_zero().into() {
            return Err(ProtocolError::AssertionFailed(
                "Received zero share of kd, indicating a triple wasn't available.".to_string(),
            ));
        }

        kd += C::Scalar::from(kd_j);
    }

    let big_kd = state.big_kd.unwrap();

    // Spec 2.3
    if big_kd != (C::ProjectivePoint::generator() * kd).into() {
        return Err(ProtocolError::AssertionFailed(
            "received incorrect shares of kd".to_string(),
        ));
    }

    let ka_i = state.ka_i.unwrap();
    let xb_i = state.xb_i.unwrap();

    // Spec 2.4 and 2.5
    let mut ka = ka_i;
    let mut xb = xb_i;

    for p in participant_list.others(me) {
        let (ka_j, xb_j) = msgs.wait_1.get(&p).unwrap();

        ka += C::Scalar::from(*ka_j);
        xb += C::Scalar::from(*xb_j);
    }

    let big_k: C::ProjectivePoint = state.big_k.unwrap().into();
    let big_a: C::ProjectivePoint = state.big_a.unwrap().into();
    let big_x: C::ProjectivePoint = state.big_x.unwrap().into();
    let big_b: C::ProjectivePoint = state.big_b.unwrap().into();

    // Spec 2.6
    if (C::ProjectivePoint::generator() * ka != big_k + big_a)
        || (C::ProjectivePoint::generator() * xb != big_x + big_b)
    {
        return Err(ProtocolError::AssertionFailed(
            "received incorrect shares of additive triple phase.".to_string(),
        ));
    }

    let big_d = state.big_d.unwrap();

    // Spec 2.7
    let kd_inv: Option<C::Scalar> = kd.invert().into();
    let kd_inv =
        kd_inv.ok_or_else(|| ProtocolError::AssertionFailed("failed to invert kd".to_string()))?;
    let big_r = (C::ProjectivePoint::from(big_d) * kd_inv).into();

    let sk_lambda = state.sk_lambda.unwrap();
    let bt_lambda = state.bt_lambda.unwrap();

    let a_i = state.a_i.unwrap();
    let c_i = state.c_i.unwrap();

    // Spec 2.8
    let lambda_diff = bt_lambda * sk_lambda.invert().expect("to invert sk_lambda");
    let sigma_i = ka * state.keygen_out.private_share - (xb * a_i - c_i) * lambda_diff;

    let k_i = state.k_i.unwrap();

    Ok(PresignOutput {
        big_r,
        k: k_i * lambda_diff,
        sigma: sigma_i,
    })
}

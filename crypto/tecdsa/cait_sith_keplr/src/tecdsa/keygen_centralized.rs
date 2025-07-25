use elliptic_curve::{group::Curve, CurveArithmetic, Group};
use rand_core::OsRng;

use crate::{
    keyshare::CentralizedKeygenOutput,
    math::Polynomial,
    participants::ParticipantList,
    protocol::{InitializationError, Participant, ProtocolError},
    CSCurve, KeygenOutput,
};

pub fn keygen_centralized<C: CSCurve>(
    participants: &Vec<Participant>,
    threshold: usize,
) -> Result<CentralizedKeygenOutput<C>, ProtocolError> {
    if threshold < 2 {
        return Err(ProtocolError::AssertionFailed(
            "threshold must be >= 2".to_string(),
        ));
    }

    let _participant_list = ParticipantList::new(&participants)
        .ok_or_else(|| {
            InitializationError::BadParameters(
                "participant list cannot contain duplicates".to_string(),
            )
        })
        .unwrap();

    let mut rng = OsRng;
    let f = Polynomial::<C>::random(&mut rng, threshold);

    let big_x_projective = C::ProjectivePoint::generator() * f.evaluate_zero();
    let big_x_affine = big_x_projective.to_affine();

    let mut results = Vec::with_capacity(participants.len());
    for &p in participants {
        let share_i = f.evaluate(&p.scalar::<C>());
        let key_out = KeygenOutput {
            private_share: share_i,
            public_key: big_x_affine,
        };
        results.push(key_out);
    }

    Ok(CentralizedKeygenOutput {
        private_key: f.evaluate_zero(),
        keygen_outputs: results,
    })
}

pub fn combine_shares<C: CSCurve>(
    shares: &[(Participant, <C as CurveArithmetic>::Scalar)],
) -> Result<<C as CurveArithmetic>::Scalar, ProtocolError> {
    if shares.len() < 2 {
        return Err(ProtocolError::Other(
            "Need at least 2 shares to reconstruct".into(),
        ));
    }

    let participants: Vec<Participant> = shares.iter().map(|(p, _)| *p).collect();
    let participant_list = ParticipantList::new(&participants)
        .ok_or_else(|| ProtocolError::Other("participant list contains duplicates".into()))?;

    let mut result = C::Scalar::default();

    for (_, (p_i, share_i)) in shares.iter().enumerate() {
        let lambda_i = participant_list.lagrange::<C>(*p_i);

        result += *share_i * lambda_i;
    }

    Ok(result)
}

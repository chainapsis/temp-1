use elliptic_curve::Scalar;
use elliptic_curve::{Field, Group, ScalarPrimitive};
use magikitten::MeowRng;
use rand_core::{OsRng, RngCore};
use subtle::Choice;
use subtle::ConditionallySelectable;
use subtle::ConstantTimeEq;

use super::batch_random_ot_2::{hash, BatchRandomOTOutputSender};
use super::bits::{BitMatrix, BitVector, ChoiceVector, DoubleBitVector, SquareBitMatrix};
use super::random_ot_extension_2::hash_to_scalar;
use super::random_ot_extension_2::RandomOtExtensionParams;
use super::state::TriplesState;
use super::RcvdTriplesMessages;
use super::{MTAWait0Payload, MTAWait1Payload};
use crate::compat::{CSCurve, SerializablePoint};
use crate::constants::SECURITY_PARAMETER;
use crate::math::Polynomial;
use crate::participants::ParticipantList;
use crate::protocol::{Participant, ProtocolError};
use crate::triples::correlated_ot_extension::CorrelatedOtParams;

pub fn step_1<C: CSCurve>(
    st: &mut TriplesState<C>,
    _msgs: &RcvdTriplesMessages<C>,
    me: Participant,
) -> Result<(), ProtocolError> {
    let mut e_v = vec![];
    let mut f_v = vec![];

    for i in 0..st.triples_count {
        // let e = &e_v[i];
        let e_cofficients = st.e_v_coefficients[i].clone();
        let e: Polynomial<C> = Polynomial {
            coefficients: e_cofficients,
        };

        let f_cofficients = st.f_v_coefficients[i].clone();
        let f: Polynomial<C> = Polynomial {
            coefficients: f_cofficients,
        };

        e_v.push(e);
        f_v.push(f);
    }

    let e0_v: Vec<_> = e_v.iter().map(|e| e.evaluate_zero()).collect();
    let f0_v: Vec<_> = f_v.iter().map(|f| f.evaluate_zero()).collect();

    st.e0_v = e0_v;
    st.f0_v = f0_v;

    let participant_list = ParticipantList::new(&st.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    for p in participant_list.others(me) {
        if p < me {
            println!("multiplication sender");
        } else {
            println!("multiplication receiver");
        }
    }

    Ok(())
}

pub fn multiplication_receiver_many_step_1<C: CSCurve>(
    st: &mut TriplesState<C>,
    _msgs: &RcvdTriplesMessages<C>,
    _me: Participant,
) -> Result<Vec<Vec<SerializablePoint<C>>>, ProtocolError> {
    // batch random sender
    let mut big_y_v = vec![];
    let mut big_z_v = vec![];
    let mut yv = vec![];

    for _ in 0..st.triples_count {
        // Spec 1
        let y = C::Scalar::random(&mut OsRng);
        let big_y = C::ProjectivePoint::generator() * y;
        let big_z = big_y * y;
        yv.push(y);
        big_y_v.push(big_y);
        big_z_v.push(big_z.into());
    }

    let mut big_y_affine_v = vec![];
    for i in 0..st.triples_count {
        let big_y = &big_y_v[i];
        let big_y_affine = SerializablePoint::<C>::from_projective(&big_y);
        big_y_affine_v.push(big_y_affine);
    }

    st.big_y_affine_v = big_y_affine_v.clone();
    st.yv = yv;
    st.big_z_v = big_z_v;

    Ok(vec![big_y_affine_v])
}

pub fn multiplication_receiver_many_step_2<C: CSCurve>(
    st: &mut TriplesState<C>,
    msgs: &RcvdTriplesMessages<C>,
    me: Participant,
) -> Result<Vec<BitMatrix>, ProtocolError> {
    let dkv = {
        // batch random sender
        let y_v_arc = st.yv.clone();
        let big_y_affine_v_arc = st.big_y_affine_v.clone();
        let big_z_v_arc = st.big_z_v.clone();

        let mut outs: Vec<Vec<(BitVector, BitVector)>> = vec![];
        for i in 0..SECURITY_PARAMETER {
            // println!("111, {}", i);
            let yv_arc = y_v_arc.clone();
            let big_y_affine_v_arc = big_y_affine_v_arc.clone();
            let big_z_v_arc = big_z_v_arc.clone();

            // loop single time
            let mut big_x_i_affine_v = vec![];
            for (_, big_x_i_affine_v_v) in &msgs.batch_random_ot_wait_0 {
                let v = big_x_i_affine_v_v
                    .get(i)
                    .ok_or(ProtocolError::Other(
                        format!("batch_random_ot_wait_0 missing").into(),
                    ))?
                    .clone();
                big_x_i_affine_v = v;
                break;
            }

            let mut ret = vec![];
            for j in 0..st.triples_count {
                let y = &yv_arc.as_slice()[j];
                let big_y_affine = &big_y_affine_v_arc.as_slice()[j];
                let big_z = &big_z_v_arc.as_slice()[j];
                let y_big_x_i = big_x_i_affine_v[j].to_projective() * *y;
                let big_k0 = hash(i, &big_x_i_affine_v[j], big_y_affine, &y_big_x_i);
                let big_k1 = hash(i, &big_x_i_affine_v[j], big_y_affine, &(y_big_x_i - big_z));
                ret.push((big_k0, big_k1));
            }

            outs.push(ret);
        }

        let mut reshaped_outs: Vec<Vec<_>> = Vec::new();
        for _ in 0..st.triples_count {
            reshaped_outs.push(Vec::new());
        }
        for i in 0..outs.len() {
            for j in 0..st.triples_count {
                reshaped_outs[j].push(outs[i][j])
            }
        }
        let outs = reshaped_outs;
        let mut ret: Vec<BatchRandomOTOutputSender> = vec![];
        for i in 0..st.triples_count {
            let out = &outs[i];
            let big_k0: BitMatrix = out.iter().map(|r| r.0).collect();
            let big_k1: BitMatrix = out.iter().map(|r| r.1).collect();
            ret.push((big_k0.try_into().unwrap(), big_k1.try_into().unwrap()));
        }

        ret
    };

    let mut u_v = vec![];
    let mut t_v = vec![];
    let mut b_v = vec![];

    for i in 0..st.triples_count {
        let (k0, k1) = &dkv[i];
        // let a_i = &a_iv[i];
        let a_i = &st.e0_v[i];
        // let b_i = &b_iv[i];
        let b_i = &st.f0_v[i];

        let batch_size = C::BITS + SECURITY_PARAMETER;

        // random_ot_extension_receiver
        let sid = &st.my_confirmations;
        let params = RandomOtExtensionParams {
            sid: sid[i].as_ref(),
            batch_size: 2 * batch_size,
        };

        let adjusted_size = super::random_ot_extension_2::adjust_size(params.batch_size);

        // correlated_ot_sender
        // Step 2
        let correlated_ot_sender_params = CorrelatedOtParams {
            sid: params.sid,
            batch_size: adjusted_size,
        };

        let b = ChoiceVector::random(&mut OsRng, adjusted_size);

        let x: BitMatrix = b
            .bits()
            .map(|b_i| BitVector::conditional_select(&BitVector::zero(), &!BitVector::zero(), b_i))
            .collect();

        // correlated ot receiver
        assert_eq!(x.height(), correlated_ot_sender_params.batch_size);

        // Spec 1
        let t0 = k0.expand_transpose(params.sid, correlated_ot_sender_params.batch_size);
        let t1 = k1.expand_transpose(params.sid, correlated_ot_sender_params.batch_size);

        // Spec 3
        let u = &t0 ^ t1 ^ x;

        u_v.push(u);
        t_v.push(t0);
        b_v.push(b);
    }

    st.t_v = t_v;
    st.b_v = b_v;

    Ok(u_v)
}

pub fn multiplication_receiver_many_step_3<C: CSCurve>(
    st: &mut TriplesState<C>,
    msgs: &RcvdTriplesMessages<C>,
    _me: Participant,
) -> Result<Vec<(DoubleBitVector, Vec<DoubleBitVector>)>, ProtocolError> {
    let batch_size = C::BITS + SECURITY_PARAMETER;
    let adjusted_size = super::random_ot_extension_2::adjust_size(batch_size * 2);

    let random_ot_extension_params = RandomOtExtensionParams {
        sid: &[],
        batch_size: 2 * batch_size,
    };

    // Step 5
    // let seed: [u8; 32] = chan.recv(wait0).await?;
    let mut seed_v = vec![];

    for (p, seed_v_) in &msgs.random_ot_extension_wait_0 {
        seed_v = seed_v_.clone();
        break;
    }

    let mu = adjusted_size / SECURITY_PARAMETER;

    let mut small_x_t_v = vec![];
    let mut res0_v = vec![];
    let mut res1_v = vec![];

    for i in 0..st.triples_count {
        let seed = seed_v[i];
        let b = &st.b_v[i];
        let t = &st.t_v[i];

        // Step 7
        let mut prng = MeowRng::new(&seed);
        let chi: Vec<BitVector> = (0..mu).map(|_| BitVector::random(&mut prng)).collect();

        // Step 8
        let mut small_x = DoubleBitVector::zero();

        for (b_i, chi_i) in b.chunks().zip(chi.iter()) {
            small_x.xor_mut(&b_i.gf_mul(chi_i));
        }

        let small_t: Vec<_> = (0..SECURITY_PARAMETER)
            .map(|j| {
                let mut small_t_j = DoubleBitVector::zero();
                for (t_i, chi_i) in t.column_chunks(j).zip(chi.iter()) {
                    small_t_j ^= t_i.gf_mul(chi_i);
                }
                small_t_j
            })
            .collect();

        // Step 11
        // let wait1 = chan.next_waitpoint();
        // chan.send(wait1, &(small_x, small_t)).await;
        small_x_t_v.push((small_x, small_t));

        // Step 15
        let mut res0: Vec<_> = b
            .bits()
            .zip(t.rows())
            .take(random_ot_extension_params.batch_size)
            .enumerate()
            .map(|(i, (b_i, t_i))| {
                (
                    // b_i.into(),
                    b_i.unwrap_u8(),
                    super::random_ot_extension_2::hash_to_scalar::<C>(i, t_i),
                )
            })
            .collect();

        let res1 = res0.split_off(batch_size);

        res0_v.push(res0);
        res1_v.push(res1);
    }

    st.receiver_res0_v = res0_v;
    st.receiver_res1_v = res1_v;

    Ok(small_x_t_v)
}

pub fn multiplication_receiver_many_step_4<C: CSCurve>(
    st: &mut TriplesState<C>,
    msgs: &RcvdTriplesMessages<C>,
    me: Participant,
) -> Result<MTAWait1Payload<C>, ProtocolError> {
    let participant_list = ParticipantList::new(&st.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    let mut c1_v = vec![];
    let mut c2_v = vec![];

    for (p, payload) in &msgs.mta_wait_0 {
        let MTAWait0Payload { c1_v: c1, c2_v: c2 } = payload;
        c1_v = c1.clone();
        c2_v = c2.clone();
        break;
    }

    let mut chi1_seed_1_v = vec![];
    let mut chi1_seed_2_v = vec![];
    let mut gamma_sum_v = vec![];

    for i in 0..st.triples_count {
        let res0 = st.receiver_res0_v[i].clone();
        let res1 = st.receiver_res1_v[i].clone();

        let res0: Vec<(Choice, Scalar<C>)> = res0.into_iter().map(|(b, s)| (b.into(), s)).collect();
        let res1: Vec<(Choice, Scalar<C>)> = res1.into_iter().map(|(b, s)| (b.into(), s)).collect();

        let a_i = &st.e0_v[i];
        let b_i = &st.f0_v[i];

        let c1 = &c1_v[i];
        let c2 = &c2_v[i];

        let (chi1_seed_1, gamma0) = mta_receiver_step_1(c1, res0, *b_i)?;
        let (chi1_seed_2, gamma1) = mta_receiver_step_1(c2, res1, *a_i)?;

        chi1_seed_1_v.push(chi1_seed_1);
        chi1_seed_2_v.push(chi1_seed_2);

        gamma_sum_v.push(gamma0 + gamma1);
    }

    let mut outs = vec![];
    for i in 0..st.triples_count {
        let av_i = &st.e0_v.as_slice()[i];
        let bv_i = &st.f0_v.as_slice()[i];
        let out = *av_i * *bv_i;
        outs.push(out);
    }

    for _ in participant_list.others(me) {
        for i in 0..st.triples_count {
            outs[i] += gamma_sum_v[i];
        }
    }

    st.l0_v = outs;

    Ok(MTAWait1Payload {
        chi1_seed_1_v,
        chi1_seed_2_v,
    })
}

pub fn multiplication_sender_many_step_1<C: CSCurve>(
    st: &mut TriplesState<C>,
    msgs: &RcvdTriplesMessages<C>,
    _me: Participant,
) -> Result<Vec<Vec<SerializablePoint<C>>>, ProtocolError> {
    // batch random receiver
    let mut big_y_affine_v = vec![];

    for (_p, v_v) in &msgs.batch_random_ot_wait_0 {
        if v_v.len() < 1 {
            return Err(ProtocolError::Other(
                format!("big_y_affine_v_v is empty").into(),
            ));
        }

        big_y_affine_v = v_v[0].clone();
        break;
    }

    if big_y_affine_v.len() < 1 {
        return Err(ProtocolError::Other(
            format!("big_y_affine_v missing").into(),
        ));
    }

    let mut big_y_v = vec![];
    let mut delta_v = vec![];

    for i in 0..st.triples_count {
        let big_y_affine = big_y_affine_v[i];
        let big_y = big_y_affine.to_projective();
        if bool::from(big_y.is_identity()) {
            return Err(ProtocolError::AssertionFailed(
                "Big y in batch random OT was zero.".into(),
            ));
        }

        let delta = BitVector::random(&mut OsRng);
        big_y_v.push(big_y);
        delta_v.push(delta);
    }

    let big_y_v_arc = big_y_v;
    let big_y_affine_v_arc = big_y_affine_v;

    // inner is batch, outer is bits
    let mut choices: Vec<Vec<_>> = Vec::new();
    for _ in delta_v[0].bits() {
        choices.push(Vec::new());
    }
    for j in 0..st.triples_count {
        for (i, d_i) in delta_v[j].bits().enumerate() {
            choices[i].push(d_i);
        }
    }

    let mut outs = vec![];
    let mut big_x_i_affine_v_v = Vec::new();

    for i in 0..choices.len() {
        // let mut chan = chan.child(i as u64);
        // clone arcs
        let d_i_v = choices[i].clone();
        let big_y_v_arc = big_y_v_arc.clone();
        let big_y_affine_v_arc = big_y_affine_v_arc.clone();

        let mut x_i_v = Vec::new();
        let mut big_x_i_v = Vec::new();

        for j in 0..st.triples_count {
            let d_i = d_i_v[j];
            // Step 4
            let x_i = C::Scalar::random(&mut OsRng);
            let mut big_x_i = C::ProjectivePoint::generator() * x_i;
            big_x_i.conditional_assign(&(big_x_i + big_y_v_arc[j]), d_i);
            x_i_v.push(x_i);
            big_x_i_v.push(big_x_i);
        }

        let mut big_x_i_affine_v = Vec::new();
        for j in 0..st.triples_count {
            let big_x_i_affine = SerializablePoint::<C>::from_projective(&big_x_i_v[j]);
            big_x_i_affine_v.push(big_x_i_affine);
        }

        big_x_i_affine_v_v.push(big_x_i_affine_v.clone());

        // Step 5
        let mut hashv = Vec::new();
        for j in 0..st.triples_count {
            let big_x_i_affine = big_x_i_affine_v[j];
            let big_y_affine = big_y_affine_v_arc[j];
            let big_y = big_y_v_arc[j];
            let x_i = x_i_v[j];
            hashv.push(hash(i, &big_x_i_affine, &big_y_affine, &(big_y * x_i)));
        }

        outs.push(hashv);
    }

    // let outs: Vec<Vec<_>> = stream::iter(tasks).then(|t| t).collect().await;
    //
    // batch dimension is on the inside but needs to be on the outside
    let mut reshaped_outs: Vec<Vec<_>> = Vec::new();
    for _ in 0..st.triples_count {
        reshaped_outs.push(Vec::new());
    }
    for i in 0..outs.len() {
        for j in 0..st.triples_count {
            reshaped_outs[j].push(outs[i][j]);
        }
    }
    let outs = reshaped_outs;
    let mut ret = Vec::new();
    for j in 0..st.triples_count {
        let delta = delta_v[j];
        let out = &outs[j];
        let big_k: BitMatrix = out.iter().cloned().collect();
        let h = SquareBitMatrix::try_from(big_k);
        ret.push((delta, h.unwrap()))
    }

    st.dkv = ret;

    Ok(big_x_i_affine_v_v)
}

pub fn multiplication_sender_many_step_2<C: CSCurve>(
    st: &mut TriplesState<C>,
    msgs: &RcvdTriplesMessages<C>,
    _me: Participant,
) -> Result<Vec<[u8; 32]>, ProtocolError> {
    let dkv = &st.dkv;
    let sid = &st.my_confirmations;

    let mut u_v = vec![];
    for (_p, u_v_) in &msgs.correlated_ot_wait_0 {
        u_v = u_v_.clone();
        break;
    }

    let mut q_v = vec![];
    let mut seed_v = vec![];

    for i in 0..st.triples_count {
        let (delta, k) = &dkv[i];
        // let a_i = &a_iv[i];
        let a_i = &st.e0_v[i];
        // let b_i = &b_iv[i];
        let b_i = &st.f0_v[i];

        let batch_size = C::BITS + SECURITY_PARAMETER;

        // random ot extension sender
        // Step 1
        let random_ot_extension_params = RandomOtExtensionParams {
            sid: sid[i].as_ref(),
            batch_size: 2 * batch_size,
        };

        let adjusted_size =
            super::random_ot_extension_2::adjust_size(random_ot_extension_params.batch_size);

        let correlated_ot_params = CorrelatedOtParams {
            sid: random_ot_extension_params.sid,
            batch_size: adjusted_size,
        };

        // correlated ot sender
        // Spec 2
        let t = k.expand_transpose(
            random_ot_extension_params.sid,
            correlated_ot_params.batch_size,
        );

        // Spec 5
        // let wait0 = chan.next_waitpoint();
        // let u: BitMatrix = chan.recv(wait0).await?;
        let u: BitMatrix = u_v[i].clone();

        if u.height() != correlated_ot_params.batch_size {
            return Err(ProtocolError::AssertionFailed(format!(
                "expected matrix of height {} found {}",
                adjusted_size,
                u.height()
            )));
        }

        // Spec 6
        let q = (u & delta) ^ t;

        q_v.push(q);

        // Step 5
        let mut seed = [0u8; 32];
        OsRng.fill_bytes(&mut seed);

        seed_v.push(seed);
    }

    st.q_v = q_v;
    st.seed_v = seed_v.clone();

    Ok(seed_v)
}

pub fn multiplication_sender_many_step_3<C: CSCurve>(
    st: &mut TriplesState<C>,
    msgs: &RcvdTriplesMessages<C>,
    _me: Participant,
) -> Result<(), ProtocolError> {
    let batch_size = C::BITS + SECURITY_PARAMETER;
    let random_ot_extension_params = RandomOtExtensionParams {
        sid: &[],
        batch_size: batch_size * 2,
    };

    let adjusted_size = super::random_ot_extension_2::adjust_size(batch_size * 2);

    let mu = adjusted_size / SECURITY_PARAMETER;

    let mut small_x_t_v = vec![];
    for (_, small_x_t_v_) in &msgs.random_ot_extension_wait_1 {
        small_x_t_v = small_x_t_v_.clone();
        break;
    }

    let mut sender_res0_v = vec![];
    let mut sender_res1_v = vec![];

    for i in 0..st.triples_count {
        let seed = st.seed_v[i];
        // let delta = st.delta_v[i];
        let (delta, k) = &st.dkv[i];
        let q = &st.q_v[i];

        // Step 7
        let mut prng = MeowRng::new(&seed);
        let chi: Vec<BitVector> = (0..mu).map(|_| BitVector::random(&mut prng)).collect();

        // Step 11
        // let wait1 = chan.next_waitpoint();
        // let (small_x, small_t): (DoubleBitVector, Vec<DoubleBitVector>) = chan.recv(wait1).await?;
        let (small_x, small_t) = &small_x_t_v[i];

        // Step 10
        if small_t.len() != SECURITY_PARAMETER {
            return Err(ProtocolError::AssertionFailed(
                "small t of incorrect length".to_owned(),
            ));
        }

        for (j, small_t_j) in small_t.iter().enumerate() {
            let delta_j = Choice::from(delta.bit(j) as u8);

            let mut small_q_j = DoubleBitVector::zero();
            for (q_i, chi_i) in q.column_chunks(j).zip(chi.iter()) {
                small_q_j ^= q_i.gf_mul(chi_i);
            }

            let delta_j_x =
                DoubleBitVector::conditional_select(&DoubleBitVector::zero(), &small_x, delta_j);
            if !bool::from(small_q_j.ct_eq(&(small_t_j ^ delta_j_x))) {
                return Err(ProtocolError::AssertionFailed("q check failed".to_owned()));
            }
        }

        // Step 14
        let mut res0 = Vec::with_capacity(random_ot_extension_params.batch_size);

        for (i, q_i) in q
            .rows()
            .take(random_ot_extension_params.batch_size)
            .enumerate()
        {
            let v0_i = hash_to_scalar::<C>(i, q_i);
            let v1_i = hash_to_scalar::<C>(i, &(q_i ^ delta));
            res0.push((v0_i, v1_i))
        }

        let res1 = res0.split_off(batch_size);
        // println!("res1: {}", res1.len());

        sender_res0_v.push(res0);
        sender_res1_v.push(res1);
    }

    st.sender_res0_v = sender_res0_v;
    st.sender_res1_v = sender_res1_v;

    Ok(())
}

pub fn multiplication_sender_many_step_4<C: CSCurve>(
    st: &mut TriplesState<C>,
    _msgs: &RcvdTriplesMessages<C>,
    _me: Participant,
) -> Result<MTAWait0Payload<C>, ProtocolError> {
    let mut c1_v = vec![];
    let mut c2_v = vec![];

    let mut delta_1_v = vec![];
    let mut delta_2_v = vec![];

    for i in 0..st.triples_count {
        let a_i = &st.e0_v[i];
        let b_i = &st.f0_v[i];

        let res0 = &st.sender_res0_v[i];
        let res1 = &st.sender_res1_v[i];

        let (c1, delta_1) = mta_sender_step_1::<C>(res0.clone(), *a_i)?;
        let (c2, delta_2) = mta_sender_step_1::<C>(res1.clone(), *b_i)?;

        c1_v.push(c1);
        c2_v.push(c2);

        delta_1_v.push(delta_1);
        delta_2_v.push(delta_2);
    }

    st.delta_1_v = delta_1_v;
    st.delta_2_v = delta_2_v;

    Ok(MTAWait0Payload { c1_v, c2_v })
}

pub fn multiplication_sender_many_step_5<C: CSCurve>(
    st: &mut TriplesState<C>,
    msgs: &RcvdTriplesMessages<C>,
    me: Participant,
) -> Result<(), ProtocolError> {
    let participant_list = ParticipantList::new(&st.participants).ok_or_else(|| {
        ProtocolError::Other(
            "participant list cannot contain duplicates"
                .to_string()
                .into(),
        )
    })?;

    let mut chi_seed_1_v = vec![];
    let mut chi_seed_2_v = vec![];

    for (_, payload) in &msgs.mta_wait_1 {
        chi_seed_1_v = payload.chi1_seed_1_v.clone();
        chi_seed_2_v = payload.chi1_seed_2_v.clone();
        break;
    }

    let mut gamma_sum_v = vec![];
    for i in 0..st.triples_count {
        let delta_1 = &st.delta_1_v[i];
        let delta_2 = &st.delta_2_v[i];

        let (chi1_1, seed_1) = chi_seed_1_v[i];
        let (chi1_2, seed_2) = chi_seed_2_v[i];

        let gamma0 = mta_sender_step_2::<C>(delta_1, chi1_1, seed_1)?;
        let gamma1 = mta_sender_step_2::<C>(delta_2, chi1_2, seed_2)?;

        gamma_sum_v.push(gamma0 + gamma1);
    }

    let mut outs = vec![];
    for i in 0..st.triples_count {
        let av_i = &st.e0_v.as_slice()[i];
        let bv_i = &st.f0_v.as_slice()[i];
        let out = *av_i * *bv_i;
        outs.push(out);
    }

    for _ in participant_list.others(me) {
        for i in 0..st.triples_count {
            outs[i] += gamma_sum_v[i];
        }
    }

    st.l0_v = outs;

    Ok(())
}

fn mta_sender_step_1<C: CSCurve>(
    v: Vec<(C::Scalar, C::Scalar)>,
    a: C::Scalar,
) -> Result<
    (
        Vec<(ScalarPrimitive<C>, ScalarPrimitive<C>)>,
        Vec<Scalar<C>>,
    ),
    ProtocolError,
> {
    let size = v.len();

    // Step 1
    let delta: Vec<_> = (0..size).map(|_| C::Scalar::random(&mut OsRng)).collect();

    // Step 2
    let c: Vec<(ScalarPrimitive<C>, ScalarPrimitive<C>)> = delta
        .iter()
        .zip(v.iter())
        .map(|(delta_i, (v0_i, v1_i))| ((*v0_i + delta_i + a).into(), (*v1_i + delta_i - a).into()))
        .collect();

    Ok((c, delta))
}

fn mta_sender_step_2<C: CSCurve>(
    delta: &Vec<Scalar<C>>,
    chi1: Scalar<C>,
    seed: [u8; 32],
) -> Result<Scalar<C>, ProtocolError> {
    let mut alpha = delta[0] * C::Scalar::from(chi1);

    let mut prng = MeowRng::new(&seed);
    for &delta_i in &delta[1..] {
        let chi_i = C::Scalar::random(&mut prng);
        alpha += delta_i * chi_i;
    }

    Ok(-alpha)
}

fn mta_receiver_step_1<C: CSCurve>(
    c: &Vec<(ScalarPrimitive<C>, ScalarPrimitive<C>)>,
    tv: Vec<(Choice, C::Scalar)>,
    b: C::Scalar,
) -> Result<((C::Scalar, [u8; 32]), C::Scalar), ProtocolError> {
    let size = tv.len();

    if c.len() != tv.len() {
        return Err(ProtocolError::AssertionFailed(format!(
            "length of c was incorrect, c: {}, tv: {}",
            c.len(),
            tv.len(),
        ))
        .into());
    }
    let mut m = tv.iter().zip(c.iter()).map(|((t_i, v_i), (c0_i, c1_i))| {
        C::Scalar::conditional_select(&(*c0_i).into(), &(*c1_i).into(), *t_i) - v_i
    });

    // Step 4
    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);
    let mut prng = MeowRng::new(&seed);
    let chi: Vec<C::Scalar> = (1..size).map(|_| C::Scalar::random(&mut prng)).collect();

    let mut chi1 = C::Scalar::ZERO;
    for ((t_i, _), &chi_i) in tv.iter().skip(1).zip(chi.iter()) {
        chi1 += C::Scalar::conditional_select(&chi_i, &(-chi_i), *t_i);
    }
    chi1 = b - chi1;
    chi1.conditional_assign(&(-chi1), tv[0].0);

    // Step 5
    let mut beta = chi1 * m.next().unwrap();
    for (&chi_i, m_i) in chi.iter().zip(m) {
        beta += chi_i * m_i;
    }

    return Ok(((chi1, seed), beta));
}

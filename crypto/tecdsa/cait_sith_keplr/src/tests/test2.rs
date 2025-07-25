use ecdsa::Signature;
use k256::ecdsa::signature::Verifier;
use k256::ecdsa::VerifyingKey;
use k256::{PublicKey, Secp256k1};

use crate::compat;
use crate::tecdsa::triples_2::generate_triples_3;
use crate::{
    compat::scalar_hash,
    protocol::Participant,
    tecdsa::{keygen_2::keygen_2, presign_2::presign_2, sign_2::sign_2},
};

#[test]
fn test_e2e_3() {
    let participants = vec![Participant::from(0u32), Participant::from(1u32)];

    let threshold = 2;

    println!("participants: {:#?}", participants);

    let keygen_result = keygen_2::<Secp256k1>(&participants, threshold).unwrap();

    let triples_participants = vec![Participant::from(0u32), Participant::from(1u32)];
    let triples_result = generate_triples_3::<Secp256k1>(&triples_participants, threshold).unwrap();

    let p_0_keygen_result = keygen_result.get(0).unwrap().clone();
    let p_1_keygen_result = keygen_result.get(1).unwrap().clone();

    let presign_result = {
        let triples_0 = triples_result.get(0).unwrap();
        let triples_1 = triples_result.get(1).unwrap();

        let p_0_pub0 = triples_0.pub_v.get(0).unwrap().clone();
        let p_0_pub1 = triples_0.pub_v.get(1).unwrap().clone();

        let p_0_shares0 = triples_0.share_v.get(0).clone().unwrap();
        let p_0_shares1 = triples_0.share_v.get(1).clone().unwrap();

        let p_1_pub0 = triples_1.pub_v.get(0).unwrap().clone();
        let p_1_pub1 = triples_1.pub_v.get(1).unwrap().clone();

        let p_1_shares0 = triples_1.share_v.get(0).clone().unwrap();
        let p_1_shares1 = triples_1.share_v.get(1).clone().unwrap();

        let p_0_triple_0 = (p_0_shares0.clone(), p_0_pub0);
        let p_0_triple_1 = (p_0_shares1.clone(), p_0_pub1);

        let p_1_triple_0 = (p_1_shares0.clone(), p_1_pub0);
        let p_1_triple_1 = (p_1_shares1.clone(), p_1_pub1);

        presign_2::<Secp256k1>(
            &triples_participants,
            threshold,
            p_0_triple_0,
            p_0_triple_1,
            p_1_triple_0,
            p_1_triple_1,
            p_0_keygen_result.1.clone(),
            p_1_keygen_result.1.clone(),
        )
        .unwrap()
    };

    let msg = b"hello world";
    let sign_result = {
        let msg_hash = scalar_hash(msg);

        let p_0_presignature = presign_result.get(0).unwrap().clone();
        let p_1_presignature = presign_result.get(1).unwrap().clone();

        sign_2::<Secp256k1>(
            &triples_participants,
            threshold,
            p_0_keygen_result.1.clone(),
            p_1_keygen_result.1.clone(),
            p_0_presignature.1.clone(),
            p_1_presignature.1.clone(),
            msg_hash,
        )
        .unwrap()
    };

    {
        // Verify sig!
        let p_0_sig = sign_result.get(0).unwrap().clone().1;
        let p_1_sig = sign_result.get(1).unwrap().clone().1;

        let p_0_public_key = keygen_result.get(0).unwrap().clone().1.public_key;

        assert_eq!(p_0_sig.big_r, p_1_sig.big_r);
        assert_eq!(p_0_sig.s, p_1_sig.s);

        let sig =
            Signature::from_scalars(compat::x_coordinate::<Secp256k1>(&p_0_sig.big_r), p_0_sig.s)
                .unwrap();

        VerifyingKey::from(&PublicKey::from_affine(p_0_public_key).unwrap())
            .verify(&msg[..], &sig)
            .unwrap();
    }
}

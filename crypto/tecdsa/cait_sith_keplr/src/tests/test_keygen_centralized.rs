use ecdsa::Signature;
use k256::ecdsa::signature::Verifier;
use k256::ecdsa::VerifyingKey;
use k256::{ProjectivePoint, PublicKey, Secp256k1};

use crate::compat;
use crate::tecdsa::triples_2::generate_triples_3;
use crate::{
    compat::scalar_hash,
    protocol::Participant,
    tecdsa::{
        keygen_centralized::combine_shares, keygen_centralized::keygen_centralized,
        presign_2::presign_2, sign_2::sign_2,
    },
};

#[test]
fn test_keygen_centralized() {
    let participants = vec![Participant::from(0u32), Participant::from(1u32)];

    let threshold = 2;

    let keygen_result = keygen_centralized::<Secp256k1>(&participants, threshold).unwrap();

    let triples_participants = vec![Participant::from(0u32), Participant::from(1u32)];
    let triples_result = generate_triples_3::<Secp256k1>(&triples_participants, threshold).unwrap();

    let p_0_keygen_result = keygen_result.keygen_outputs.get(0).unwrap().clone();
    let p_1_keygen_result = keygen_result.keygen_outputs.get(1).unwrap().clone();

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
            p_0_keygen_result.clone(),
            p_1_keygen_result.clone(),
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
            p_0_keygen_result.clone(),
            p_1_keygen_result.clone(),
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

        let p_0_public_key = keygen_result
            .keygen_outputs
            .get(0)
            .unwrap()
            .clone()
            .public_key;

        assert_eq!(p_0_sig.big_r, p_1_sig.big_r);
        assert_eq!(p_0_sig.s, p_1_sig.s);

        let sig =
            Signature::from_scalars(compat::x_coordinate::<Secp256k1>(&p_0_sig.big_r), p_0_sig.s)
                .unwrap();

        VerifyingKey::from(&PublicKey::from_affine(p_0_public_key).unwrap())
            .verify(&msg[..], &sig)
            .unwrap();
    }

    // combine test
    {
        let shares_to_combine = vec![
            (Participant::from(0u32), p_0_keygen_result.private_share),
            (Participant::from(1u32), p_1_keygen_result.private_share),
        ];

        // recover secret key
        let recovered_priv = combine_shares::<Secp256k1>(&shares_to_combine).unwrap();
        // recover public key
        let recovered_pub = (ProjectivePoint::GENERATOR * recovered_priv).to_affine();

        let expected_pub = keygen_result.keygen_outputs.get(0).unwrap().public_key;
        assert_eq!(recovered_pub, expected_pub);
        println!(
            "\n recovered_pub: {:#?}, expected_pub: {:#?}",
            recovered_pub, expected_pub
        );
    }
}

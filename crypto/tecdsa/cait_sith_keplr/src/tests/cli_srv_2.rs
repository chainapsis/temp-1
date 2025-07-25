use ecdsa::Signature;
use k256::Secp256k1;

use crate::compat::{self, scalar_hash};
use crate::keyshare::RcvdKeyshareMessages;
use crate::presign::RcvdPresignMessages2;
use crate::sign::{RcvdSignMessages, SignOutput};
use crate::tecdsa_cli_srv::cli_keygen::{ClientKeygenStepOutput, KeygenClient};
use crate::tecdsa_cli_srv::cli_presign::{PresignClient, PresignStepOutput};
use crate::tecdsa_cli_srv::cli_sign::{ClientSignStepOutput, SignClient};
use crate::tecdsa_cli_srv::cli_triples::{TriplesCliStepOutput, TriplesClient};
use crate::tecdsa_cli_srv::srv_keygen::{KeygenServer, ServerKeygenStepOutput};
use crate::tecdsa_cli_srv::srv_presign::PresignServer;
use crate::tecdsa_cli_srv::srv_sign::{ServerSignStepOutput, SignServer};
use crate::tecdsa_cli_srv::srv_triples::{TriplesServer2, TriplesSrvStepOutput};
use crate::tecdsa_cli_srv::verifier::verify_sig;
use crate::triples::{RcvdTriplesMessages, TriplePub, TripleShare};
use crate::KeygenOutput;

struct ClientKeygenResult {
    pub keygen_0: KeygenOutput<Secp256k1>,
}

struct ServerKeygenResult {
    pub keygen_1: KeygenOutput<Secp256k1>,
}

struct ClientTriplesResult {
    pub pub_0: TriplePub<Secp256k1>,
    pub pub_1: TriplePub<Secp256k1>,
    pub share_0_0: TripleShare<Secp256k1>,
    pub share_1_0: TripleShare<Secp256k1>,
}

struct ServerTriplesResult {
    pub pub_0: TriplePub<Secp256k1>,
    pub pub_1: TriplePub<Secp256k1>,
    pub share_0_1: TripleShare<Secp256k1>,
    pub share_1_1: TripleShare<Secp256k1>,
}

#[test]
pub fn test_e2e_cli_srv_2() {
    let (cli_keygen_res, srv_keygen_res) = {
        let mut cli_msgs_0 = RcvdKeyshareMessages::new();
        let mut srv_msgs_1 = RcvdKeyshareMessages::new();

        // browser js call
        let cli_keygen_step_1_out = KeygenClient::cli_keygen_step_1().unwrap();
        println!("\ncli_keygen_step_1_out: {:?}", cli_keygen_step_1_out);

        // cli => srv (step 1)
        let ClientKeygenStepOutput { st_0, msgs_1 } = cli_keygen_step_1_out;

        for (p, data) in msgs_1.wait_0.into_iter() {
            println!("Send msg, wait_0, from: {:?}, to: 1", p);
            srv_msgs_1.wait_0.insert(p, data);
        }

        // node js call
        let srv_keygen_step_1_out = KeygenServer::srv_keygen_step_1().unwrap();
        println!("\nrecv_keygen_step_1_out: {:?}", srv_keygen_step_1_out);

        // srv => cli (step 1, wait 0)
        let ServerKeygenStepOutput { st_1, msgs_0 } = srv_keygen_step_1_out;

        for (p, data) in msgs_0.wait_0.into_iter() {
            println!("Send msg, wait_0, from: {:?}, to: 0", p);
            cli_msgs_0.wait_0.insert(p, data);
        }

        // browser js call
        let cli_keygen_step_2_out = KeygenClient::cli_keygen_step_2(st_0, &cli_msgs_0).unwrap();
        println!("\ncli_keygen_step_2_out: {:?}", cli_keygen_step_2_out);

        // cli => srv (step 2)
        let ClientKeygenStepOutput { st_0, msgs_1 } = cli_keygen_step_2_out;

        for (p, data) in msgs_1.wait_1.into_iter() {
            println!("Send msg, wait_1, from: {:?}, to: 1", p);
            srv_msgs_1.wait_1.insert(p, data);
        }

        // node js call
        let srv_keygen_step_2_out = KeygenServer::srv_keygen_step_2(st_1, &srv_msgs_1).unwrap();
        println!("\nsrv_keygen_step_2_out: {:?}", srv_keygen_step_2_out);

        // srv => cli (step 2)
        let ServerKeygenStepOutput { st_1, msgs_0 } = srv_keygen_step_2_out;

        for (p, data) in msgs_0.wait_1.into_iter() {
            println!("Send msg, wait_1, from: {:?}, to: 0", p);
            cli_msgs_0.wait_1.insert(p, data);
        }

        // browser js call
        let cli_keygen_step_3_out = KeygenClient::cli_keygen_step_3(st_0, &cli_msgs_0).unwrap();
        println!("\n cli_keygen_step_3_out: {:?}", cli_keygen_step_3_out);

        // cli => srv (step 3)
        let ClientKeygenStepOutput { st_0, msgs_1 } = cli_keygen_step_3_out;

        for (p, data) in msgs_1.wait_2.into_iter() {
            println!("Send msg, wait_2, from: {:?}, to: 1", p);
            srv_msgs_1.wait_2.insert(p, data);
        }

        // node js call
        let srv_keygen_step_3_out = KeygenServer::srv_keygen_step_3(st_1, &srv_msgs_1).unwrap();
        println!("\n srv_keygen_step_3_out: {:?}", srv_keygen_step_3_out);

        // srv => cli (step 3)
        let ServerKeygenStepOutput { st_1, msgs_0 } = srv_keygen_step_3_out;

        for (p, data) in msgs_0.wait_2.into_iter() {
            println!("Send msg, wait_2, from: {:?}, to: 0", p);
            cli_msgs_0.wait_2.insert(p, data);
        }

        // browser js call
        let cli_keygen_step_4_out = KeygenClient::cli_keygen_step_4(st_0, &cli_msgs_0).unwrap();
        println!("\n cli_keygen_step_4_out: {:?}", cli_keygen_step_4_out);

        // cli => srv (step 4)
        let ClientKeygenStepOutput { st_0, msgs_1 } = cli_keygen_step_4_out;

        for (p, data) in msgs_1.wait_3.into_iter() {
            println!("Keygen Send msg, wait_3, from: {:?}, to: 1", p);
            srv_msgs_1.wait_3.insert(p, data);
        }

        // node js call
        let srv_keygen_step_4_out = KeygenServer::srv_keygen_step_4(st_1, &srv_msgs_1).unwrap();
        println!("\n srv_keygen_step_4_out: {:?}", srv_keygen_step_4_out);

        // srv => cli (step 4)
        let ServerKeygenStepOutput { st_1, msgs_0 } = srv_keygen_step_4_out;

        for (p, data) in msgs_0.wait_3.into_iter() {
            println!("Keygen Send msg, wait_3, from: {:?}, to: 0", p);
            cli_msgs_0.wait_3.insert(p, data);
        }

        // browser js call
        let keygen_0 = KeygenClient::cli_keygen_step_5(st_0, &cli_msgs_0).unwrap();
        println!("keygen_0: {:?}", keygen_0);

        let keygen_1 = KeygenServer::srv_keygen_step_5(st_1, &srv_msgs_1).unwrap();
        println!("keygen_1: {:?}", keygen_1);

        let cli_res = ClientKeygenResult { keygen_0 };
        let srv_res = ServerKeygenResult { keygen_1 };

        (cli_res, srv_res)
    };

    let (cli_triples_res, srv_triples_res) = {
        let mut cli_msgs_0 = RcvdTriplesMessages::<Secp256k1>::new();
        let mut srv_msgs_2 = RcvdTriplesMessages::<Secp256k1>::new();

        // browser call
        let cli_triples_step_1_out = TriplesClient::triples_step_1().unwrap();
        let TriplesCliStepOutput { st_0, msgs_1 } = cli_triples_step_1_out;

        // cli => srv (step 1)
        for (p, data) in msgs_1.wait_0.into_iter() {
            println!("Triples Send msg, wait_0, from: {:?}, to: 1", p);
            srv_msgs_2.wait_0.insert(p, data);
        }

        // node js call
        let srv_triples_step_1_out = TriplesServer2::triples_step_1().unwrap();
        let TriplesSrvStepOutput { st_1, msgs_0 } = srv_triples_step_1_out;

        // srv => cli (step 1)
        for (p, data) in msgs_0.wait_0.into_iter() {
            println!("Triples Send msg, wait_0, from: {:?}, to: 0", p);
            cli_msgs_0.wait_0.insert(p, data);
        }

        // browser call
        let cli_triples_step_2_out =
            TriplesClient::triples_step_2(st_0, cli_msgs_0.clone()).unwrap();
        let TriplesCliStepOutput { st_0, msgs_1 } = cli_triples_step_2_out;

        // cli => srv (step 2)
        for (p, data) in msgs_1.wait_1.into_iter() {
            println!("Triples Send msg, wait_1, from: {:?}, to: 1", p);
            srv_msgs_2.wait_1.insert(p, data);
        }

        // node js call
        let srv_triples_step_2_out =
            TriplesServer2::triples_step_2(st_1, srv_msgs_2.clone()).unwrap();
        let TriplesSrvStepOutput { st_1, msgs_0 } = srv_triples_step_2_out;

        // srv => cli (step 2)
        for (p, data) in msgs_0.wait_1.into_iter() {
            println!("Triples Send msg, wait_1, from: {:?}, to: 0", p);
            cli_msgs_0.wait_1.insert(p, data);
        }

        // browser call
        let cli_triples_step_3_out =
            TriplesClient::triples_step_3(st_0, cli_msgs_0.clone()).unwrap();
        let TriplesCliStepOutput { st_0, msgs_1 } = cli_triples_step_3_out;

        // cli => srv (step 2)
        for (p, data) in msgs_1.wait_2.into_iter() {
            println!("Triples Send msg, wait_2, from: {:?}, to: 1", p);
            srv_msgs_2.wait_2.insert(p, data);
        }

        // node js call
        let srv_triples_step_3_out =
            TriplesServer2::triples_step_3(st_1, srv_msgs_2.clone()).unwrap();
        let TriplesSrvStepOutput { st_1, msgs_0 } = srv_triples_step_3_out;

        // srv => cli (step 3)
        for (p, data) in msgs_0.wait_2.into_iter() {
            println!("Triples Send msg, wait_2, from: {:?}, to: 0", p);
            cli_msgs_0.wait_2.insert(p, data);
        }

        // browser call
        let cli_triples_step_4_out =
            TriplesClient::triples_step_4(st_0, cli_msgs_0.clone()).unwrap();
        let TriplesCliStepOutput { st_0, msgs_1 } = cli_triples_step_4_out;

        // cli => srv (step 4)
        for (p, data) in msgs_1.wait_3.into_iter() {
            println!("Triples Send msg, wait_3, from: {:?}, to: 1", p);
            srv_msgs_2.wait_3.insert(p, data);
        }

        // node js call
        let srv_triples_step_4_out =
            TriplesServer2::triples_step_4(st_1, srv_msgs_2.clone()).unwrap();
        let TriplesSrvStepOutput { st_1, msgs_0 } = srv_triples_step_4_out;

        // srv => cli (step 4)
        for (p, data) in msgs_0.wait_3.into_iter() {
            println!("Triples Send msg, wait_3, from: {:?}, to: 0", p);
            cli_msgs_0.wait_3.insert(p, data);
        }

        // browser call
        let cli_triples_step_5_out =
            TriplesClient::triples_step_5(st_0, cli_msgs_0.clone()).unwrap();
        let TriplesCliStepOutput { st_0, msgs_1 } = cli_triples_step_5_out;

        // cli => srv (step 5)
        for (p, data) in msgs_1.wait_4.into_iter() {
            println!("Triples Send msg, wait_4, from: {:?}, to: 1", p);
            srv_msgs_2.wait_4.insert(p, data);
        }

        // node js call
        let srv_triples_step_5_out =
            TriplesServer2::triples_step_5(st_1, srv_msgs_2.clone()).unwrap();
        let TriplesSrvStepOutput { st_1, msgs_0 } = srv_triples_step_5_out;

        // srv => cli (step 5)
        for (p, data) in msgs_0.wait_4.into_iter() {
            println!("Triples Send msg, wait_4, from: {:?}, to: 0", p);
            cli_msgs_0.wait_4.insert(p, data);
        }

        // browser call
        let cli_triples_step_6_out =
            TriplesClient::triples_step_6(st_0, cli_msgs_0.clone()).unwrap();
        let TriplesCliStepOutput { st_0, msgs_1 } = cli_triples_step_6_out;

        // cli => srv (step 6)
        for (p, data) in msgs_1.batch_random_ot_wait_0.into_iter() {
            println!(
                "Triples Send msg, batch_random_ot_wait_0, from: {:?}, to: 1",
                p
            );
            srv_msgs_2.batch_random_ot_wait_0.insert(p, data);
        }

        // node js call
        let srv_triples_step_6_out =
            TriplesServer2::triples_step_6(st_1, srv_msgs_2.clone()).unwrap();
        let TriplesSrvStepOutput { st_1, msgs_0 } = srv_triples_step_6_out;

        // srv => cli (step 6)
        for (p, data) in msgs_0.batch_random_ot_wait_0.into_iter() {
            println!(
                "Triples Send msg, batch_random_ot_wait_0, from: {:?}, to: 0",
                p
            );
            cli_msgs_0.batch_random_ot_wait_0.insert(p, data);
        }

        // browser call
        let cli_triples_step_7_out =
            TriplesClient::triples_step_7(st_0, cli_msgs_0.clone()).unwrap();
        let TriplesCliStepOutput { st_0, msgs_1 } = cli_triples_step_7_out;

        // cli => srv (step 7)
        for (p, data) in msgs_1.correlated_ot_wait_0.into_iter() {
            println!(
                "Triples Send msg, correlated_ot_wait_0, from: {:?}, to: 1",
                p
            );
            srv_msgs_2.correlated_ot_wait_0.insert(p, data);
        }

        // node js call
        let srv_triples_step_7_out =
            TriplesServer2::triples_step_7(st_1, srv_msgs_2.clone()).unwrap();
        let TriplesSrvStepOutput { st_1, msgs_0 } = srv_triples_step_7_out;

        // srv => cli (step 7)
        for (p, data) in msgs_0.random_ot_extension_wait_0.into_iter() {
            println!(
                "Triples Send msg, random_ot_extension_wait_0, from: {:?}, to: 0",
                p
            );
            cli_msgs_0.random_ot_extension_wait_0.insert(p, data);
        }

        // browser call
        let cli_triples_step_8_out =
            TriplesClient::triples_step_8(st_0, cli_msgs_0.clone()).unwrap();
        let TriplesCliStepOutput { st_0, msgs_1 } = cli_triples_step_8_out;

        // cli => srv (step 8)
        for (p, data) in msgs_1.random_ot_extension_wait_1.into_iter() {
            println!(
                "Triples Send msg, random_ot_extension_wait_1, from: {:?}, to: 1",
                p
            );
            srv_msgs_2.random_ot_extension_wait_1.insert(p, data);
        }

        // node js call
        let srv_triples_step_8_out =
            TriplesServer2::triples_step_8(st_1, srv_msgs_2.clone()).unwrap();
        let TriplesSrvStepOutput { st_1, msgs_0 } = srv_triples_step_8_out;

        // srv => cli (step 8)
        for (p, data) in msgs_0.mta_wait_0.into_iter() {
            println!("Triples Send msg, mta_wait_0, from: {:?}, to: 0", p);
            cli_msgs_0.mta_wait_0.insert(p, data);
        }

        // browser call
        let cli_triples_step_9_out =
            TriplesClient::triples_step_9(st_0, cli_msgs_0.clone()).unwrap();
        let TriplesCliStepOutput { st_0, msgs_1 } = cli_triples_step_9_out;

        // cli => srv (step 9)
        for (p, data) in msgs_1.mta_wait_1.into_iter() {
            println!("Triples Send msg, mta_wait_1, from: {:?}, to: 1", p);
            srv_msgs_2.mta_wait_1.insert(p, data);
        }

        // node js call (step 9)
        let TriplesSrvStepOutput { st_1, .. } =
            TriplesServer2::triples_step_9(st_1, srv_msgs_2.clone()).unwrap();

        // browser call
        let cli_triples_step_10_out =
            TriplesClient::triples_step_10(st_0, cli_msgs_0.clone()).unwrap();
        let TriplesCliStepOutput { st_0, msgs_1 } = cli_triples_step_10_out;

        // cli => srv (step 10)
        for (p, data) in msgs_1.wait_5.into_iter() {
            println!("Triples Send msg, wait_5, from: {:?}, to: 1", p);
            srv_msgs_2.wait_5.insert(p, data);
        }

        // cli => srv (step 10)
        for (p, data) in msgs_1.wait_6.into_iter() {
            println!("Triples Send msg, wait_6, from: {:?}, to: 1", p);
            srv_msgs_2.wait_6.insert(p, data);
        }

        // node js call
        let srv_triples_step_10_out =
            TriplesServer2::triples_step_10(st_1, srv_msgs_2.clone()).unwrap();
        let TriplesSrvStepOutput { st_1, msgs_0 } = srv_triples_step_10_out;

        // srv => cli (step 10)
        for (p, data) in msgs_0.wait_5.into_iter() {
            println!("Triples Send msg, wait_5, from: {:?}, to: 0", p);
            cli_msgs_0.wait_5.insert(p, data);
        }

        // srv => cli (step 10)
        for (p, data) in msgs_0.wait_6.into_iter() {
            println!("Triples Send msg, wait_6, from: {:?}, to: 0", p);
            cli_msgs_0.wait_6.insert(p, data);
        }

        let cli_res = TriplesClient::triples_step_11(st_0, cli_msgs_0.clone()).unwrap();
        let srv_res = TriplesServer2::triples_step_11(st_1, srv_msgs_2.clone()).unwrap();

        // cli
        let cli_res = ClientTriplesResult {
            pub_0: cli_res.pub_v.get(0).unwrap().clone(),
            pub_1: cli_res.pub_v.get(1).unwrap().clone(),

            share_0_0: cli_res.share_v.get(0).unwrap().clone(),
            share_1_0: cli_res.share_v.get(1).unwrap().clone(),
        };

        // srv
        let srv_res = ServerTriplesResult {
            pub_0: srv_res.pub_v.get(0).unwrap().clone(),
            pub_1: srv_res.pub_v.get(1).unwrap().clone(),

            share_0_1: srv_res.share_v.get(0).unwrap().clone(),
            share_1_1: srv_res.share_v.get(1).unwrap().clone(),
        };

        (cli_res, srv_res)
    };

    let (cli_presign_res, srv_presign_res) = {
        let mut cli_msgs_0 = RcvdPresignMessages2::<Secp256k1>::new();
        let mut srv_msgs_2 = RcvdPresignMessages2::<Secp256k1>::new();

        // browser call
        let cli_presign_step_1_out = PresignClient::presign_step_1(
            cli_triples_res.pub_0,
            cli_triples_res.pub_1,
            cli_triples_res.share_0_0,
            cli_triples_res.share_1_0,
            cli_keygen_res.keygen_0.clone(),
        )
        .unwrap();

        // cli => srv (step 1)
        let PresignStepOutput { st_0, msgs_1 } = cli_presign_step_1_out;

        for (p, data) in msgs_1.wait_0.into_iter() {
            println!("Presign Send msg, wait_0, from: {:?}, to: 1", p);
            srv_msgs_2.wait_0.insert(p, data);
        }

        // node js call
        let srv_presign_step_1_out = PresignServer::presign_step_1(
            srv_triples_res.pub_0,
            srv_triples_res.pub_1,
            srv_triples_res.share_0_1,
            srv_triples_res.share_1_1,
            srv_keygen_res.keygen_1.clone(),
        )
        .unwrap();

        // srv => cli (step 1)
        let (st_2, msgs_0) = srv_presign_step_1_out;

        for (p, data) in msgs_0.wait_0.into_iter() {
            println!("Presign Send msg, wait_0, from: {:?}, to: 0", p);
            cli_msgs_0.wait_0.insert(p, data);
        }

        // browser call
        let cli_presign_step_2_out = PresignClient::presign_step_2(st_0).unwrap();

        // cli => srv (step 2)
        let PresignStepOutput { st_0, msgs_1 } = cli_presign_step_2_out;

        for (p, data) in msgs_1.wait_1.into_iter() {
            println!("Presign Send msg, wait_1, from: {:?}, to: 1", p);
            srv_msgs_2.wait_1.insert(p, data);
        }

        // node js call
        let srv_presign_step_2_out = PresignServer::presign_step_2(st_2).unwrap();

        let (st_2, msgs_0) = srv_presign_step_2_out;

        for (p, data) in msgs_0.wait_1.into_iter() {
            println!("Presign Send msg, wait_1, from: {:?}, to: 0", p);
            cli_msgs_0.wait_1.insert(p, data);
        }

        // browser call
        let cli_presign_res = PresignClient::presign_step_3(st_0, &cli_msgs_0).unwrap();

        // node js call
        let srv_presign_res = PresignServer::presign_step_3(st_2, &srv_msgs_2).unwrap();

        (cli_presign_res, srv_presign_res)
    };

    {
        let mut cli_msgs_0 = RcvdSignMessages::<Secp256k1>::new();
        let mut srv_msgs_2 = RcvdSignMessages::<Secp256k1>::new();

        let msg = b"some";
        let msg_hash = scalar_hash(msg);

        // browser call
        let cli_sign_step_1_out =
            SignClient::sign_step_1(msg_hash, cli_presign_res.clone()).unwrap();

        // cli => srv (step 1)
        let ClientSignStepOutput { mut st_0, msgs_1 } = cli_sign_step_1_out;

        for (p, data) in msgs_1.wait_0.into_iter() {
            println!("Sign Send msg, wait_0, from: {:?}, to: 1", p);
            srv_msgs_2.wait_0.insert(p, data);
        }

        // node js call
        let srv_sign_step_1_out =
            SignServer::sign_step_1(msg_hash, srv_presign_res.clone()).unwrap();

        // srv => cli (step 1)
        let ServerSignStepOutput { mut st_1, msgs_0 } = srv_sign_step_1_out;

        for (p, data) in msgs_0.wait_0.into_iter() {
            println!("Sign Send msg, wait_0, from: {:?}, to: 0", p);
            cli_msgs_0.wait_0.insert(p, data);
        }

        // browser call
        let SignOutput { sig: sig_0, .. } =
            SignClient::sign_step_2(&mut st_0, &cli_msgs_0, cli_presign_res).unwrap();

        println!("cli sig: {:?}", sig_0);

        // node js call
        let sig_2 = SignServer::sign_step_2(&mut st_1, &srv_msgs_2, srv_presign_res).unwrap();

        println!("srv sig: {:?}", sig_2);

        let sig: Signature<Secp256k1> =
            Signature::from_scalars(compat::x_coordinate::<Secp256k1>(&sig_0.big_r), sig_0.s)
                .unwrap();

        verify_sig(sig, cli_keygen_res.keygen_0.public_key, msg).unwrap();
    }
}

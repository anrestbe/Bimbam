//! This utility generates data used for testing Sway's ec_recover function (stdlib/ecr.sw).
//! NOT to be used for key-generation as this is NEITHER SECURE NOR RANDOM !!!

use std::str::FromStr;

use anyhow::Result;
use secp256k1_test::{Message as SecpMessage, Secp256k1};

use secp256k1_test::{
    recovery::{RecoverableSignature, RecoveryId},
    Message,
};
// use sha256::digest_bytes;
use sha3::{Digest, Keccak256};

fn main() -> Result<()> {
    let secp = Secp256k1::new();
    let message_arr = "It's a small(er) world";
    // Got keys from:
    // https://github.com/bluealloy/revm/blob/2e96f08897078ccb8b0f520b110c180ad6f77219/bins/revme/src/statetest/runner.rs#L82-L84
    let secret_key = secp256k1_test::key::SecretKey::from_str("45a915e4d060149eb4365960e6a7a45f334393093061116b197e3240065ff2d8")?;
    // secret key matched eth address: 0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b
    
    // https://emn178.github.io/online-tools/keccak_256.html good
    let message_hashed = Keccak256::digest(message_arr).to_vec();
    let mut hash = [0; 32];
    hash[..].copy_from_slice(&message_hashed);

    let message = SecpMessage::from_slice(&hash).unwrap();
    // @note sign_recoverable sig is not 128 bytes long! (130 bytes)
    let signature = secp.sign_recoverable(&message, &secret_key);
    let (rec_id, data) = signature.serialize_compact();
	let mut sig = [0; 65];

	// no need to check if s is low, it always is
	sig[0..64].copy_from_slice(&data[0..64]);
	sig[64] = rec_id.to_i32() as u8;

    
    println!("private key: {}", secret_key);
    println!("message: {:?}", message);
    println!("Signature: {:?} \n", hex::encode(sig));

    //recovery
    let (rec,eth) = secp256k1_ecdsa_recover(&sig,&hash)?;
    println!("rec key:{:?}",hex::encode(rec));
    println!("eth key:{:?}",hex::encode(eth));

    Ok(())
}

fn secp256k1_ecdsa_recover(
    sig: &[u8; 65],
    msg: &[u8; 32],
) -> Result<(Vec<u8>,Vec<u8>), secp256k1_test::Error> {
    let sig = RecoverableSignature::from_compact(
        &sig[0..64],
        RecoveryId::from_i32((sig[64]) as i32)?,
    )?;

    let secp = Secp256k1::new();
    let public = secp.recover(&Message::from_slice(&msg[..32])?, &sig)?;

    let mut eth = vec![0; 20];
    let rec = &public.serialize_uncompressed();
    eth.copy_from_slice(&Keccak256::digest(&rec[1..])[12..]);
    Ok((rec.to_vec(),eth))
}

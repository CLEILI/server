use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
};
use cipher::generic_array::GenericArray;
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};
//use ipfs_unixfs::file::adder::{Chunker, FileAdder};
use ipfs_unixfs::file::adder::{Chunker, FileAdder};
use std::any::type_name;
use zkcore::{Inputs, Outputs};
fn type_name_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}
fn main() {
    // read the input
    let input: Inputs = env::read();
    println!("4{:?}", input.key);
    println!("5{:?}", input.nonce);
    //env::log("ok");

    let key = GenericArray::from(input.key);
    println!("{:?}", input.key);
    //let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = GenericArray::from(input.nonce); // 96-bits; unique per message
    println!("{:?}", input.nonce);
    //env::log("ok2");
    let ciphertext = cipher.encrypt(&nonce, input.data.as_ref()).unwrap();
    println!("ciphertext: {}", ciphertext[2]);
    println!("key: {:?}", type_name_of(&key));
    println!("nonce: {:?}", type_name_of(&nonce));
    //the key is a 32bytes array
    //the host generate the key and nonce
    //let n=AesGcm::NonceSize;
    let hashofsk = *Impl::hash_bytes(&(input.key));
    let hashstr = hex::encode(&hashofsk);
    println!("{}", hashstr);

    let mut adder = FileAdder::builder()
        .with_chunker(Chunker::Size(input.data.len()))
        .build();
    let blocks_received = adder.push(&(input.data));
    let (cid_data_iter, _size) = blocks_received;
    let mut finalcid: String = "".to_string();
    for (cid, _data) in cid_data_iter {
        println!("cid: {}", cid.to_string());
        finalcid = cid.to_string();
    }
    // TODO: do something with the input
    let output = Outputs {
        hashsk: hashstr,
        cid: finalcid,
        ct: ciphertext,
        nonce: input.nonce.to_vec(),
    };

    // write public output to the journal
    env::commit(&output);
}

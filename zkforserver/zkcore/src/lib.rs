use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub hashsk: String,
    pub cid: String,
    pub ct: Vec<u8>,
    pub nonce: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Inputs {
    pub data: Vec<u8>,
    pub key: [u8; 32],
    pub nonce: [u8; 12],
}
//TODO:change to GnericArray<u8,{unknown}>
//guest commit the hash of sk

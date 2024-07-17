use starknet_rs_crypto::Felt;

pub trait ToHexString {
    fn to_prefixed_hex_str(&self) -> String;
}

pub trait HashProducer {
    type Error;
    fn generate_hash(&self) -> Result<Felt, Self::Error>;
}

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use super::KeyGenerator;
use models::*;

pub struct KeyGeneratorMock;

impl KeyGenerator for KeyGeneratorMock {
    fn generate_key(&self, currency: Currency) -> (PrivateKey, BlockChainAddress) {
        let key: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
        let address: String = thread_rng().sample_iter(&Alphanumeric).take(15).collect();
        (PrivateKey::new(key), BlockChainAddress::new(address))
    }
}

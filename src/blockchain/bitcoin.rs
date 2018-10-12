use btcchain::{OutPoint, Transaction, TransactionInput, TransactionOutput};
use btckey::generator::{Generator, Random};
use btckey::{Address, DisplayLayout, Error as BtcKeyError, Network, Type as AddressType};
use btcprimitives::hash::{H160, H256};
use btcscript::Builder as ScriptBuilder;
use config::BtcNetwork;

use super::error::*;
use super::utils::bytes_to_hex;
use super::BlockchainService;
use models::*;
use prelude::*;

pub struct BitcoinService {
    btc_network: BtcNetwork,
}

impl BlockchainService for BitcoinService {
    fn sign(&self, key: PrivateKey, tx: UnsignedTransaction) -> Result<RawTransaction, Error> {
        let utxos = self.needed_utxos(&tx.utxos, tx.value)?;

        let from_address = tx.from.clone().into_inner();
        let address_from: Address = from_address.parse().map_err(|e: BtcKeyError| {
            let e = format_err!("{}", e);
            ectx!(try err e, ErrorKind::MalformedAddress)
        })?;
        if address_from.kind != AddressType::P2PKH {
            return Err(ectx!(err ErrorContext::UnsupportedAddress, ErrorKind::MalformedAddress => tx));
        }
        let address_from_hash = address_from.hash;
        let script_sig = ScriptBuilder::build_p2pkh(&address_from_hash);

        let mut inputs: Result<Vec<TransactionInput>, Error> = utxos
            .iter()
            .map(|utxo| -> Result<TransactionInput, Error> {
                let Utxo { tx_hash, value, index } = utxo;
                let tx_hash = tx_hash
                    .parse()
                    .map_err(|_| ectx!(try err ErrorKind::MalformedHexString, ErrorKind::MalformedHexString))?;
                let outpoint = OutPoint {
                    hash: tx_hash,
                    index: *index as u32,
                };
                Ok(TransactionInput {
                    previous_output: outpoint,
                    script_sig: script_sig.to_bytes(),
                    sequence: u32::max_value(),
                    script_witness: vec![],
                })
            }).collect();
        let mut inputs = inputs?;
        let to_address = tx.to.clone().into_inner();
        let address_to: Address = to_address.parse().map_err(|e: BtcKeyError| {
            let e = format_err!("{}", e);
            ectx!(try err e, ErrorKind::MalformedAddress)
        })?;
        if address_to.kind != AddressType::P2PKH {
            return Err(ectx!(err ErrorContext::UnsupportedAddress, ErrorKind::MalformedAddress => tx));
        }
        let address_to_hash = address_to.hash;

        let output_script = ScriptBuilder::build_p2pkh(&address_to_hash);
        let output = TransactionOutput {
            value: tx.value.to_inner() as u64,
            script_pubkey: output_script.to_bytes(),
        };
        let mut outputs = vec![output.clone()];
        let sum_inputs: u64 = utxos.iter().map(|u| u.value.to_inner() as u64).sum();
        if sum_inputs < output.value {
            return Err(ectx!(err ErrorKind::NotEnoughUtxo, ErrorKind::NotEnoughUtxo => tx));
        };
        let rest = output.value - sum_inputs;
        if rest > 0 {
            let script = ScriptBuilder::build_p2pkh(&address_from_hash);
            let output = TransactionOutput {
                value: rest as u64,
                script_pubkey: script.to_bytes(),
            };
            outputs.push(output);
        }
        unimplemented!()
    }

    fn generate_key(&self, currency: Currency) -> Result<(PrivateKey, BlockchainAddress), Error> {
        assert_eq!(currency, Currency::Btc, "unexpected currency: {:?}", currency);
        let network = match self.btc_network {
            BtcNetwork::Test => Network::Testnet,
            BtcNetwork::Main => Network::Mainnet,
        };
        let random = Random::new(network);
        let keypair = random.generate().map_err(|e| {
            let e = format_err!("{}", e);
            ectx!(try err e, ErrorSource::Random, ErrorKind::Internal)
        })?;
        let address = BlockchainAddress::new(format!("{}", keypair.address()));
        let pk_bytes = bytes_to_hex(&keypair.private().layout());
        let private_key = PrivateKey::new(pk_bytes);
        Ok((private_key, address))
    }
}

impl BitcoinService {
    pub fn new(btc_network: BtcNetwork) -> Self {
        BitcoinService { btc_network }
    }

    fn needed_utxos(&self, utxos: &[Utxo], value: Amount) -> Result<Vec<Utxo>, Error> {
        let mut utxos = utxos.to_vec();
        utxos.sort_by_key(|x| x.value);
        let mut res = Vec::new();
        let mut sum = Amount::new(0);
        for utxo in utxos.iter().rev() {
            res.push(utxo.clone());
            sum = sum
                .checked_add(utxo.value)
                .ok_or(ectx!(try err ErrorKind::Overflow, ErrorKind::Overflow => utxos, value))?;
            if sum >= value {
                return Ok(res);
            }
        }
        Err(ectx!(err ErrorKind::NotEnoughUtxo, ErrorKind::NotEnoughUtxo => utxos, value))
    }
}

// #[derive(Debug, PartialEq, Default, Clone)]
// pub struct Transaction {
// 	pub version: i32, // 1
// 	pub inputs: Vec<TransactionInput>,
// 	pub outputs: Vec<TransactionOutput>,
// 	pub lock_time: u32, // 0
// }

// #[derive(Debug, PartialEq, Clone, Serializable, Deserializable)]
// pub struct TransactionOutput {
// 	pub value: u64,
// 	pub script_pubkey: Bytes,
// }

// #[derive(Debug, PartialEq, Eq, Clone, Default, Serializable, Deserializable)]
// pub struct OutPoint {
// 	pub hash: H256,
// 	pub index: u32,
// }

// #[derive(Debug, PartialEq, Default, Clone)]
// pub struct TransactionInput {
// 	pub previous_output: OutPoint,
// 	pub script_sig: Bytes,
// 	pub sequence: u32,
// 	pub script_witness: Vec<Bytes>,
// }

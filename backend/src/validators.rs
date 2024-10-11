use bitcoin::Address as BtcAddress;
use ethereum_types::Address as EthAddress;
use solana_sdk::pubkey::Pubkey;
use cardano_serialization_lib::address::Address as CardanoAddress;
use bech32::{self, decode, Variant};
use sha3::{Digest, Keccak256};
use thiserror::Error;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum BlockchainType {
    Bitcoin,
    Ethereum,
    EthereumLayer2(String), // For Polygon, Optimism, Base, etc.
    Solana,
    Cardano,
    Sui,
    Aptos,
    Sei,
    Polkadot,
    Tezos,
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid address format")]
    InvalidFormat,
    #[error("Unknown blockchain")]
    UnknownBlockchain,
}

pub struct AddressValidator;

impl AddressValidator {
    pub fn validate(address: &str, blockchain: Option<&str>) -> Result<BlockchainType, ValidationError> {
        if let Some(blockchain) = blockchain {
            match blockchain.to_lowercase().as_str() {
                "bitcoin" => Self::validate_bitcoin(address),
                "ethereum" => Self::validate_ethereum(address),
                "polygon" | "optimism" | "base" => Self::validate_ethereum_layer2(address, blockchain),
                "solana" => Self::validate_solana(address),
                "cardano" => Self::validate_cardano(address),
                "sui" => Self::validate_sui(address),
                "aptos" => Self::validate_aptos(address),
                "sei" => Self::validate_sei(address),
                "polkadot" => Self::validate_polkadot(address),
                "tezos" => Self::validate_tezos(address),
                _ => Err(ValidationError::UnknownBlockchain),
            }
        } else {
            Self::guess_and_validate(address)
        }
    }

    fn guess_and_validate(address: &str) -> Result<BlockchainType, ValidationError> {
        if Self::validate_bitcoin(address).is_ok() {
            return Ok(BlockchainType::Bitcoin);
        }
        if let Ok(blockchain_type) = Self::validate_ethereum(address) {
            return Ok(blockchain_type);
        }
        if Self::validate_solana(address).is_ok() {
            return Ok(BlockchainType::Solana);
        }
        if Self::validate_aptos(address).is_ok() {
            return Ok(BlockchainType::Aptos);
        }
        if Self::validate_cardano(address).is_ok() {
            return Ok(BlockchainType::Cardano);
        }
        if Self::validate_sui(address).is_ok() {
            return Ok(BlockchainType::Sui);
        }
        if Self::validate_sei(address).is_ok() {
            return Ok(BlockchainType::Sei);
        }
        if Self::validate_polkadot(address).is_ok() {
            return Ok(BlockchainType::Polkadot);
        }
        if Self::validate_tezos(address).is_ok() {
            return Ok(BlockchainType::Tezos);
        }
        Err(ValidationError::InvalidFormat)
    }

    fn validate_bitcoin(address: &str) -> Result<BlockchainType, ValidationError> {
        BtcAddress::from_str(address)
            .map(|_| BlockchainType::Bitcoin)
            .map_err(|_| ValidationError::InvalidFormat)
    }

    fn validate_ethereum(address: &str) -> Result<BlockchainType, ValidationError> {
        EthAddress::from_str(address)
            .map(|_| BlockchainType::Ethereum)
            .map_err(|_| ValidationError::InvalidFormat)
    }

    fn validate_ethereum_layer2(address: &str, blockchain: &str) -> Result<BlockchainType, ValidationError> {
        let checksum = Keccak256::digest(address.as_bytes());
        let checksum_hex = hex::encode(checksum);
        if checksum_hex.starts_with("0x") {
            Ok(BlockchainType::EthereumLayer2(blockchain.to_string()))
        } else {
            Err(ValidationError::InvalidFormat)
        }
    }

    fn validate_solana(address: &str) -> Result<BlockchainType, ValidationError> {
        Pubkey::from_str(address)
            .map(|_| BlockchainType::Solana)
            .map_err(|_| ValidationError::InvalidFormat)
    }

    fn validate_cardano(address: &str) -> Result<BlockchainType, ValidationError> {
        CardanoAddress::from_bech32(address)
            .map(|_| BlockchainType::Cardano)
            .map_err(|_| ValidationError::InvalidFormat)
    }

    fn validate_sui(address: &str) -> Result<BlockchainType, ValidationError> {
        let re = Regex::new(r"^0x[a -fA-F0-9]{64}$").unwrap();
        if re.is_match(address) {
            Ok(BlockchainType::Sui)
        } else {
            Err(ValidationError::InvalidFormat)
        }
    }

    fn validate_aptos(address: &str) -> Result<BlockchainType, ValidationError> {
        let re = Regex::new(r"^0x[a-fA-F0-9]{64}$").unwrap();
        if re.is_match(address) {
            Ok(BlockchainType::Aptos)
        } else {
            Err(ValidationError::InvalidFormat)
        }
    }

    fn validate_sei(address: &str) -> Result<BlockchainType, ValidationError> {
        match decode(address) {
            Ok((hrp, _, Variant::Bech32m)) if hrp == "sei" => Ok(BlockchainType::Sei),
            _ => Err(ValidationError::InvalidFormat),
        }
    }

    fn validate_polkadot(address: &str) -> Result<BlockchainType, ValidationError> {
        match decode(address) {
            Ok((hrp, _, Variant::Bech32)) if hrp.starts_with("1") => Ok(BlockchainType::Polkadot),
            _ => Err(ValidationError::InvalidFormat),
        }
    }

    fn validate_tezos(address: &str) -> Result<BlockchainType, ValidationError> {
        match decode(address) {
            Ok((hrp, _, Variant::Bech32)) if hrp == "tz1" || hrp == "tz2" || hrp == "tz3" => Ok(BlockchainType::Tezos),
            _ => Err(ValidationError::InvalidFormat),
        }
    }
}
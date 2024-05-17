use serde::{Deserialize, Serialize};
use solana_sdk::{account::Account as SolanaAccount, clock::Slot, pubkey::Pubkey};

use crate::compression::CompressionType;

use super::slot_identifier::SlotIdentifier;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Account {
    pub slot_identifier: SlotIdentifier,
    pub pubkey: Pubkey,
    pub owner: Pubkey,
    pub write_version: u64,
    pub data: Vec<u8>,
    pub compression_type: CompressionType,
    pub data_length: u64,
}

impl Account {
    pub fn get_account_for_test(slot: Slot, data_size: usize) -> Self {
        Account {
            slot_identifier: SlotIdentifier { slot },
            pubkey: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            write_version: 0,
            data: vec![178; data_size],
            compression_type: CompressionType::None,
            data_length: data_size as u64,
        }
    }

    pub fn new(
        pubkey: Pubkey,
        solana_account: SolanaAccount,
        compression_type: CompressionType,
        slot_identifier: SlotIdentifier,
        write_version: u64,
    ) -> Self {
        let binary = bincode::serialize(&solana_account).expect("account should be serializable");
        let data_length = solana_account.data.len() as u64;

        let data = match compression_type {
            CompressionType::None => binary,
            CompressionType::Lz4Fast(speed) => lz4::block::compress(
                &binary,
                Some(lz4::block::CompressionMode::FAST(speed as i32)),
                true,
            )
            .expect("Compression should work"),
            CompressionType::Lz4(compression) => lz4::block::compress(
                &binary,
                Some(lz4::block::CompressionMode::HIGHCOMPRESSION(
                    compression as i32,
                )),
                true,
            )
            .expect("compression should work"),
        };
        Account {
            slot_identifier,
            pubkey,
            owner: solana_account.owner,
            write_version,
            data,
            compression_type,
            data_length,
        }
    }

    pub fn solana_account(&self) -> SolanaAccount {
        match self.compression_type {
            CompressionType::None => bincode::deserialize(&self.data).expect("Should deserialize"),
            CompressionType::Lz4(_) | CompressionType::Lz4Fast(_) => {
                let uncompressed =
                    lz4::block::decompress(&self.data, None).expect("should uncompress");
                bincode::deserialize(&uncompressed).expect("Should deserialize")
            }
        }
    }
}

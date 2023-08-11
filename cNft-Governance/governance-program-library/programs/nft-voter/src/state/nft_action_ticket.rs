use anchor_lang::prelude::*;
use crate::tools::anchor::DISCRIMINATOR_SIZE;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq, Default)]
pub struct NftActionTicket {
    pub nft_mint: Pubkey,
    pub weight: u64,
    pub expiry: Option<u64>,
}

impl NftActionTicket {
    /// sha256("account:NftActionTicket")
    /// python:
    /// from hashlib import sha256
    /// list(sha256("account:NftActionTicket".encode()).digest())[:8]
    pub const ACCOUNT_DISCRIMINATOR: [u8; 8] = [170, 179, 4, 130, 24, 148, 185, 97];
    pub fn get_space() -> usize {
        DISCRIMINATOR_SIZE + 32 + 8 + 1 + 8
    }
}

/// Registrar which stores NFT voting configuration for the given Realm
#[account]
#[derive(Debug, PartialEq)]
pub struct NftTicketTable {
    pub registrar: Pubkey,
    pub governing_token_owner: Pubkey,

    /// MPL Collection used for voting
    pub nft_tickets: Vec<NftActionTicket>,

    /// Reserved for future upgrades
    pub reserved: [u8; 128],
}

impl NftTicketTable {
    pub fn get_space(max_nfts: u8) -> usize {
        DISCRIMINATOR_SIZE + 32 * 2 + 4 + (max_nfts as usize) * NftActionTicket::get_space() + 128
    }
}

/// ticket_type = format!("nft-{}-ticket", &voter_weight_action).to_string();
pub fn get_nft_tickets_table_seeds<'a>(
    ticket_type: &'a str,
    registrar: &'a Pubkey,
    owner: &'a Pubkey
) -> [&'a [u8]; 3] {
    [&ticket_type.as_bytes(), registrar.as_ref(), owner.as_ref()]
}

pub fn get_nft_tickets_table_address(
    ticket_type: &str,
    registrar: &Pubkey,
    owner: &Pubkey
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &get_nft_tickets_table_seeds(ticket_type, registrar, owner),
        &crate::id()
    )
}

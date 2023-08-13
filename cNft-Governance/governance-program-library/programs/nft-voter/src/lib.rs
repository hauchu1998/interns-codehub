use anchor_lang::prelude::*;

pub mod error;

mod instructions;
use instructions::*;

pub mod state;

pub mod tools;

use crate::state::*;

declare_id!("GnftVc21v2BRchsRa9dGdrVmJPLZiRHe9j2offnFTZFg");

#[program]
pub mod nft_voter {
    use crate::state::VoterWeightAction;

    use super::*;
    pub fn create_registrar(ctx: Context<CreateRegistrar>, max_collections: u8) -> Result<()> {
        log_version();
        instructions::create_registrar(ctx, max_collections)
    }
    pub fn create_voter_weight_record(
        ctx: Context<CreateVoterWeightRecord>,
        governing_token_owner: Pubkey
    ) -> Result<()> {
        log_version();
        instructions::create_voter_weight_record(ctx, governing_token_owner)
    }
    pub fn create_max_voter_weight_record(ctx: Context<CreateMaxVoterWeightRecord>) -> Result<()> {
        log_version();
        instructions::create_max_voter_weight_record(ctx)
    }
    pub fn update_voter_weight_record(
        ctx: Context<UpdateVoterWeightRecord>,
        voter_weight_action: VoterWeightAction,
        governing_token_owner: Pubkey,
        nft_ticket_table_bump: u8
    ) -> Result<()> {
        log_version();
        instructions::update_voter_weight_record(
            ctx,
            voter_weight_action,
            governing_token_owner,
            nft_ticket_table_bump
        )
    }

    pub fn relinquish_nft_vote(ctx: Context<RelinquishNftVote>) -> Result<()> {
        log_version();
        instructions::relinquish_nft_vote(ctx)
    }
    pub fn configure_collection(
        ctx: Context<ConfigureCollection>,
        weight: u64,
        size: u32
    ) -> Result<()> {
        log_version();
        instructions::configure_collection(ctx, weight, size)
    }

    pub fn cast_nft_vote<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CastNftVote<'info>>,
        proposal: Pubkey,
        governing_token_owner: Pubkey,
        nft_ticket_table_bump: u8
    ) -> Result<()> {
        log_version();
        instructions::cast_nft_vote(ctx, proposal, governing_token_owner, nft_ticket_table_bump)
    }

    pub fn create_nft_action_ticket<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CreateNftActionTicket<'info>>,
        voter_weight_action: VoterWeightAction,
        max_nfts: u8,
        nft_ticket_table_bump: u8
    ) -> Result<()> {
        log_version();
        instructions::create_nft_action_ticket(
            ctx,
            voter_weight_action,
            max_nfts,
            nft_ticket_table_bump
        )
    }

    pub fn create_cnft_action_ticket<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CreateCnftActionTicket<'info>>,
        voter_weight_action: VoterWeightAction,
        max_nfts: u8,
        nft_ticket_table_bump: u8,
        params: Vec<CompressedNftAsset>
    ) -> Result<()> {
        log_version();
        instructions::create_cnft_action_ticket(
            ctx,
            voter_weight_action,
            max_nfts,
            nft_ticket_table_bump,
            params
        )
    }
}

fn log_version() {
    // TODO: Check if Anchor allows to log it before instruction is deserialized
    msg!("VERSION:{:?}", env!("CARGO_PKG_VERSION"));
}

use crate::error::NftVoterError;
use crate::state::*;
use crate::tools::accounts::create_nft_tickets_table_account;
use anchor_lang::prelude::*;
use itertools::Itertools;

/// Create NFT action ticket. Everytime a voter want to do some voting with NFT, they need to get a ticket first.
/// This instruction will check the validation of the NFT and create a ticket for the voter.
/// For each action, they get the specific tickets for it. For example, cast vote get nft-castVote-ticket.
///
/// These tickets will be used in the corresponding instructions, ex: cast_nft_vote and update_voter_weight_record.
/// If the action instruction succeed, the ticket will be closed.
/// Otherwise, the ticket will be kept and can be used in the next action.
///
/// This is the instruction for verifying NFT.
#[derive(Accounts)]
#[instruction(voter_weight_action:VoterWeightAction, governing_token_owner:Pubkey, max_nfts: u8)]
pub struct CreateNftActionTicket<'info> {
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm
        @ NftVoterError::InvalidVoterWeightRecordRealm,
        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ NftVoterError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    pub voter_authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_nft_action_ticket<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateNftActionTicket<'info>>,
    voter_weight_action: VoterWeightAction,
    max_nfts: u8,
    nft_ticket_table_bump: u8
) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let governing_token_owner = &ctx.accounts.voter_weight_record.governing_token_owner;
    let system_program = &ctx.accounts.system_program.to_account_info();
    let payer = &ctx.accounts.payer.to_account_info();
    let mut unique_nft_mints: Vec<Pubkey> = vec![];
    let ticket_type = format!("nft-{}-tickets", &voter_weight_action).to_string();

    let (first_account, remaining_accounts) = ctx.remaining_accounts.split_at(1);
    let nft_tickets_table = &first_account[0];
    if nft_tickets_table.data_is_empty() {
        create_nft_tickets_table_account(
            payer,
            &nft_tickets_table,
            &registrar.key().clone(),
            &governing_token_owner.key().clone(),
            max_nfts,
            &ticket_type,
            &[nft_ticket_table_bump],
            &system_program
        )?;

        let serialized_data = NftTicketTable {
            registrar: registrar.key().clone(),
            governing_token_owner: governing_token_owner.key().clone(),
            nft_tickets: vec![],
            reserved: [0u8; 128],
        };

        nft_tickets_table.data.borrow_mut().copy_from_slice(&serialized_data.try_to_vec()?);
    }

    let data_bytes = nft_tickets_table.try_borrow_mut_data()?;
    let mut nft_tickets_table_data = NftTicketTable::try_from_slice(&data_bytes)?;

    for (nft_info, nft_metadata_info) in remaining_accounts.iter().tuples() {
        let (nft_vote_weight, nft_mint) = resolve_nft_vote_weight_and_mint(
            registrar,
            &governing_token_owner,
            nft_info,
            nft_metadata_info,
            &mut unique_nft_mints
        )?;

        let nft_action_ticket = NftActionTicket {
            nft_mint: nft_mint.clone(),
            weight: nft_vote_weight,
            expiry: Some(Clock::get()?.slot + 10),
        };

        let ticket_idx = nft_tickets_table_data.nft_tickets
            .iter()
            .position(|x| x.nft_mint == nft_mint);
        if let Some(ticket_idx) = ticket_idx {
            nft_tickets_table_data.nft_tickets[ticket_idx] = nft_action_ticket;
        } else {
            nft_tickets_table_data.nft_tickets.push(nft_action_ticket);
        }
    }

    Ok(())
}

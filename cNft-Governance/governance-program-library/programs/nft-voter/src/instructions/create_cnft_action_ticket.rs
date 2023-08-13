use crate::error::NftVoterError;
use crate::state::*;
use anchor_lang::prelude::*;
use spl_account_compression::program::SplAccountCompression;
use crate::tools::accounts::create_nft_tickets_table_account;

/// Create NFT action ticket. Everytime a voter want to do some voting with NFT, they need to get a ticket first.
/// This instruction will check the validation of the NFT and create a ticket for the voter.
/// For each action, they get the specific tickets for it. For example, cast vote get nft-castVote-ticket.
///
/// These tickets will be used in the corresponding instructions, ex: cast_nft_vote and update_voter_weight_record.
/// If the action instruction succeed, the ticket will be closed.
/// Otherwise, the ticket will be kept and can be used in the next action.
///
/// This is the instruction for verifying compressed NFT.
#[derive(Accounts)]
#[instruction(voter_weight_action:VoterWeightAction, params: Vec<CompressedNftAsset>)]
pub struct CreateCnftActionTicket<'info> {
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

    pub compression_program: Program<'info, SplAccountCompression>,
    pub system_program: Program<'info, System>,
}

pub fn create_cnft_action_ticket<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateCnftActionTicket<'info>>,
    voter_weight_action: VoterWeightAction,
    max_nfts: u8,
    nft_ticket_table_bump: u8,
    params: Vec<CompressedNftAsset>
) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let governing_token_owner = &ctx.accounts.voter_weight_record.governing_token_owner;
    let compression_program = &ctx.accounts.compression_program.to_account_info();
    let system_program = &ctx.accounts.system_program.to_account_info();
    let payer = &ctx.accounts.payer.to_account_info();
    let mut unique_asset_ids: Vec<Pubkey> = vec![];
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
            system_program
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

    let mut start = 0;
    for i in 0..params.len() {
        let param = &params[i];
        let proof_len = param.proof_len;
        let accounts = &remaining_accounts[start..start + (proof_len as usize) + 1];

        let tree_account = accounts[0].clone();
        let proofs = accounts[1..(proof_len as usize) + 1].to_vec();

        let (cnft_vote_weight, asset_id) = resolve_cnft_vote_weight(
            &registrar,
            &governing_token_owner,
            &tree_account,
            &mut unique_asset_ids,
            &param,
            proofs,
            compression_program
        )?;

        let nft_action_ticket = NftActionTicket {
            nft_mint: asset_id.clone(),
            weight: cnft_vote_weight,
            expiry: Some(Clock::get()?.slot + 10),
        };

        let ticket_idx = nft_tickets_table_data.nft_tickets
            .iter()
            .position(|x| x.nft_mint == asset_id);
        if let Some(ticket_idx) = ticket_idx {
            nft_tickets_table_data.nft_tickets[ticket_idx] = nft_action_ticket;
        } else {
            nft_tickets_table_data.nft_tickets.push(nft_action_ticket);
        }

        start += (proof_len as usize) + 1;
    }

    Ok(())
}

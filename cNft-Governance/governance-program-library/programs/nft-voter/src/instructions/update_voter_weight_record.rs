use crate::error::NftVoterError;
use crate::state::*;
use anchor_lang::prelude::*;

/// Updates VoterWeightRecord to evaluate governance power for non voting use cases: CreateProposal, CreateGovernance etc...
/// This instruction updates VoterWeightRecord which is valid for the current Slot and the given target action only
/// and hance the instruction has to be executed inside the same transaction as the corresponding spl-gov instruction
///
/// Note: UpdateVoterWeight is not cumulative the same way as CastNftVote and hence voter_weight for non voting scenarios
/// can only be used with max 10 NFTs due to Solana transaction size limit
/// It could be supported in future version by introducing bookkeeping accounts to track the NFTs
/// which were already used to calculate the total weight
#[derive(Accounts)]
#[instruction(voter_weight_action:VoterWeightAction, governing_token_owner:Pubkey, nft_ticket_table_bump: u8,)]
pub struct UpdateVoterWeightRecord<'info> {
    /// The NFT voting Registrar
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut, 
        seeds=[b"nft-CastVote-tickets".as_ref(), registrar.key().as_ref(), governing_token_owner.as_ref()],
        bump=nft_ticket_table_bump,
        close=payer)]
    pub nft_tickets_table: Account<'info, NftTicketTable>,

    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm
        @ NftVoterError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ NftVoterError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

pub fn update_voter_weight_record(
    ctx: Context<UpdateVoterWeightRecord>,
    voter_weight_action: VoterWeightAction,
    governing_token_owner: Pubkey,
    nft_ticket_table_bump: u8
) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;
    let governing_token_owner = &voter_weight_record.governing_token_owner;
    let nft_tickets_table = &ctx.accounts.nft_tickets_table;

    match voter_weight_action {
        // voter_weight for CastVote action can't be evaluated using this instruction
        VoterWeightAction::CastVote => {
            return err!(NftVoterError::CastVoteIsNotAllowed);
        }
        | VoterWeightAction::CommentProposal
        | VoterWeightAction::CreateGovernance
        | VoterWeightAction::CreateProposal
        | VoterWeightAction::SignOffProposal => {}
    }

    let mut voter_weight = 0u64;
    let mut unique_nft_action_tickets = vec![];

    require!(
        *nft_tickets_table.to_account_info().owner == crate::id(),
        NftVoterError::InvalidAccountOwner
    );
    // let ticket_type = format!("nft-{}-tickets", &voter_weight_action).to_string();
    // let nft_tickets_table_address = get_nft_tickets_table_address(
    //     &ticket_type,
    //     &registrar.key(),
    //     governing_token_owner
    // ).0;

    require!(
        nft_tickets_table.governing_token_owner == *governing_token_owner,
        NftVoterError::InvalidNftTicketTable
    );

    for nft_ticket in nft_tickets_table.nft_tickets.iter() {
        if unique_nft_action_tickets.contains(&nft_ticket.nft_mint) {
            return Err(NftVoterError::DuplicatedNftDetected.into());
        }

        require!(nft_ticket.expiry.unwrap() >= Clock::get()?.slot, NftVoterError::NftTicketExpired);
        let weight = nft_ticket.weight;

        unique_nft_action_tickets.push(nft_ticket.nft_mint);
        voter_weight = voter_weight.checked_add(weight).unwrap();
    }

    voter_weight_record.voter_weight = voter_weight;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // Set the action to make it specific and prevent being used for voting
    voter_weight_record.weight_action = Some(voter_weight_action);
    voter_weight_record.weight_action_target = None;

    Ok(())
}

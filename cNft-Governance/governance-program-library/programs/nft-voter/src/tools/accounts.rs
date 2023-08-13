use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::{ rent::Rent, sysvar::Sysvar };
use solana_program::system_instruction::create_account;
use solana_program::system_program;
use solana_program::program::invoke_signed;
use solana_program::msg;
use crate::state::*;

/// ticket_type = format!("nft-{}-tickets", &voter_weight_action).to_string();
pub fn get_nft_tickets_table_seeds<'a>(
    ticket_type: &'a str,
    registrar: &'a Pubkey,
    owner: &'a Pubkey,
    bump: &'a [u8]
) -> [&'a [u8]; 4] {
    [&ticket_type.as_bytes(), registrar.as_ref(), owner.as_ref(), bump.as_ref()]
}

pub fn get_nft_tickets_table_address(
    ticket_type: &str,
    registrar: &Pubkey,
    owner: &Pubkey,
    bump: &[u8]
) -> Result<Pubkey, ProgramError> {
    let seeds = get_nft_tickets_table_seeds(ticket_type, registrar, owner, bump);
    let address = Pubkey::create_program_address(&seeds, &crate::id())?;
    Ok(address)
}

pub fn create_nft_tickets_table_account<'a>(
    payer: &AccountInfo<'a>,
    account_info: &AccountInfo<'a>,
    registrar: &Pubkey,
    owner: &Pubkey,
    max_nfts: u8,
    ticket_type: &str,
    bump_seed: &[u8],
    system_program: &AccountInfo<'a>
) -> Result<(), ProgramError> {
    let account_address_seeds = get_nft_tickets_table_seeds(
        ticket_type,
        registrar,
        owner,
        bump_seed
    );
    let account_address = get_nft_tickets_table_address(ticket_type, registrar, owner, bump_seed)?;

    if account_address != *account_info.key {
        msg!(
            "Create account with PDA: {:?} was requested while PDA: {:?} was expected",
            account_info.key,
            account_address
        );
        return Err(ProgramError::InvalidSeeds);
    }

    let rent = Rent::get()?;

    let lamports = rent.minimum_balance(NftTicketTable::get_space(max_nfts));
    let mut signers_seeds = account_address_seeds.to_vec();
    signers_seeds.push(bump_seed);

    let create_account_instruction = create_account(
        payer.key,
        &account_address,
        lamports,
        NftTicketTable::get_space(max_nfts) as u64,
        &crate::id()
    );

    invoke_signed(
        &create_account_instruction,
        &[payer.clone(), account_info.clone(), system_program.clone()],
        &[&signers_seeds]
    )?;
    Ok(())
}

pub fn serialize_nft_tickets_table_account(
    serialized_data: &Vec<u8>,
    account_info: &AccountInfo
) -> Result<(), ProgramError> {
    account_info.data.borrow_mut().copy_from_slice(&serialized_data);
    Ok(())
}

pub fn close_nft_tickets_table_account(
    account_info: &AccountInfo,
    beneficiary_info: &AccountInfo
) -> Result<(), ProgramError> {
    let account_lamports = account_info.lamports();
    **account_info.lamports.borrow_mut() = 0;

    **beneficiary_info.lamports.borrow_mut() = beneficiary_info
        .lamports()
        .checked_add(account_lamports)
        .unwrap();

    account_info.assign(&system_program::id());
    account_info.realloc(0, false)
}

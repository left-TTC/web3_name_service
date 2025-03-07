use anchor_lang::prelude::*;
use crate::create_name_service;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::state::calculate_PDA;

pub fn create_root_domain(ctx: Context<create_name_service>) -> ProgramResult {
    //confirm the name_accounts_key
    let program_id = ctx.program_id;
    let hashed_name = ctx.accounts.base_data.hashed_name.clone();
    let pubkey_parent: Option<Pubkey> = ctx.accounts.name_parent_owner_opt.as_ref().map(|acc| acc.pubkey);
    let (name_accounts_key, seeds) = calculate_PDA(
        program_id, 
        hashed_name,
        &pubkey_parent,
    );

    if name_accounts_key != *ctx.accounts.name_account.key {
        #[cfg(feature = "Debug")]
        msg!("incoming domain name err");
        return Err(ProgramError::InvalidArgument);
    }

    if ctx.accounts.name_account.data.borrow().len() > 0{

    }

    Ok(())
}

pub fn create_common_domain(ctx: Context<create_name_service>) -> ProgramResult {
    Ok(())
}
use crate::{transfer_info, transfer_name_service};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::prelude::*;
use crate::state::NameRecordHeader;
use anchor_lang::solana_program::program_pack::Pack;



pub fn transfer(
    ctx: Context<transfer_name_service>,
    info: transfer_info,
    ) -> ProgramResult {
    let mut name_record_header =
        NameRecordHeader::unpack_from_slice(&ctx.accounts.name_account.data.borrow())?;

    //don't need to verify
    //we forbid transfering class account and root domain
    //these two should be owned by the program
    if *ctx.accounts.class.key != name_record_header.class ||
        *ctx.accounts.root_domain_account.key != name_record_header.root {
            msg!("Invalid root or class");
            return Err(ProgramError::InvalidArgument);
    }

    if !ctx.accounts.submit_account.is_signer ||
        *ctx.accounts.submit_account.key != name_record_header.owner {
            msg!("Invalid submition");
            return Err(ProgramError::InvalidArgument);
    }

    //prevent forgery
    if name_record_header.class == Pubkey::default() ||
        !ctx.accounts.class.is_signer {
            msg!("The given name class account is incorrect or not a signer.");
            return Err(ProgramError::InvalidArgument)
        }

    name_record_header.owner = info.owner;
    name_record_header
        .pack_into_slice(&mut ctx.accounts.name_account.data.borrow_mut()[..NameRecordHeader::LEN]);

    Ok(())
}
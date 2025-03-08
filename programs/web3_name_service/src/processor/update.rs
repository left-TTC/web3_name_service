use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::prelude::*;
use crate::update_name_service;
use crate::state::NameRecordHeader;
use anchor_lang::solana_program::program_pack::Pack;
use crate::state::fun::write_data;


pub fn update(ctx: Context<update_name_service>) -> ProgramResult {
    //deserialize the original data in the account_data 
    let name_record_header = 
        NameRecordHeader::unpack_from_slice(&ctx.accounts.name_account.data.borrow())?;
    //only the owner can update the storaged info
    if name_record_header.owner != *ctx.accounts.name_update_signer.key {
        #[cfg(feature = "Debug")]
        msg!("invalid updater");
        return Err(ProgramError::InvalidArgument);
    }
    //need signature
    if !ctx.accounts.name_update_signer.is_signer {
        #[cfg(feature = "Debug")]
        msg!("invalid signature");
        return Err(ProgramError::InvalidArgument);
    }
    //Confirm the root domain name and obtain the recognition of the root domain name
    if *ctx.accounts.root_domain.key != name_record_header.root {
        #[cfg(feature = "Debug")]
        msg!("invalid root");
        return Err(ProgramError::InvalidArgument);
    }else {
        if ! ctx.accounts.root_domain.is_signer{
            #[cfg(feature = "Debug")]
            msg!("without the admition");
            return Err(ProgramError::InvalidArgument);
        }
    }

    //write new data into the account
    if !write_data(
        &ctx.accounts.name_account,
        &ctx.accounts.update_data
    ) {
        return Err(ProgramError::InvalidArgument);
    }
    

    Ok(())
}
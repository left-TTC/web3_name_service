use crate::transfer_name_service;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::prelude::*;
use crate::state::NameRecordHeader;
use anchor_lang::solana_program::program_pack::Pack;



pub fn transfer(ctx: Context<transfer_name_service>) -> ProgramResult {
    let mut name_record_header =
        NameRecordHeader::unpack_from_slice(&ctx.accounts.name_account.data.borrow())?;
    
    //We do not manage the parent domain
    //I believe that ownership should be fully vested in the purchaser after creation
    if (*ctx.accounts.submit_account.key != name_record_header.owner) ||
        (ctx.accounts.submit_account.is_signer){
        #[cfg(feature = "Debug")]
        msg!("Submitter has no permission");
        return Err(ProgramError::InvalidArgument);
    }

    name_record_header.owner = ctx.accounts.new_owner.pubkey;
    name_record_header
        .pack_into_slice(&mut ctx.accounts.name_account.data.borrow_mut()[..NameRecordHeader::LEN]);

    Ok(())
}
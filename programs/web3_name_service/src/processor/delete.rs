use crate::delete_name_service;
use anchor_lang::accounts::account;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::prelude::*;
use crate::state::NameRecordHeader;
use anchor_lang::solana_program::program_pack::Pack;



pub fn delete(ctx: Context<delete_name_service>) -> ProgramResult {
    let name_record_header =
        NameRecordHeader::unpack_from_slice(&ctx.accounts.name_account.data.borrow())?;
    
    if (*ctx.accounts.submit_account.key != name_record_header.owner) ||
        (ctx.accounts.submit_account.is_signer){
        #[cfg(feature = "Debug")]
        msg!("Submitter has no permission");
        return Err(ProgramError::InvalidArgument);
    }
    //Overwrite data to zero
    //Equivalent to clearing account data
    let zero_vec = vec![0; ctx.accounts.name_account.data_len()];
    let mut account_data = ctx.accounts.name_account.data.borrow_mut();
    account_data.copy_from_slice(&zero_vec);
    
    //transfer the balance to the refund target
    let source_amount: &mut u64 = &mut ctx.accounts.name_account.lamports.borrow_mut();
    let dest_amount: &mut u64 = &mut ctx.accounts.refund_target.lamports.borrow_mut();
    *dest_amount = dest_amount.saturating_add(*source_amount);
    *source_amount = 0;

    Ok(())
}




use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use crate::create_name_service;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::state::fun::get_seeds_and_key;
use crate::state::NameRecordHeader;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::system_instruction;


/*
@Function: create_root_domain
@description: this function used to create a root domain     
*/
pub fn create_root_domain(ctx: Context<create_name_service>) -> ProgramResult {
    //confirm the name_accounts_key
    let program_id = ctx.program_id;
    let hashed_name = ctx.accounts.base_data.hashed_name.clone();
    //the root dimain account
    let pubkey_root: Option<Pubkey> = 
        ctx.accounts.root_domain_opt.as_ref().map(|acc| acc.key());
    //generate the PDA for the name accounts    
    let (name_accounts_key, seeds) = get_seeds_and_key(
        program_id, 
        hashed_name,
        &pubkey_root,
    );
    //check whether the domain name entered is correct
    if name_accounts_key != *ctx.accounts.name_account.key {
        #[cfg(feature = "Debug")]
        msg!("incoming domain name err");
        return Err(ProgramError::InvalidArgument);
    }
    //prevent secondary creation
    //check the account_info from last contract
    if ctx.accounts.name_account.data.borrow().len() > 0 {
        //deserialize the data
        let name_record_header =
            NameRecordHeader::unpack_from_slice(&ctx.accounts.name_account.data.borrow())?;
        //if the stored data is't the default data: return err
        if name_record_header.owner != Pubkey::default() {
            #[cfg(feature = "Debug")]
            msg!("The given name account already exists.");
            return Err(ProgramError::InvalidArgument);
        }
        //Conflict
        return Err(ProgramError::InvalidArgument);
    }
    //No additional types are considered for now

    //The root domain does not require a root domain verify
    //if register contract does't convert the root domain
    //We think that creating a common domain name
    if let Some(root_domain) = &ctx.accounts.root_domain_opt {
        #[cfg(feature = "Debug")]
        msg!("this is a common domain");
        if !root_domain.is_signer {
            #[cfg(feature = "Debug")]
            msg!("don't have the siganature of root domain");
            return Err(ProgramError::InvalidArgument);
        }else {
            let root_domain_record_header = 
                NameRecordHeader::unpack_from_slice(&root_domain.data.borrow())?;
            
            if &root_domain_record_header.owner != root_domain.key {
                msg!("The given root domain account owner is not correct.");
                return Err(ProgramError::InvalidArgument);
            }
        }
    }else {
        #[cfg(feature = "Debug")]
        msg!("this is a root domain");
    }
    //ensure there is a domain owner
    if &ctx.accounts.name_owner.pubkey == &Pubkey::default() {
        #[cfg(feature = "Debug")]
        msg!("The owner cannot be `Pubkey::default()`.");
        return Err(ProgramError::InvalidArgument);
    }

    //valid data length
    if ctx.accounts.name_account.data.borrow().len() == 0 {
        //transfe lammport to name_account and create the account
        invoke(
            &system_instruction::transfer(ctx.accounts.payer.key, &name_accounts_key, ctx.accounts.base_data.lamports),
            &[
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts.name_account.to_account_info().clone(),
                ctx.accounts.system_account.to_account_info().clone(),
            ],
        )?;
        //Apply for a space and pay the fee
        invoke_signed(
            &system_instruction::allocate(
                &*ctx.accounts.name_account.key,
                NameRecordHeader::LEN.saturating_add(ctx.accounts.base_data.space as usize) as u64,
            ),
            &[ctx.accounts.name_account.to_account_info().clone(), ctx.accounts.system_account.to_account_info().clone()],
            &[&seeds.chunks(32).collect::<Vec<&[u8]>>()],
        )?;
        //assign the program ownership
        invoke_signed(
            &system_instruction::assign(ctx.accounts.name_account.key, ctx.program_id),
            &[ctx.accounts.name_account.to_account_info().clone(), ctx.accounts.system_account.to_account_info().clone()],
            //due to name_account is a PDA, so provide a seed
            &[&seeds.chunks(32).collect::<Vec<&[u8]>>()],
        )?;
    }



    Ok(())
}

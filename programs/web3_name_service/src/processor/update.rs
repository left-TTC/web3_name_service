use std::default;

use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::prelude::*;
use crate::{update_data, update_name_service};
use anchor_lang::solana_program::program_pack::Pack;
use crate::state::Utils::write_data;

 
pub fn update(
    ctx: Context<update_name_service>,
    update_data: update_data) -> ProgramResult {
    //fristly, get the storaged data in the account
    // let name_record_header = 
    //     NameRecordHeader::unpack_from_slice(&ctx.accounts.name_account.data.borrow())?;

    // //Determine whether it is a root domain name
    // //Passing in root means it is not root
    // let if_common_account = if let Some(root_domain_account) = &ctx.accounts.root_domain {
    //     //if true, check root key
    //     if *root_domain_account.key != name_record_header.root {
    //         msg!("error root");
    //         return Err(ProgramError::InvalidArgument);
    //     }
    //     let root_record_header = 
    //         NameRecordHeader::unpack_from_slice(&root_domain_account.data.borrow())?;

    //     //if updater is't the record owner
    //     root_record_header.owner == *ctx.accounts.name_update_signer.key
    // }else {
    //     //meams it is the root domain
    //     false
    // };

    // //need updater's signature
    // if !ctx.accounts.name_update_signer.is_signer {
    //     msg!("The given name class or owner is not a signer.");
    //     return Err(ProgramError::InvalidArgument);
    // };


    // if *ctx.accounts.name_update_signer.key != name_record_header.owner
    //         && !if_common_account
    //     {
    //         msg!("The given name owner account is incorrect.");
    //         return Err(ProgramError::InvalidArgument);
    //     };
    

    // //write new data into the account
    // if !write_data(
    //     &ctx.accounts.name_account,
    //     &update_data
    // ) {
    //     return Err(ProgramError::InvalidArgument);
    // }
    
    
    Ok(())
}
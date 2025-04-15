use anchor_lang::prelude::*;
use crate::{BaseData, CreateNameService};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::state::utils::AUTION;
use left_utils::get_hashed_name;


pub fn create(
    ctx: Context<CreateNameService>,
    init_data: BaseData
    ) -> ProgramResult {
        
    let record_root= if let Some(value) = &ctx.accounts.root_domain_opt {
        msg!("root is {}, this is a common domain", value.key());
        if value.owner != AUTION {
            msg!("the given root domain's record owner should be AUTION key");
            return Err(ProgramError::InvalidArgument);
        }
        value.key()
    }else{
        msg!("root is none, this is a root domain");
        if init_data.owner != AUTION
            || init_data.root != Pubkey::default(){
            msg!("creater is't the AUCTION program");
            msg!("owner is {}", init_data.owner);
            return Err(ProgramError::InvalidArgument);
        }
        init_data.root
    };

    msg!("[2] check root domain ok");

    let cal_hash = get_hashed_name(&init_data.name);

    if cal_hash != init_data.hased_name {
        msg!("provided wrong info");
        return Err(ProgramError::InvalidArgument);
    }

    if init_data.owner == Pubkey::default(){
        msg!("owner should not be the default");
        return Err(ProgramError::InvalidArgument);
    }

    //create name account and write data in
    msg!("write name account record data");
    let name_account = &mut ctx.accounts.name_account;

    name_account.owner = init_data.owner;
    name_account.ipfs = init_data.ipfs;
    name_account.root = record_root;

    msg!("[3] create name account over");

    let name_record = &mut ctx.accounts.record_account;
    msg!("now record root is {}", name_record.root);
    if name_record.root == Pubkey::default() {
        //there are three confitions: common init or root init or root add 
        msg!("this is a new common usr or root domain");
        //if the common usr init, write in root PDA
        //if root domian, it will update to Pubkey::default or write in pubkey::default
        //every situation is feasible
        name_record.root = record_root;

        check_and_add(&mut name_record.domains, init_data.name)?;  
    }else{
        if name_record.root != record_root{
            msg!("wrong provided record account");
            return Err(ProgramError::InvalidArgument);
        }

        check_and_add(&mut name_record.domains, init_data.name)?;   
    }
    msg!("[4] create or add record account ok");

    Ok(())
}


fn check_and_add (record_domains: &mut Vec<u8>, add_name: String) -> ProgramResult{
    let length = record_domains.len();
    let add_name_bytes = add_name.as_bytes();
    if (length % 32 + add_name_bytes.len() + 1) > 32 {
        msg!("Need to reallocate space");

    }else {
        record_domains.extend_from_slice(add_name_bytes);
        record_domains.extend_from_slice(".".as_bytes());
    }

    Ok(())
}


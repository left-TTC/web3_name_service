use anchor_lang::prelude::*;
use crate::{create_name_service, update_name_service,
            transfer_name_service, delete_name_service};
use anchor_lang::solana_program::entrypoint::ProgramResult;

pub mod create;

use create::{create_root_domain, create_common_domain};

pub struct Processor{}

impl Processor {
    pub fn create_name_process(ctx: Context<create_name_service>) -> ProgramResult{
        #[cfg(feature = "Debug")]
        msg!("start create a domian name");
    
        //fristly, comfirm the name is right
        if let Some(name_parent) = ctx.accounts.name_parent_owner_opt.as_ref(){
            create_root_domain(ctx)
        }else {
            create_common_domain(ctx)
        }
    
    }


    pub fn update_name_process(ctx: Context<update_name_service>) -> ProgramResult{


        Ok(())
    }


    pub fn transfer_name_process(ctx: Context<transfer_name_service>) -> ProgramResult{


        Ok(())
    }


    pub fn delete_name_process(ctx: Context<delete_name_service>) -> ProgramResult{


        Ok(())
    }
}







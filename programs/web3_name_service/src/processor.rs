use anchor_lang::prelude::*;
use crate::{BaseData, CreateNameService, delete_name_service, transfer_info, transfer_name_service, update_data, UpdateNameService};
use anchor_lang::solana_program::entrypoint::ProgramResult;

pub mod create;
pub mod delete;
pub mod transfer;


use create::create;
use transfer::transfer;
use delete::delete;

pub struct Processor{}

impl Processor {
    pub fn create_process(
        ctx: Context<CreateNameService>,
        data: BaseData) -> ProgramResult{
        #[cfg(feature = "Debug")]
        msg!("start create a domian name");
        create(ctx, data)
    }

    pub fn update_process(
        ctx: Context<UpdateNameService>,
        update_ipfs: [u8; 46]) -> ProgramResult{

        msg!("start update domain data");

        match std::str::from_utf8(&update_ipfs) {
            Ok(ipfs_str) => msg!("Updating IPFS: {}", ipfs_str),
            Err(_) => msg!("Error: IPFS data is not valid UTF-8"),
        };

        let name_account = &mut ctx.accounts.name_account;
        name_account.ipfs = Some(update_ipfs);

        Ok(())
    }

    pub fn transfer_process(
        ctx: Context<transfer_name_service>,
        transfer_info: transfer_info,
        ) -> ProgramResult{
        #[cfg(feature = "Debug")]
        msg!("start transfer domain data");
        transfer(ctx, transfer_info)
    }

    pub fn delete_process(ctx: Context<delete_name_service>) -> ProgramResult{
        #[cfg(feature = "Debug")]
        msg!("start delete domain data");
        delete(ctx)
    }
}







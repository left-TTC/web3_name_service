use anchor_lang::prelude::*;
use crate::{base_data, create_name_service, delete_name_service, transfer_info, transfer_name_service, update_data, update_name_service};
use anchor_lang::solana_program::entrypoint::ProgramResult;

pub mod create;
pub mod delete;
pub mod update;
pub mod transfer;


use create::create;
use update::update;
use transfer::transfer;
use delete::delete;

pub struct Processor{}

impl Processor {
    pub fn create_process(
        ctx: Context<create_name_service>,
        data: base_data) -> ProgramResult{
        #[cfg(feature = "Debug")]
        msg!("start create a domian name");
        create(ctx, data)
    }

    pub fn update_process(
        ctx: Context<update_name_service>,
        data: update_data) -> ProgramResult{
        #[cfg(feature = "Debug")]
        msg!("start update domain data");
        update(ctx, data)
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







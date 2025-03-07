use anchor_lang::{accounts::{signer, unchecked_account}, prelude::*, solana_program::pubkey};
use processor::Processor;

declare_id!("8urcfunkMXmN59aTzW8SLPGVc5r8Fk7LEokbnQCZXvCp");

pub mod processor;
pub mod state;

#[program]
pub mod web3_name_service {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn create_name (ctx: Context<create_name_service>) -> ProgramResult {
        Processor::create_name_process(ctx)
    }

    pub fn update_name (ctx: Context<update_name_service>) -> ProgramResult {
        Processor::update_name_process(ctx)
    }

    pub fn transfer_name (ctx: Context<transfer_name_service>) -> ProgramResult {
        Processor::transfer_name_process(ctx)
    }

    pub fn delete_name (ctx: Context<delete_name_service>) -> ProgramResult {
        Processor::delete_name_process(ctx)
    }
}

#[derive(Accounts)]
//this accounts info used to create root or domain
//Hierarchical domain names are not considered for now
pub struct create_name_service<'info>{
    name_account: UncheckedAccount<'info>,
    payer_key: Signer<'info>,
    name_owner: Account<'info, only_pub>,
    base_data: Account<'info, base_info>,
    //name_class_opt: Account<'info, only_pub>,
    //parent domain: have -- common domain,  no -- create root domain
    name_parent_owner_opt: Option<Account<'info, only_pub>>,
    init_data: Option<Account<'info, data>>
}

#[derive(Accounts)]
//use to update storaged info
pub struct update_name_service<'info> {
    name_account: UncheckedAccount<'info>,
    name_update_signer: Signer<'info>,
    name_parent: Option<Account<'info, only_pub>>,
    update_data: Account<'info, data>,
}

#[derive(Accounts)]
//use to transfer domain
pub struct transfer_name_service<'info> {
    new_owner: Account<'info, only_pub>,
    name_account: UncheckedAccount<'info>,
    name_owner_key: Signer<'info>,
    name_class_opt: Option<Account<'info, only_pub>,>,
}

#[derive(Accounts)]
//refund and logout
pub struct  delete_name_service<'info> {
    name_account: UncheckedAccount<'info>,
    name_owner_key: Signer<'info>,
    refund_target: Account<'info, only_pub>,
}

#[account]
pub struct only_pub{
    pub pubkey: Pubkey,
}

#[account]
pub struct data{
    pub ipfs: Option<Vec<u8>>,
    pub data: Option<Vec<u8>>,
}

#[account]
pub struct base_info{
    pub lamports: u64,
    pub hashed_name: Vec<u8>,
    pub space: u32,
}

// #[account]
// pub struct create_type {
//     pub if_root: bool,
// }



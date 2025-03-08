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
    //the domain account that will be created
    name_account: UncheckedAccount<'info>,
    //the solana program account
    system_account:Program<'info, System>,
    //to pay the of the domain,need sign
    payer: Signer<'info>,
    //pubkey to bind the domain
    name_owner: Account<'info, only_pub>,
    //the base data:such as account's space,lamport,and the hased name
    base_data: Account<'info, base_info>,                        
    //name_class_opt: Account<'info, only_pub>,
    //parent domain: have -- common domain,  no -- create root domain
    root_domain_opt: Option<Signer<'info>>,
    //the data i want add, the ipfs data and other data
    init_data: Option<Account<'info, web3_data>>
}

#[derive(Accounts)]
//use to update storaged info
pub struct update_name_service<'info> {
    //The domain name account to be modified
    name_account: UncheckedAccount<'info>,
    //updater
    name_update_signer: Signer<'info>,
    //the data need to be updated
    update_data: Account<'info, web3_data>,
    //root domain accout
    root_domain: Signer<'info>,
}

#[derive(Accounts)]
//use to transfer domain
pub struct transfer_name_service<'info> {
    //new owner
    new_owner: Account<'info, only_pub>,
    //name account
    name_account: UncheckedAccount<'info>,
    //The account that requested the transfer transaction
    submit_account: Signer<'info>,
}

#[derive(Accounts)]
//refund and logout
pub struct  delete_name_service<'info> {
    name_account: UncheckedAccount<'info>,
    //The account that requested the delete transaction
    submit_account: Signer<'info>,
    refund_target: UncheckedAccount<'info>,
}

#[account]
pub struct only_pub{
    pub pubkey: Pubkey,
}

#[account]
pub struct web3_data{
    pub ipfs: Option<Vec<u8>>,
}

#[account]
pub struct base_info{
    pub lamports: u64,
    pub hashed_name: Vec<u8>,
    pub space: u32,
}


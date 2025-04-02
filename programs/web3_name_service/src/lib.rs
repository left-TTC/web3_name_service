use anchor_lang::{accounts::{signer, unchecked_account}, prelude::*, solana_program::pubkey};
use processor::Processor;
use state::Utils::{get_hashed_name, AUTION, REGISTER_ID};
use anchor_lang::solana_program::program_pack::{Pack, Sealed};


declare_id!("EWVnJDmu8CRLPyuHQqxgR1oFB8WhXBXRENRr1skQZxA9");

pub mod processor;
pub mod state;

#[program]
pub mod web3_name_service {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn create (
        ctx: Context<create_name_service>,
        data: base_data
        ) -> ProgramResult {
        Processor::create_process(ctx, data)
    }

    pub fn update (
        ctx: Context<update_name_service>,
        update_ipfs: [u8; 46]) -> ProgramResult {
        Processor::update_process(ctx, update_ipfs)
    }

    pub fn transfer(
        ctx: Context<transfer_name_service>,
        transfer: transfer_info
        ) -> ProgramResult {
        Processor::transfer_process(ctx, transfer)
    }
   
    pub fn delete(ctx: Context<delete_name_service>) -> ProgramResult {
        Processor::delete_process(ctx)
    }
}

#[derive(Accounts)]
pub struct create_name_service<'info>{
    
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    #[account(mut)]
    pub name_account: UncheckedAccount<'info>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    #[account(mut)]
    pub record_account: UncheckedAccount<'info>,

    //the solana program account
    pub system_program: Program<'info, System>,
    
    //to pay the of the domain,need sign
    
    payer: Signer<'info>,  

    #[account( owner = AUTION )]
    root_domain_opt: Option<Account<'info, NameAccount>>,

    // #[account(
    //     seeds = [b"authority"], 
    //     bump,
    //     seeds::program = &REGISTER_ID
    // )]
    // pda_signer: Signer<'info>,
}

#[account]
pub struct base_data {
    pub lamports: u64,
    pub name: String,
    pub space: u32,
    pub owner: Pubkey,
    pub ipfs: Option<[u8; 46]>,
}


#[account]
pub struct NameAccount{
    pub owner: Pubkey,
    pub root: Pubkey,
    pub ipfs: Option<[u8; 46]>,
}


#[account]
pub struct RecordAccount {
    pub root: Pubkey,
    pub domains: Vec<u8>,
}


#[derive(Accounts)]
//use to update storaged info
pub struct update_name_service<'info> {
    //The domain name account to be modified
    // #[account(mut,
    //     constraint = name_account.key() != name_account.root
    // )]
    #[account(mut)]
    name_account: Account<'info, NameAccount>,
    //updater
    #[account( address = name_account.owner )]
    name_update_signer: Signer<'info>,

    // #[account( address = name_account.root )]
    root_domain: Account<'info, NameAccount>,
}

#[account]
pub struct update_data {
    ipfs: [u8; 46],
}

#[derive(Accounts)]
//use to transfer domain
pub struct transfer_name_service<'info> {
    //We specify that there must be a class
    //and top domain, class domain can't be transfered **
    class: Signer<'info>,
    //name account
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    name_account: UncheckedAccount<'info>,
    //The account that requested the transfer transaction
    submit_account: Signer<'info>,
    //transer don't need the signature of the root
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    root_domain_account: UncheckedAccount<'info>,
}

#[account]
pub struct transfer_info {
    owner: Pubkey,
}

#[derive(Accounts)]
//refund and logout
pub struct  delete_name_service<'info> {
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    name_account: UncheckedAccount<'info>,
    //The account that requested the delete transaction
    submit_account: Signer<'info>,
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
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









#[cfg(test)]
mod test {
    use std::string;

    use crate::state::Utils::{self, create_record_data};

    use super::*;
    use anchor_lang::solana_program::nonce::state::Data;
    use anchor_lang::{prelude::*, Discriminator};
    use anchor_lang::solana_program::hash::hash;
    use anchor_lang::solana_program::pubkey;

    #[test]
    fn test1() {
        let account_data = NameAccount{
            owner: AUTION,
            root: AUTION,
            ipfs: Some([0; 46]),
        };

        let mut data = Vec::new();
        data.extend_from_slice(&NameAccount::DISCRIMINATOR);
        account_data.serialize(&mut data).unwrap(); 

        let check_data = &data[8..];
        let des_data = NameAccount::try_from_slice(check_data).unwrap();

        println!("owner:{}",des_data.owner);
        println!("this test means we should skip the frist eight bytes");
    }

    #[test]
    fn test2() {

    }
}


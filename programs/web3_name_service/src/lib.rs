use anchor_lang::{accounts::{signer, unchecked_account}, prelude::*, solana_program::pubkey};
use processor::Processor;

declare_id!("BWK7ZQWjQ9fweneHfsYmof7znPr5GyedCWs2J8JhHxD3");

pub mod processor;
pub mod state;
pub mod cpi;

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
        data: update_data) -> ProgramResult {
        Processor::update_process(ctx, data)
    }

    pub fn transfer(
        ctx: Context<transfer_name_service>,
        transfer: transfer_info
        ) -> ProgramResult {
        Processor::transfer_process(ctx, transfer)
    }
    //It doesn't seem necessary
    pub fn delete(ctx: Context<delete_name_service>) -> ProgramResult {
        Processor::delete_process(ctx)
    }
}

#[derive(Accounts)]
//this accounts info used to create root or domain
//Hierarchical domain names are not considered for now
pub struct create_name_service<'info>{
    //the domain account that will be created
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    #[account(mut)]
    name_account: UncheckedAccount<'info>,
    //the solana program account
    system_account:Program<'info, System>,
    //to pay the of the domain,need sign
    payer: Signer<'info>,  

    //this is type of the class
    //we only have the common class now
    //but if we want to add other function
    //class will make it easier
    //common, twitter
    domain_class: Signer<'info>,
    //parent domain: have -- common domain,  no -- create root domain
    root_domain_opt: Option<UncheckedAccount<'info>>,
}

#[account]
pub struct base_data {
    pub lamports: u64,
    pub hashed_name: Vec<u8>,
    pub space: u32,
    pub owner: Pubkey,
    pub ipfs: Option<Vec<u8>>,
}


#[derive(Accounts)]
pub struct create_record_service<'info>{
    //the domain account that will be created
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    #[account(mut)]
    record_account: UncheckedAccount<'info>,
    //the solana program account
    system_account:Program<'info, System>,
    //to pay the of the domain,need sign
    payer: Signer<'info>,  

    //parent domain: have -- common domain,  no -- create root domain
    root_domain_opt: Option<UncheckedAccount<'info>>,
}

#[derive(Accounts)]
//use to update storaged info
pub struct update_name_service<'info> {
    //The domain name account to be modified
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    name_account: UncheckedAccount<'info>,
    //updater
    //should be the class account
    name_update_signer: Signer<'info>,
    //root domain accout
    //Y - common account update the info
    //N - top domain
    root_domain: Option<Signer<'info>>,
}

#[account]
pub struct update_data {
    ipfs: Vec<u8>,
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
    use super::*;
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::hash::hash;
    use std::convert::TryInto;

    // 计算指令的 Discriminator
    fn get_discriminator(name: &str) -> [u8; 8] {
        let sighash = hash(format!("global:{}", name).as_bytes());
        sighash.to_bytes()[..8].try_into().unwrap()
    }

    #[test]
    fn test_all_discriminators() {
        let create_domain_discriminator = get_discriminator("create_domain");
        let update_domain_discriminator = get_discriminator("update_domain");
        let transfer_domain_discriminator = get_discriminator("transfer_domain");
        let delete_domain_discriminator = get_discriminator("delete_domain");

        println!("Discriminators:");
        println!("create_domain: {:?}", create_domain_discriminator);
        println!("update_domain: {:?}", update_domain_discriminator);
        println!("transfer_domain: {:?}", transfer_domain_discriminator);
        println!("delete_domain: {:?}", delete_domain_discriminator);

        assert_eq!(create_domain_discriminator.len(), 8);
        assert_eq!(update_domain_discriminator.len(), 8);
        assert_eq!(transfer_domain_discriminator.len(), 8);
        assert_eq!(delete_domain_discriminator.len(), 8);
    }
}


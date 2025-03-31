use anchor_lang::{accounts::{signer, unchecked_account}, prelude::*, solana_program::pubkey};
use processor::Processor;
use state::Utils::{get_hashed_name, AUTION, REGISTER_ID};
use anchor_lang::solana_program::program_pack::{Pack, Sealed};


declare_id!("2zwHkEcbGRfzif4iCtpNgQntPgZDRhAukteiuDeAcjYU");

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
        data: update_data) -> ProgramResult {
        Processor::update_process(ctx, data)
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
    use std::string;

    use crate::state::Utils::{self, create_record_data};

    use super::*;
    use anchor_lang::solana_program::nonce::state::Data;
    use anchor_lang::{prelude::*, Discriminator};
    use anchor_lang::solana_program::hash::hash;
    use anchor_lang::solana_program::pubkey;

    #[test]
    fn storage_domain_record_test() {

        println!("[1] create a record");

        let mut will_record_data = create_record_data(
            String::from("aaa"), AUTION);

        println!("Serialized data length: {}", will_record_data.len()); 
        
        let mut decoded = RecordAccount::try_from_slice(&will_record_data).unwrap();
        println!("deserialized domains: {}", String::from_utf8_lossy(&decoded.domains));

        if !decoded.domains.is_empty() {
            
            let mut recorded_domains = decoded.domains;
            let add = String::from("xyz");
            let will_add_domain = add.as_bytes();

            if let Some(pos) = recorded_domains.iter().rposition(|&c| c == b'.') {
                recorded_domains.truncate(pos + 1);
            }
    
            if recorded_domains.len() % 32 + 1 + will_add_domain.len() > 32 {
                msg!("need add space");
            }else {
                recorded_domains.extend_from_slice(will_add_domain);
                recorded_domains.extend_from_slice(".".as_bytes());
                recorded_domains.extend(vec![0u8; 32 - recorded_domains.len()%32]);
    
                decoded.domains = recorded_domains;
                let mut new_write = Vec::new();
                decoded.serialize(&mut new_write).unwrap();

                let mut decoded = RecordAccount::try_from_slice(&new_write).unwrap();
                println!("deserialized domains: {}", String::from_utf8_lossy(&decoded.domains));

                if !decoded.domains.is_empty() {
            
                    let mut recorded_domains = decoded.domains;
                    let add = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
                    let will_add_domain = add.as_bytes();
        
                    if let Some(pos) = recorded_domains.iter().rposition(|&c| c == b'.') {
                        recorded_domains.truncate(pos + 1);
                    }
            
                    if recorded_domains.len() % 32 + 1 + will_add_domain.len() > 32 {
                        msg!("need add space");
                    }else {
                        recorded_domains.extend_from_slice(will_add_domain);
                        recorded_domains.extend_from_slice(".".as_bytes());
                        recorded_domains.extend(vec![0u8; 32 - recorded_domains.len()%32]);
            
                        decoded.domains = recorded_domains;
                        let mut new_write = Vec::new();
                        decoded.serialize(&mut new_write).unwrap();
        
                        let mut decoded = RecordAccount::try_from_slice(&new_write).unwrap();
                        println!("deserialized domains: {}", String::from_utf8_lossy(&decoded.domains))
                    }
                }
            }
        }
        
        
        // // 6. 反序列化
        // let decoded = RecordAccount::try_from_slice(&data);
        // match decoded {
        //     Ok(decoded) => {
        //         println!("[6] Decoded successfully:");
        //         println!("  - Root: {}", decoded.root);
                
        //         let decoded_str = String::from_utf8(decoded.domains.clone())
        //             .expect("Invalid UTF-8 in domains");
        //         println!("  - Domains: {}", decoded_str);
                
        //         // 7. 完整性验证
        //         assert_eq!(decoded.root, AUTION, "Root mismatch");
        //         assert_eq!(
        //             decoded_str, "aaa.web",
        //             "Domains content mismatch"
        //         );
        //         assert_eq!(
        //             decoded.domains, domains_bytes,
        //             "Domains binary mismatch"
        //         );
        //         println!("[7] All assertions passed!");
        //     }
        //     Err(e) => {
        //         panic!("Deserialization failed: {}", e);
        //     }
        // }
    }
}


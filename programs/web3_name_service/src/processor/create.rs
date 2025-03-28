use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use crate::{create_name_service, base_data};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::state::fun::{get_hashed_name, get_seeds_and_key};
use crate::state::{DomainRecordHeader, NameRecordHeader};
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::system_instruction;


/*
@Function: create_root_domain
@description: this function used to create a root domain     
*/
pub fn create(
    ctx: Context<create_name_service>,
    init_data: base_data
    ) -> ProgramResult {

    //confirm the name_accounts_key
    let mut if_root = false;
    
    let root_domain_key = if let Some(value) = &ctx.accounts.root_domain_opt {
        msg!("root: {}", value.key);
        msg!("payer: {}", ctx.accounts.payer.key);
        if value.key == ctx.accounts.payer.key
            || value.key == &Pubkey::default() {
            if_root = true;
            None
        }else{
            if_root = true;
            Some(value.key)
        }
    }else{
        msg!("root is none, ok");
        None
    };

    let (name_account_key, seeds) = get_seeds_and_key(
        ctx.program_id,
        init_data.hashed_name.clone(),
        root_domain_key,
    );

    if *ctx.accounts.name_account.key != name_account_key {
        #[cfg(feature = "Debug")]
        msg!("incoming domain name err");
        msg!("coming {}", ctx.accounts.name_account.key);
        msg!("need {}", name_account_key);
        return Err(ProgramError::InvalidArgument);
    };
    msg!("name account key ok");


    //prevent secondary creation
    //record account is same,but we only do here is create
    //not update, so the data's lenth should be 0
    if ctx.accounts.name_account.data.borrow().len() > 0 {
        let name_record_header =
            NameRecordHeader::unpack_from_slice(&ctx.accounts.name_account.data.borrow())?;
        //default = can register but need't to create it
        //we can revise it all the time
        if name_record_header.owner != Pubkey::default() {
            #[cfg(feature = "Debug")]
            msg!("The given name account already exists.");
            msg!("now the owner: {}",name_record_header.owner);
            return Err(ProgramError::InvalidArgument);
        }
    }

    msg!("name account data ok");


    //create a mut root: used to construct NameRecordHeader 
    let mut root:Pubkey;

    //The root domain does not require a root domain verify
    //if register contract does't convert the root domain
    //We think that creating a common domain name
    if !if_root{
        if let Some(root_domain) = &ctx.accounts.root_domain_opt{
            #[cfg(feature = "Debug")]
            msg!("this is a common domain");
            let root_domain_record_header = 
                NameRecordHeader::unpack_from_slice(&root_domain.data.borrow())?;
            
            if &root_domain_record_header.owner != root_domain.key {
                msg!("The given root domain account owner is not correct.");
                return Err(ProgramError::InvalidArgument);
            }
            //common accout: use root account;s key
            root = root_domain.key.clone();
        }else{
            msg!("this is a root domain");
            //rppt domain: use self
            root = ctx.accounts.name_account.key.clone();
        }   
    }else {
        #[cfg(feature = "Debug")]
        msg!("this is a root domain");
        //rppt domain: use self
        root = ctx.accounts.name_account.key.clone();
    }
    msg!("point ok");

    //ensure there is a domain owner
    if init_data.owner == Pubkey::default() {
        #[cfg(feature = "Debug")]
        msg!("The owner cannot be `Pubkey::default()`.");
        return Err(ProgramError::InvalidArgument);
    }
    msg!("owner ok");

    //we need to determine wheather create record account
    let mut byte_owner: Vec<u8> = Vec::new();
    let byte = init_data.owner.clone().to_bytes();
    byte_owner.extend_from_slice(&byte);

    let mut if_record = false;
    if byte_owner == init_data.hashed_name.clone(){
        if_record = true;
    }


    //valid data length
    #[cfg(feature = "devnet")]
    if ctx.accounts.name_account.data.borrow().len() == 0 {
        //transfe lammport to name_account and create the account
        invoke(
            &system_instruction::transfer(
                ctx.accounts.payer.key, 
                ctx.accounts.name_account.key, 
                init_data.lamports),
            &[
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts.name_account.to_account_info().clone(),
                ctx.accounts.system_account.to_account_info().clone(),
                ],
        )?;
        if !if_record{
            //create a name account
            //Apply for a space and pay the fee
            invoke_signed(
                &system_instruction::allocate(
                    &*ctx.accounts.name_account.key,
                    NameRecordHeader::LEN.saturating_add(init_data.space as usize) as u64,
                ),
                &[ctx.accounts.name_account.to_account_info().clone(), ctx.accounts.system_account.to_account_info().clone()],
                &[&seeds.chunks(32).collect::<Vec<&[u8]>>()],
            )?;
        }else {
            //create record account
            //Apply for a space and pay the fee
            invoke_signed(
                &system_instruction::allocate(
                    &*ctx.accounts.name_account.key,
                    //Directly hard-coded in units of 32 bytes
                    DomainRecordHeader::LEN.saturating_add(32) as u64,
                ),
                &[ctx.accounts.name_account.to_account_info().clone(), ctx.accounts.system_account.to_account_info().clone()],
                &[&seeds.chunks(32).collect::<Vec<&[u8]>>()],
            )?;
        }
        //assign the program ownership
        invoke_signed(
            &system_instruction::assign(ctx.accounts.name_account.key, ctx.program_id),
            &[ctx.accounts.name_account.to_account_info().clone(), ctx.accounts.system_account.to_account_info().clone()],
            //due to name_account is a PDA, so provide a seed
            &[&seeds.chunks(32).collect::<Vec<&[u8]>>()],
        )?;
    }


    if !if_record{
        //Data writing does not require explicit calls to code that interacts with the chain
        //In Solana, account data modification is implicit, and the program only needs to modify the data field directly.
        //trasfer ipfs data to u8
        let ipfs_data = if let Some(ref init_data_account) = init_data.ipfs {
            msg!("Check IPFS: {}", String::from_utf8_lossy(init_data_account));
    
            if init_data_account.len() != 46 {
                #[cfg(feature = "Debug")]
                msg!("IPFS CID must be 46 bytes long");
                return Err(ProgramError::InvalidArgument);
            }
    
            let mut ipfs_array = [0u8; 46];
            ipfs_array.copy_from_slice(init_data_account);
            Some(ipfs_array)
        } else {
            None
        };

        //Construct a NameRecordHeader structure
        let will_record_data = NameRecordHeader {
            owner: init_data.owner,
            root: root,
            ipfs: ipfs_data.clone(),
        };
        //Implicit Write in
        //Solana's account data is directly mapped into memory

        will_record_data.pack_into_slice(&mut ctx.accounts.name_account.data.borrow_mut());
    }else {
        let domain_data = if let Some(ref init_data_account) = init_data.ipfs {
            init_data_account
        }else{
            msg!("record account must get a domain data");
            return Err(ProgramError::InvalidArgument);
        };

        let will_record_data = DomainRecordHeader{
            root: root,
            domains: domain_data.clone(),
        };

        will_record_data.pack_into_slice(&mut ctx.accounts.name_account.data.borrow_mut());
    }

    msg!("no problem here");
    Ok(())
}

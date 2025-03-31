
use anchor_lang::{prelude::*, Discriminator};
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use crate::{base_data, create_name_service, NameAccount, RecordAccount};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::state::Utils::{self,create_record_data, get_PDA_key, get_hashed_name, if_over_size, AUTION};
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::{lamports, system_instruction};


pub fn create(
    ctx: Context<create_name_service>,
    init_data: base_data
    ) -> ProgramResult {
    //domain type flag
    let mut is_common_domain = true;
    
    let root_domain_key = if let Some(value) = &ctx.accounts.root_domain_opt {
        msg!("root: {}", value.key());
        msg!("payer: {}", ctx.accounts.payer.key);
        if value.key() == *ctx.accounts.payer.key
            || value.key() == Pubkey::default() {
            is_common_domain = false;
            None
        }else{
            Some(value.key())
        }
    }else{
        msg!("root is none, ok");
        is_common_domain = false;
        None
    };

    //manually check name account key
    let (name_account_key, name_seeds) = get_PDA_key(
        ctx.program_id,
        get_hashed_name(&init_data.name),
        root_domain_key.as_ref(),
    );

    if *ctx.accounts.name_account.key != name_account_key {
        #[cfg(feature = "Debug")]
        msg!("incoming domain name err");
        msg!("coming {}", ctx.accounts.name_account.key);
        msg!("need {}", name_account_key);
        return Err(ProgramError::InvalidArgument);
    };
    msg!("name account key ok");

    //manually check record account key
    let (record_account_key, record_seeds) = get_PDA_key(
        ctx.program_id,
        get_hashed_name(&init_data.owner.to_string()),
        root_domain_key.as_ref(),
    );

    if *ctx.accounts.record_account.key != record_account_key {
        #[cfg(feature = "Debug")]
        msg!("incoming domain name err");
        msg!("coming {}", ctx.accounts.record_account.key);
        msg!("need {}", record_account_key);
        return Err(ProgramError::InvalidArgument);
    };
    msg!("record account key ok");

    //try to get onchain data
    if check_if_init(&ctx.accounts.name_account) {
        msg!("name account has inited");
        return Err(ProgramError::InvalidArgument);
    }
    msg!("name account ok");

    //create a mut root: used to construct NameRecordHeader 
    let mut root;
    if is_common_domain{
        msg!("process to create a common domain");
        if let Some(root_domain_account) = &ctx.accounts.root_domain_opt {
            msg!("reading the data in root domain");
            if root_domain_account.owner != AUTION {
                msg!("the given root domain's record owner should be AUTION key");
                return Err(ProgramError::InvalidArgument);
            }

            root = record_account_key.key().clone();
        }else{
            msg!("no root domain");
            return Err(ProgramError::InvalidArgument);
        }
    }else {
        msg!("process to create root domain");
        root = *ctx.accounts.name_account.key;

        if init_data.owner != AUTION {
            msg!("creater is't the AUCTION program");
            return Err(ProgramError::InvalidArgument);
        }
    }
    msg!("root value ok");

    if init_data.owner == Pubkey::default() {
        #[cfg(feature = "Debug")]
        msg!("The owner cannot be `Pubkey::default()`.");
        return Err(ProgramError::InvalidArgument);
    }
    msg!("owner ok");

    //create name account and write data in
    {
        invoke_to_create(
            &ctx, name_seeds, false, init_data.lamports)?;

        msg!("write name account record data");

        let write_name_account_data = NameAccount{
            owner: init_data.owner,
            root: root,
            ipfs: init_data.ipfs,
        };

        let mut data = Vec::new();
        write_name_account_data.serialize(&mut data)?;  

        msg!("serialize success");
        msg!("name data length: {}", data.len());

        ctx.accounts.name_account
            .try_borrow_mut_data()?
            .copy_from_slice(&data);

        msg!("create name account over");
    }

    if check_if_init(&ctx.accounts.record_account) {
        let account_data = &mut ctx.accounts.record_account.try_borrow_mut_data()?;

        let mut recorded_data = RecordAccount::try_from_slice(&account_data)?;

        let mut recorded_domains = recorded_data.domains;
        let will_add_domain = init_data.name.as_bytes();

        if let Some(pos) = recorded_domains.iter().rposition(|&c| c == b'.') {
            recorded_domains.truncate(pos + 1);
        }else {
            return Err(ProgramError::InvalidArgument);
        }

        if recorded_domains.len() % 32 + 1 + will_add_domain.len() > 32 {
            msg!("need add space");
            return Err(ProgramError::InvalidArgument);
        }else {
            recorded_domains.extend_from_slice(will_add_domain);
            recorded_domains.extend_from_slice(".".as_bytes());
            recorded_domains.extend(vec![0u8; 32 - recorded_domains.len()%32]);

            recorded_data.domains = recorded_domains;
            let mut new_write = Vec::new();
            recorded_data.serialize(&mut new_write)?;

            account_data[..new_write.len()].copy_from_slice(&new_write);
        }
    }else {
        msg!("create domain fristly");
        //transfer
        invoke_to_create(
            &ctx, record_seeds, true, init_data.lamports)?;

        if init_data.name.as_bytes().len() <= 32{
            let init_account_data = create_record_data(init_data.name, root);
            msg!("data's length: {}", init_account_data.len());
            ctx.accounts.record_account
                .try_borrow_mut_data()?
                .copy_from_slice(&init_account_data);
        }else {
            return Err(ProgramError::InvalidArgument);   
        }
    }

    msg!("no problem here");
    Ok(())
}



fn invoke_to_create(
    ctx: &Context<create_name_service>,
    seed: Vec<u8>,
    if_create_record: bool,
    calculated_lamports: u64
) -> ProgramResult {
    let (create_account, space) = if if_create_record {
        (&ctx.accounts.record_account, Utils::RECORD_LEN)
    } else{
        (&ctx.accounts.name_account, Utils::NAME_LEN)
    };

    //transfer
    msg!("transfer");
    invoke(
        &system_instruction::transfer(
            ctx.accounts.payer.key, 
            create_account.key, 
            calculated_lamports),
        &[
            ctx.accounts.payer.to_account_info().clone(),
            create_account.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            ],
    )?;
    //Apply for a space and pay the fee
    msg!("space");
    invoke_signed(
        &system_instruction::allocate(
            create_account.key,
            space as u64,
        ),
        &[create_account.to_account_info().clone(), ctx.accounts.system_program.to_account_info().clone()],
        &[&seed.chunks(32).collect::<Vec<&[u8]>>()],
    )?;
    //assign the program ownership
    invoke_signed(
        &system_instruction::assign(create_account.key, ctx.program_id),
        &[create_account.to_account_info().clone(), ctx.accounts.system_program.to_account_info().clone()],
        &[&seed.chunks(32).collect::<Vec<&[u8]>>()],
    )?;

    Ok(())
}

fn check_if_init (account: &UncheckedAccount) -> bool {
    let account_data = account.try_borrow_data().unwrap();

    if account_data.len() > 0 {
        true
    }else {
        false
    }
}


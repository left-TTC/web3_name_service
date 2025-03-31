use borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::solana_program::program_pack::{Pack, Sealed};
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::msg;


pub mod Utils{
    use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
    use crate::{update_data, web3_data, RecordAccount};
    use anchor_lang::solana_program::hash::hashv;
    use anchor_lang::solana_program::ed25519_program;


    pub const NAME_LEN: usize = 32 + 32 + 1 + 46;
    pub const RECORD_LEN: usize = 32 + 4 + 32;
    pub const AUTION: Pubkey = pubkey!("DWNSuxCniY8m11DazRoN3VqvDZK8Sps2wgoQHWx3t4Sx");
    pub const HASH_PREFIX: &str = "WEB3 Name Service";
    pub const REGISTER_ID: Pubkey = pubkey!("7MReDm6FiS3n4A1sxTxdHu8p92TQutQSws715azLqtYj");

    pub fn get_hashed_name(name: &str) -> Vec<u8> {
        hashv(&[(HASH_PREFIX.to_owned() + name).as_bytes()])
            .as_ref()
            .to_vec()
    }

    pub fn get_PDA_key(
        program_id: &Pubkey,
        hashed_name: Vec<u8>,
        root_opt: Option<&Pubkey>,
    ) -> (Pubkey, Vec<u8>) {        
        let mut seeds_vec: Vec<u8> = hashed_name;

        //root domain(when create a root domian,use default)
        let root_domian = root_opt.cloned().unwrap_or_default();
        //add root to the seed
        for b in root_domian.to_bytes() {
            seeds_vec.push(b);
        }
    
        let (name_account_key, bump) =
            Pubkey::find_program_address(&seeds_vec.chunks(32).collect::<Vec<&[u8]>>(), program_id);
        seeds_vec.push(bump);
    
        (name_account_key, seeds_vec)
    }

    pub fn write_data(write_account: &AccountInfo, input: &update_data) -> bool{
        let mut account_data = write_account.data.borrow_mut();
        //Serialize
        if let Ok(serialized_data) = input.try_to_vec()  {
            if serialized_data.len() <= account_data.len() {
                account_data[..serialized_data.len()].copy_from_slice(&serialized_data);
            } else {
                #[cfg(feature = "Debug")]
                msg!("Serialized data exceeds account storage size.");
                return false;
            }
        } else {
            #[cfg(feature = "Debug")]
            msg!("Failed to serialize data.");
            return false;
        }
        return true;
    }


    pub fn create_record_data(name: String, root: Pubkey) -> Vec<u8> {
        let mut name_vec = Vec::new();
        name_vec.extend_from_slice(name.as_bytes());
        name_vec.extend_from_slice(".".as_bytes());
        
        name_vec.extend(vec![0u8; 32 - name_vec.len()]);

        let record = RecordAccount {
            root: root,
            domains: name_vec,
        };

        let mut data = Vec::new();
        record.serialize(&mut data).unwrap();

        data
    }


    pub fn if_over_size(now_len: usize, will_add_len: usize) -> bool {
        let size = now_len % 32;
        if (size + will_add_len + 1) > 32 {
            true
        }else {
            false
        }
    }



        

}




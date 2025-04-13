use anchor_lang::solana_program::msg;


pub mod utils{
    use anchor_lang::Discriminator;
    use anchor_lang::prelude::*;
    use crate::RecordAccount;

    pub const AUTION: Pubkey = pubkey!("4FvqywYKRiv1uhah5Y1s39PZqoX7qLVjMiyUhguSwbvv"); 
    pub const REGISTER_ID: Pubkey = pubkey!("7MReDm6FiS3n4A1sxTxdHu8p92TQutQSws715azLqtYj");


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
        data.extend_from_slice(&RecordAccount::DISCRIMINATOR);
        record.serialize(&mut data).unwrap();

        data
    }



}




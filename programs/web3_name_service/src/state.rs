use borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::solana_program::program_pack::{Pack, Sealed};
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::msg;




#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct NameRecordHeader{
    //owner of this name
    pub owner: Pubkey,
    //define the data type
    //ipfs cid?   
    pub ipfs: Option<[u8; 46]>,
    //root domain pubkey
    pub root: Pubkey,
}
//Prevent external code from implementing certain traits for NameRecordHeader
impl Sealed for NameRecordHeader {}

//Serialize and deserialize structures into byte arrays
impl Pack for NameRecordHeader {
    //pubkey:32 ipfs:
    const LEN: usize = 79;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap()
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mut p = src;
        NameRecordHeader::deserialize(&mut p).map_err(|_| {
            msg!("Failed to deserialize name record");
            ProgramError::InvalidAccountData
        })
    }
}




pub mod fun{
    use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
    use crate::web3_data;

    //usage: calculate the PDA
    //program_id: the id of current program
    //hashed_name: off-chain, the hased value of domain
    //if use root, means it's common domain,
    pub fn get_seeds_and_key(
        program_id: &Pubkey,
        hashed_name: Vec<u8>,
        root_opt: &Option<Pubkey>,
    ) -> (Pubkey, Vec<u8>) {
        //hashed name as the init seeds
        let mut seeds_vec: Vec<u8> = hashed_name;
        //root domain(when create a root domian,use default)
        let root_domian = root_opt.clone().unwrap_or_default();
        //add root to the sed
        for b in root_domian.to_bytes() {
            seeds_vec.push(b);
        }
    
        let (name_account_key, bump) =
            Pubkey::find_program_address(&seeds_vec.chunks(32).collect::<Vec<&[u8]>>(), program_id);
        seeds_vec.push(bump);
    
        (name_account_key, seeds_vec)
    }

    pub fn write_data(write_account: &AccountInfo, input: &web3_data) -> bool{
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

        

}




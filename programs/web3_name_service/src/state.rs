use anchor_lang::prelude::*;


pub fn calculate_PDA(
    program_id: &Pubkey,
    hashed_name: Vec<u8>,
    name_class_opt: &Option<Pubkey>,
) -> (Pubkey, Vec<u8>) {
    //hashed name as the init seeds
    let mut seeds_vec: Vec<u8> = hashed_name;
    //root domain
    let name_class = name_class_opt.clone().unwrap_or_default();
    //add parent
    for b in name_class.to_bytes() {
        seeds_vec.push(b);
    }

    let (name_account_key, bump) =
        Pubkey::find_program_address(&seeds_vec.chunks(32).collect::<Vec<&[u8]>>(), program_id);
    seeds_vec.push(bump);

    (name_account_key, seeds_vec)
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Competition {
    pub creator: Pubkey,
    pub entry_fee: u64,
    pub prize_pool: u64,
    pub participants: Vec<Pubkey>,
}

pub fn create_competition(
    accounts: &[AccountInfo],
    entry_fee: u64,
) -> ProgramResult {
    let account = &accounts[0]; // Competition account
    let creator = &accounts[1]; // User creating the competition
    let mut competition_data = Competition {
        creator: *creator.key,
        entry_fee,
        prize_pool: 0,
        participants: Vec::new(),
    };

    // Handle competition creation logic
    competition_data.prize_pool += entry_fee;
    msg!("Competition created with entry fee: {}", entry_fee);

    Ok(())
}

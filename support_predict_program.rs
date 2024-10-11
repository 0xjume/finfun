#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SupportPredict {
    pub supporter: Pubkey,
    pub player: Pubkey,
    pub stake: u64,
}

pub fn support_player(
    accounts: &[AccountInfo],
    player: Pubkey,
    stake: u64,
) -> ProgramResult {
    let supporter = &accounts[0];
    let mut support_data = SupportPredict {
        supporter: *supporter.key,
        player,
        stake,
    };

    // Logic to register support and stake $FUN
    msg!(
        "{} supported {} with {} $FUN",
        supporter.key.to_string(),
        player.to_string(),
        stake
    );

    Ok(())
}

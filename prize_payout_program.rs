pub fn distribute_rewards(
    accounts: &[AccountInfo],
    winner: Pubkey,
    prize_pool: u64,
    platform_fee: u64,
) -> ProgramResult {
    let winner_account = &accounts[0];
    let platform_account = &accounts[1];

    let reward = prize_pool - platform_fee;
    msg!("Distributing rewards. Winner gets: {}", reward);

    // Transfer reward to the winner and fee to the platform
    invoke(
        &system_instruction::transfer(
            &accounts[0].key,
            &winner_account.key,
            reward,
        ),
        &[accounts[0].clone(), winner_account.clone()],
    )?;
    invoke(
        &system_instruction::transfer(
            &accounts[0].key,
            &platform_account.key,
            platform_fee,
        ),
        &[accounts[0].clone(), platform_account.clone()],
    )?;

    Ok(())
}

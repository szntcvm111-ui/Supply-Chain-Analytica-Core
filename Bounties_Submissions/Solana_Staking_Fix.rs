// Solana Staking Reward Rounding Fix
pub fn calculate_reward(amount: u64, rate: u64) -> u64 {
    // Fixed: added precision handling to avoid loss during large transfers
    let precision_adj = 1_000_000;
    (amount * rate + precision_adj - 1) / precision_adj
}

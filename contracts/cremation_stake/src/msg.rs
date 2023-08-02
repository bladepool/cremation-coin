use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub enum StakingPeriod {
    Short,
    Medium,
    Long,
}

#[cw_serde]
pub struct RewardInfoItem {
    pub period: StakingPeriod,
    pub staking_days: u64,
    pub reward_rate: Decimal,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub token_address: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    Receive { msg: Cw20ReceiveMsg },
    Unstake {},
}

#[cw_serde]
pub enum Cw20HookMsg {
    Stake { staking_period: StakingPeriod },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns the staking status of the given address.
    #[returns(StakedResponse)]
    Staked { address: Addr },
    /// Returns the reward info of staking contract.
    #[returns(RewardInfoResponse)]
    RewardInfo {},
    /// Check if staking is available.
    #[returns(CanStakeResponse)]
    CanStake {},
}

#[cw_serde]
pub struct StakedResponse {
    pub staked_amount: Uint128,
    pub pending_reward: Uint128,
    pub claim_reward_at: u64, // seconds
}

#[cw_serde]
pub struct RewardInfoResponse {
    pub reward_info: [RewardInfoItem; 3],
}

#[cw_serde]
pub struct CanStakeResponse {
    pub can_stake: bool,
}

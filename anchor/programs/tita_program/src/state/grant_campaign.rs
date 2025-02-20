use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GrantCampaign {
    pub total_funding: u64,
    pub remaining_funding: u64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
    pub grant_provider: Pubkey,
    #[max_len(32)]
    pub campaign_id: String,
}
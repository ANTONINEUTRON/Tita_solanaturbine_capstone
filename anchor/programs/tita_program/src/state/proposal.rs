use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub grant_campaign: Pubkey,
    pub applicant: Pubkey,
    pub status: ProposalStatus,
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
}
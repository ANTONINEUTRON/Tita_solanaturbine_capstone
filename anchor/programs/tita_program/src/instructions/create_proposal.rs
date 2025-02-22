use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub applicant: Signer<'info>,

    #[account(
        init,
        payer = applicant,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [
            grant_campaign.key().as_ref(),
            applicant.key().as_ref(),
        ],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        constraint = grant_campaign.is_active
    )]
    pub grant_campaign: Account<'info, GrantCampaign>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateProposal<'info> {
    pub fn create_proposal(
        &mut self,
        bump: u8
    ) -> Result<()> {
        let proposal = &mut self.proposal;
        let clock = Clock::get()?;

        proposal.grant_campaign = self.grant_campaign.key();
        proposal.applicant = self.applicant.key();
        proposal.status = ProposalStatus::Pending;
        proposal.created_at = clock.unix_timestamp;
        proposal.updated_at = clock.unix_timestamp;
        proposal.bump = bump;

        Ok(())
    }
}
use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = applicant,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [
            b"proposal",
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

    #[account(mut)]
    pub applicant: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateProposal<'info> {
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        bump: u8
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        proposal.grant_campaign = ctx.accounts.grant_campaign.key();
        proposal.applicant = ctx.accounts.applicant.key();
        proposal.status = ProposalStatus::Pending;
        proposal.created_at = clock.unix_timestamp;
        proposal.updated_at = clock.unix_timestamp;
        proposal.bump = bump;

        Ok(())
    }
}
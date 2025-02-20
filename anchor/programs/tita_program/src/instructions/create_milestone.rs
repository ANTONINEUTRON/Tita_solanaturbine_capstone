use crate::{state::*, TitaErrorCode};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateMilestone<'info> {
    #[account(
        init,
        payer = applicant,
        space = 8 + Milestone::INIT_SPACE,
        seeds = [
            b"milestone",
            proposal.key().as_ref(),
        ],
        bump
    )]
    pub milestone: Account<'info, Milestone>,

    #[account(
        mut,
        constraint = proposal.status == ProposalStatus::Approved,
        constraint = proposal.applicant == applicant.key()
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        mut,
        constraint = grant_campaign.key() == proposal.grant_campaign,
        constraint = grant_campaign.is_active
    )]
    pub grant_campaign: Account<'info, GrantCampaign>,

    #[account(mut)]
    pub applicant: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateMilestone<'info> {
    pub fn create_milestone(
        ctx: Context<CreateMilestone>, 
        amount: u64, 
        proof_uri: String,
        bumps: u8
    ) -> Result<()> {
        require!(
            amount <= ctx.accounts.grant_campaign.remaining_funding,
            TitaErrorCode::InsufficientFunds
        );

        let milestone = &mut ctx.accounts.milestone;
        let clock = Clock::get()?;

        milestone.proposal = ctx.accounts.proposal.key();
        milestone.amount = amount;
        milestone.created_at = clock.unix_timestamp;
        milestone.updated_at = clock.unix_timestamp;
        milestone.bump = bumps;
        milestone.proof_uri = proof_uri;
        milestone.status = MilestoneStatus::Pending;

        // Update remaining funding in campaign
        ctx.accounts.grant_campaign.remaining_funding = ctx
            .accounts
            .grant_campaign
            .remaining_funding
            .checked_sub(amount)
            .ok_or(TitaErrorCode::CalculationError)?;

        Ok(())
    }
}

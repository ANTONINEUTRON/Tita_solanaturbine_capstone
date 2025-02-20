use anchor_lang::prelude::*;
use crate::state::*;

use super::TitaErrorCode;

#[derive(Accounts)]
pub struct SetMilestoneStatus<'info> {
    #[account(
        mut,
        constraint = milestone.proposal == proposal.key()
    )]
    pub milestone: Account<'info, Milestone>,

    #[account(
        constraint = proposal.grant_campaign == grant_campaign.key()
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        constraint = grant_campaign.grant_provider == grant_provider.key()
    )]
    pub grant_campaign: Account<'info, GrantCampaign>,

    #[account(mut)]
    pub grant_provider: Signer<'info>,
}

impl<'info> SetMilestoneStatus<'info>{
    pub fn set_milestone_status(
        ctx: Context<SetMilestoneStatus>,
        status: MilestoneStatus
    ) -> Result<()> {
        let milestone = &mut ctx.accounts.milestone;
        let clock = Clock::get()?;

        milestone.status = status;
        milestone.updated_at = clock.unix_timestamp;

        Ok(())
    }
}
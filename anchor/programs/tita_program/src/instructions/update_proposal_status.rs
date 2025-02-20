use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateProposalStatus<'info> {
    #[account(
        mut,
        constraint = proposal.grant_campaign == grant_campaign.key(),
        constraint = proposal.status == ProposalStatus::Pending
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        constraint = grant_campaign.grant_provider == grant_provider.key()
    )]
    pub grant_campaign: Account<'info, GrantCampaign>,

    #[account(mut)]
    pub grant_provider: Signer<'info>,
}

impl<'info> UpdateProposalStatus<'info>{
    pub fn update_proposal_status(
        ctx: Context<UpdateProposalStatus>,
        status: ProposalStatus
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        proposal.status = status;
        proposal.updated_at = clock.unix_timestamp;

        Ok(())
    }
}
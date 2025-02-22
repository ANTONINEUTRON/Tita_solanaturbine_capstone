use crate::{state::*, TitaErrorCode};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseCampaign<'info> {
    #[account(
        mut,
        close = grant_provider,
        constraint = grant_campaign.grant_provider == grant_provider.key(),
        constraint = grant_campaign.is_active,
        constraint = grant_campaign.deadline.map_or(true, |deadline| Clock::get().unwrap().unix_timestamp >= deadline)
    )]
    pub grant_campaign: Account<'info, GrantCampaign>,

    #[account(mut)]
    pub grant_provider: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CloseCampaign<'info> {
    pub fn close_campaign(&mut self) -> Result<()> {
        let campaign = &mut self.grant_campaign;

        // Validate campaign can be closed
        require!(
            campaign.remaining_funding > 0,
            TitaErrorCode::NoRemainingFunds
        );

        // Deactivate campaign before closing
        campaign.is_active = false;
        campaign.updated_at = Clock::get()?.unix_timestamp;

        Ok(())
    }
}

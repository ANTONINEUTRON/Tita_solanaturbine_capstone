use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateCampaign<'info> {
    #[account(
        mut,
        constraint = grant_campaign.grant_provider == grant_provider.key()
    )]
    pub grant_campaign: Account<'info, GrantCampaign>,

    #[account(mut)]
    pub grant_provider: Signer<'info>,
}

impl<'info> UpdateCampaign<'info> {
    pub fn update_campaign(
        ctx: Context<UpdateCampaign>,
        total_funding: u64,
        is_active: bool,
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.grant_campaign;
        let clock = Clock::get()?;

        campaign.total_funding = total_funding;
        campaign.remaining_funding = total_funding;
        campaign.is_active = is_active;
        campaign.updated_at = clock.unix_timestamp;

        Ok(())
    }
}
